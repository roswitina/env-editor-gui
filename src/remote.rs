// =============================================================================
//  remote.rs — SSH/SFTP-Anbindung + sichere Speicherung von Zugangsdaten
// =============================================================================
//
// Nutzt `russh` (reine Rust-Implementierung des SSH-Protokolls) statt der
// C-Bibliothek libssh2. Hintergrund: libssh2 unterstützt bestimmte moderne,
// von OpenSSH inzwischen bevorzugte Verfahren (u.a. den AEAD-Cipher
// "chacha20-poly1305@openssh.com") schlicht nicht, wodurch Verbindungen zu
// Servern scheiterten, die ausschließlich solche Verfahren anbieten.
// `russh` implementiert das SSH-Protokoll selbst in Rust und unterstützt
// diese Verfahren nativ, wodurch keine Abhängigkeit von der jeweils
// installierten/vendored libssh2-Version mehr besteht.
//
// Zugangsdaten werden weiterhin bewusst NICHT als Klartext in einer
// Konfigurationsdatei abgelegt:
//   - Host/Port/Benutzername/Auth-Art (unkritisch) -> JSON-Datei im
//     Benutzer-Konfigurationsverzeichnis.
//   - Passwort bzw. Key-Passphrase (kritisch)      -> OS-Schlüsselbund
//     (Windows Credential Manager / macOS Keychain / Linux Secret Service)
//     über die `keyring`-Crate.
//
// Zusätzlich wird der Host-Key jedes Servers gemerkt (Trust-On-First-Use,
// wie es der normale SSH-Client mit ~/.ssh/known_hosts macht): Beim ersten
// Verbinden mit einem Host wird dessen Schlüssel-Fingerabdruck akzeptiert
// und gespeichert; ändert er sich später unerwartet, wird die Verbindung
// mit einer Warnung abgelehnt (möglicher Man-in-the-Middle-Angriff).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use russh::keys::*;
use russh::*;
use russh_sftp::client::SftpSession;
use russh_sftp::protocol::OpenFlags;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const KEYRING_SERVICE: &str = "oxicloud-env-editor";

/// Die drei Dateinamen, nach denen im Remote-Verzeichnis gesucht wird.
const ENV_CANDIDATES: &[&str] = &["example.env", ".env.example", ".env"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthKind {
    Password,
    Key,
}

/// Ein gespeichertes Verbindungsprofil. Enthält bewusst KEIN Passwort und
/// KEINE Passphrase — diese liegen (falls vorhanden) im OS-Schlüsselbund,
/// referenziert über `name`/`host`/`port`/`username`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedConnection {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub auth_kind: AuthKind,
    /// Nur bei Key-Auth relevant: Pfad zur privaten Schlüsseldatei.
    pub key_path: Option<PathBuf>,
    /// Ob zu diesem Profil ein Geheimnis (Passwort/Passphrase) im
    /// Schlüsselbund hinterlegt wurde.
    pub has_secret: bool,
}

#[derive(Default, Serialize, Deserialize)]
struct ConnectionStore {
    connections: Vec<SavedConnection>,
}

fn config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("oxicloud-env-editor"))
}

fn store_path() -> Option<PathBuf> {
    config_dir().map(|p| p.join("connections.json"))
}

/// Lädt die Liste der gespeicherten Verbindungsprofile (ohne Geheimnisse).
/// Gibt bei Fehlern/Nichtvorhandensein eine leere Liste zurück, damit der
/// App-Start nie an einer fehlenden/kaputten Konfigurationsdatei scheitert.
pub fn load_connections() -> Vec<SavedConnection> {
    let Some(path) = store_path() else {
        return Vec::new();
    };
    let Ok(data) = std::fs::read_to_string(&path) else {
        return Vec::new();
    };
    serde_json::from_str::<ConnectionStore>(&data)
        .map(|s| s.connections)
        .unwrap_or_default()
}

fn save_connections(list: &[SavedConnection]) -> Result<(), String> {
    let dir = config_dir().ok_or("Kein Konfigurationsverzeichnis gefunden.")?;
    std::fs::create_dir_all(&dir).map_err(|e| format!("Konfigurationsordner: {e}"))?;
    let path = dir.join("connections.json");
    let store = ConnectionStore {
        connections: list.to_vec(),
    };
    let json = serde_json::to_string_pretty(&store).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| format!("Schreiben der Profile: {e}"))
}

fn keyring_account(conn: &SavedConnection) -> String {
    format!("{}@{}:{}", conn.username, conn.host, conn.port)
}

/// Speichert (oder überschreibt) ein Verbindungsprofil. `secret`, sofern
/// gesetzt und nicht leer, wird NICHT in der JSON-Datei abgelegt, sondern im
/// OS-Schlüsselbund unter einem aus Benutzer/Host/Port abgeleiteten Konto.
pub fn save_connection(mut conn: SavedConnection, secret: Option<&str>) -> Result<(), String> {
    let mut list = load_connections();
    list.retain(|c| c.name != conn.name);

    if let Some(secret) = secret {
        if !secret.is_empty() {
            let entry = keyring::Entry::new(KEYRING_SERVICE, &keyring_account(&conn))
                .map_err(|e| format!("Schlüsselbund-Fehler: {e}"))?;
            entry
                .set_password(secret)
                .map_err(|e| format!("Schlüsselbund-Fehler: {e}"))?;
            conn.has_secret = true;
        }
    }

    list.push(conn);
    save_connections(&list)
}

/// Entfernt ein Profil aus der JSON-Datei sowie — falls vorhanden — dessen
/// Geheimnis aus dem Schlüsselbund.
pub fn delete_connection(conn: &SavedConnection) -> Result<(), String> {
    let mut list = load_connections();
    list.retain(|c| c.name != conn.name);
    save_connections(&list)?;

    if conn.has_secret {
        if let Ok(entry) = keyring::Entry::new(KEYRING_SERVICE, &keyring_account(conn)) {
            let _ = entry.delete_credential();
        }
    }
    Ok(())
}

/// Liest das gespeicherte Geheimnis (Passwort oder Key-Passphrase) für ein
/// Profil aus dem OS-Schlüsselbund, sofern vorhanden.
pub fn load_secret(conn: &SavedConnection) -> Option<String> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, &keyring_account(conn)).ok()?;
    entry.get_password().ok()
}

// ---------- Host-Key-Pinning (Trust-On-First-Use) ----------

fn known_hosts_path() -> Option<PathBuf> {
    config_dir().map(|p| p.join("known_hosts.json"))
}

fn load_known_hosts() -> HashMap<String, String> {
    let Some(path) = known_hosts_path() else {
        return HashMap::new();
    };
    let Ok(data) = std::fs::read_to_string(&path) else {
        return HashMap::new();
    };
    serde_json::from_str(&data).unwrap_or_default()
}

fn save_known_hosts(map: &HashMap<String, String>) -> Result<(), String> {
    let dir = config_dir().ok_or("Kein Konfigurationsverzeichnis gefunden.")?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("known_hosts.json");
    let json = serde_json::to_string_pretty(map).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())
}

/// Ergebnis der Host-Key-Prüfung während des Handshakes, damit der
/// aufrufende Code nach dem Verbindungsaufbau entscheiden kann, ob es sich
/// um einen neu akzeptierten oder einen abweichenden (potenziell
/// gefährlichen) Host-Key handelte.
#[derive(Clone)]
enum HostKeyOutcome {
    New(String),
    Mismatch { known: String, seen: String },
}

/// `russh::client::Handler`-Implementierung, die den Host-Key wie ein
/// klassisches `~/.ssh/known_hosts` behandelt: erster Kontakt wird
/// akzeptiert und gemerkt, jede spätere Abweichung wird abgelehnt.
struct HostKeyHandler {
    host_id: String,
    known_fingerprint: Option<String>,
    outcome: Arc<Mutex<Option<HostKeyOutcome>>>,
}

impl client::Handler for HostKeyHandler {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        server_public_key: &ssh_key::PublicKey,
    ) -> Result<bool, Self::Error> {
        let fingerprint = server_public_key.fingerprint(HashAlg::Sha256).to_string();

        match &self.known_fingerprint {
            Some(known) if *known == fingerprint => Ok(true),
            Some(known) => {
                *self.outcome.lock().unwrap() = Some(HostKeyOutcome::Mismatch {
                    known: known.clone(),
                    seen: fingerprint,
                });
                Ok(false)
            }
            None => {
                let mut hosts = load_known_hosts();
                hosts.insert(self.host_id.clone(), fingerprint.clone());
                let _ = save_known_hosts(&hosts);
                *self.outcome.lock().unwrap() = Some(HostKeyOutcome::New(fingerprint));
                Ok(true)
            }
        }
    }
}

/// Ein einzelner Eintrag beim Auflisten eines Remote-Verzeichnisses.
pub struct RemoteDirEntry {
    pub name: String,
    pub is_dir: bool,
}

/// Authentifizierungsmethode für einen einzelnen Verbindungsversuch.
pub enum Auth {
    Password(String),
    Key {
        path: PathBuf,
        passphrase: Option<String>,
    },
}

/// Parameter für einen einzelnen Verbindungsversuch (unabhängig davon, ob
/// er anschließend gespeichert wird).
pub struct ConnectParams {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub auth: Auth,
}

/// Eine aktive SSH-Sitzung. Jede Dateioperation öffnet intern kurz einen
/// eigenen SFTP-Kanal über dieselbe SSH-Verbindung — einfacher und robuster
/// als eine einzelne SFTP-Sitzung über beliebig lange Zeit offenzuhalten,
/// und für die kurzen .env-Dateien hier ohne spürbaren Mehraufwand.
pub struct RemoteSession {
    rt: tokio::runtime::Runtime,
    handle: client::Handle<HostKeyHandler>,
    /// Gesetzt, wenn beim Verbindungsaufbau ein bislang unbekannter
    /// Host-Key akzeptiert und neu gespeichert wurde (Trust-On-First-Use),
    /// damit die Oberfläche das dem Nutzer genauso transparent anzeigen
    /// kann wie ein normaler SSH-Client es tun würde.
    pub new_host_key_fingerprint: Option<String>,
}

impl RemoteSession {
    /// Baut die SSH-Verbindung auf, prüft/merkt den Host-Key (Trust-On-
    /// First-Use) und authentifiziert sich per Passwort oder privatem
    /// Schlüssel.
    pub fn connect(params: &ConnectParams) -> Result<Self, String> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| format!("Konnte Laufzeitumgebung nicht starten: {e}"))?;

        let host_id = format!("{}:{}", params.host, params.port);
        let known_fingerprint = load_known_hosts().get(&host_id).cloned();
        let outcome: Arc<Mutex<Option<HostKeyOutcome>>> = Arc::new(Mutex::new(None));

        let handler = HostKeyHandler {
            host_id,
            known_fingerprint,
            outcome: outcome.clone(),
        };

        let host = params.host.clone();
        let port = params.port;
        let username = params.username.clone();

        let handle: Result<client::Handle<HostKeyHandler>, String> = rt.block_on(async {
            let config = Arc::new(client::Config::default());
            let mut session = client::connect(config, (host.as_str(), port), handler)
                .await
                .map_err(|e| format!("Verbindung/Handshake fehlgeschlagen: {e}"))?;

            let auth_ok = match &params.auth {
                Auth::Password(pw) => session
                    .authenticate_password(&username, pw)
                    .await
                    .map_err(|e| format!("Authentifizierung fehlgeschlagen: {e}"))?
                    .success(),
                Auth::Key { path, passphrase } => {
                    let key = load_secret_key(path, passphrase.as_deref())
                        .map_err(|e| format!("Schlüssel konnte nicht gelesen werden: {e}"))?;
                    let hash_alg = session
                        .best_supported_rsa_hash()
                        .await
                        .map_err(|e| format!("Authentifizierung fehlgeschlagen: {e}"))?
                        .flatten();
                    session
                        .authenticate_publickey(
                            &username,
                            PrivateKeyWithHashAlg::new(Arc::new(key), hash_alg),
                        )
                        .await
                        .map_err(|e| format!("Authentifizierung fehlgeschlagen: {e}"))?
                        .success()
                }
            };

            if !auth_ok {
                return Err(
                    "Authentifizierung fehlgeschlagen (Passwort/Key/Benutzername prüfen)."
                        .to_string(),
                );
            }

            Ok(session)
        });

        // Eine Host-Key-Abweichung wiegt schwerer als ein Auth-Fehler und
        // wird zuerst geprüft, damit die Warnung nicht von einer generischen
        // Fehlermeldung verdeckt wird.
        let outcome_snapshot = outcome.lock().unwrap().clone();
        if let Some(HostKeyOutcome::Mismatch { known, seen }) = &outcome_snapshot {
            return Err(format!(
                "⚠️ Host-Key hat sich geändert — möglicher Man-in-the-Middle-Angriff! \
                 Bekannt: {known}, jetzt gemeldet: {seen}. Verbindung abgebrochen."
            ));
        }

        let new_host_key_fingerprint = match outcome_snapshot {
            Some(HostKeyOutcome::New(fp)) => Some(fp),
            _ => None,
        };

        let handle = handle?;
        Ok(Self {
            rt,
            handle,
            new_host_key_fingerprint,
        })
    }

    /// Öffnet einen neuen SFTP-Kanal über die bestehende SSH-Verbindung.
    async fn open_sftp(&self) -> Result<SftpSession, String> {
        let channel = self
            .handle
            .channel_open_session()
            .await
            .map_err(|e| format!("Kanal konnte nicht geöffnet werden: {e}"))?;
        channel
            .request_subsystem(true, "sftp")
            .await
            .map_err(|e| format!("SFTP-Subsystem fehlgeschlagen: {e}"))?;
        SftpSession::new(channel.into_stream())
            .await
            .map_err(|e| format!("SFTP-Sitzung fehlgeschlagen: {e}"))
    }

    /// Listet den Inhalt eines Remote-Verzeichnisses auf (Unterordner zuerst,
    /// dann Dateien, jeweils alphabetisch), damit sich der Nutzer zum Ziel
    /// durchklicken kann, statt den Pfad blind eintippen zu müssen.
    pub fn list_dir(&self, remote_dir: &str) -> Result<Vec<RemoteDirEntry>, String> {
        let dir = if remote_dir.is_empty() {
            "."
        } else {
            remote_dir
        };

        self.rt.block_on(async {
            let sftp = self.open_sftp().await?;
            let entries = sftp
                .read_dir(dir)
                .await
                .map_err(|e| format!("Verzeichnis konnte nicht gelesen werden: {e}"))?;

            let mut result = Vec::new();
            for entry in entries {
                let name = entry.file_name();
                if name == "." || name == ".." {
                    continue;
                }
                let is_dir = entry.metadata().is_dir();
                result.push(RemoteDirEntry { name, is_dir });
            }
            result.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then_with(|| a.name.cmp(&b.name)));
            Ok(result)
        })
    }

    /// Listet die vorhandenen Dateien aus {example.env, .env.example, .env}
    /// im angegebenen Remote-Verzeichnis auf (volle Pfade).
    pub fn find_env_files(&self, remote_dir: &str) -> Result<Vec<String>, String> {
        let dir = if remote_dir.is_empty() {
            "."
        } else {
            remote_dir
        };

        self.rt.block_on(async {
            let sftp = self.open_sftp().await?;
            let mut found = Vec::new();
            for name in ENV_CANDIDATES {
                let full = format!("{}/{}", dir.trim_end_matches('/'), name);
                if sftp.metadata(&full).await.is_ok() {
                    found.push(full);
                }
            }
            Ok(found)
        })
    }

    /// Liest den Inhalt einer Remote-Datei vollständig als String ein.
    pub fn read_file(&self, remote_path: &str) -> Result<String, String> {
        self.rt.block_on(async {
            let sftp = self.open_sftp().await?;
            let mut file = sftp
                .open_with_flags(remote_path, OpenFlags::READ)
                .await
                .map_err(|e| format!("Öffnen fehlgeschlagen: {e}"))?;
            let mut content = String::new();
            file.read_to_string(&mut content)
                .await
                .map_err(|e| format!("Lesen fehlgeschlagen: {e}"))?;
            Ok(content)
        })
    }

    /// Schreibt Inhalt in eine Remote-Datei. Existiert die Datei bereits,
    /// wird vorher ein "<datei>.bak" mit dem alten Inhalt angelegt — analog
    /// zum lokalen Speichern.
    pub fn write_file(&self, remote_path: &str, content: &str) -> Result<(), String> {
        self.rt.block_on(async {
            let sftp = self.open_sftp().await?;

            if sftp.metadata(remote_path).await.is_ok() {
                let mut old_file = sftp
                    .open_with_flags(remote_path, OpenFlags::READ)
                    .await
                    .map_err(|e| format!("Backup fehlgeschlagen: {e}"))?;
                let mut old_content = String::new();
                old_file
                    .read_to_string(&mut old_content)
                    .await
                    .map_err(|e| format!("Backup fehlgeschlagen: {e}"))?;

                let backup_path = format!("{remote_path}.bak");
                let mut backup_file = sftp
                    .open_with_flags(
                        &backup_path,
                        OpenFlags::CREATE | OpenFlags::TRUNCATE | OpenFlags::WRITE,
                    )
                    .await
                    .map_err(|e| format!("Backup fehlgeschlagen: {e}"))?;
                backup_file
                    .write_all(old_content.as_bytes())
                    .await
                    .map_err(|e| format!("Backup fehlgeschlagen: {e}"))?;
                let _ = backup_file.shutdown().await;
            }

            let mut file = sftp
                .open_with_flags(
                    remote_path,
                    OpenFlags::CREATE | OpenFlags::TRUNCATE | OpenFlags::WRITE,
                )
                .await
                .map_err(|e| format!("Schreiben fehlgeschlagen: {e}"))?;
            file.write_all(content.as_bytes())
                .await
                .map_err(|e| format!("Schreiben fehlgeschlagen: {e}"))?;
            file.shutdown()
                .await
                .map_err(|e| format!("Schreiben fehlgeschlagen: {e}"))?;
            Ok(())
        })
    }
}
