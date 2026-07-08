#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use rfd::FileDialog;
use rand::{rngs::OsRng, RngCore};
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};

mod remote;

// =============================================================================
//  Sprachen
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Language {
    German,
    English,
    French,
    Spanish,
    Italian,
}

impl Language {
    fn name(&self) -> &'static str {
        match self {
            Self::German => "Deutsch",
            Self::English => "English",
            Self::French => "Français",
            Self::Spanish => "Español",
            Self::Italian => "Italiano",
        }
    }

    fn translate(&self, key: &str) -> &'static str {
        match (self, key) {
            // ---------------- Deutsch ----------------
            (Self::German, "title") => "🔒 OxiCloud .env Konfigurations-Editor",
            (Self::German, "load_template") => "📁 1. Vorlage laden",
            (Self::German, "load_env") => "🔄 2. Bestehende .env dazuladen",
            (Self::German, "save") => "💾 Speichern unter...",
            (Self::German, "filter_hint") => "🔍 Filter nach Key oder Beschreibung...",
            (Self::German, "template_lbl") => "Vorlage",
            (Self::German, "target_lbl") => "Ziel",
            (Self::German, "none") => "Keine",
            (Self::German, "status_start") => "Bitte laden Sie eine Template-Datei (z.B. example.env / env.example)...",
            (Self::German, "status_template_err") => "Fehler: Laden Sie zuerst eine Template-Datei!",
            (Self::German, "status_template_ok") => "Template erfolgreich geladen!",
            (Self::German, "status_env_ok") => "Bestehende .env abgeglichen!",
            (Self::German, "grid_desc") => "📝 Beschreibung / Hilfetext",
            (Self::German, "grid_key") => "🔑 Schlüssel (Key)",
            (Self::German, "grid_val") => "✍️ Wert (Value)",
            (Self::German, "no_desc") => "Keine Beschreibung vorhanden.",
            (Self::German, "gen_secret") => "🔑 Secret generieren & kopieren",
            (Self::German, "secret_copied") => "Kryptografischer Schlüssel generiert und in die Zwischenablage kopiert!",
            (Self::German, "line_endings") => "Zeilenumbrüche für:",
            (Self::German, "secret_len_lbl") => "Länge:",
            (Self::German, "err_open") => "Fehler beim Öffnen: {}",
            (Self::German, "err_read") => "Fehler beim Lesen: {}",
            (Self::German, "err_create") => "Fehler beim Erstellen der Datei: {}",
            (Self::German, "err_write") => "Fehler beim Schreiben: {}",
            (Self::German, "err_backup") => "Backup fehlgeschlagen: {}",
            (Self::German, "saved_to") => "Gespeichert unter: {}",
            (Self::German, "count_vars") => "{} Variablen gefunden.",
            (Self::German, "count_merged") => "{} Werte übernommen.",
            (Self::German, "toggle_visibility") => "Sichtbarkeit umschalten",
            (Self::German, "secret_hover") => "Generiert ein sicheres Krypto-Secret und kopiert es automatisch.",
            (Self::German, "pick_template") => "Wähle eine Template-Datei (example.env / env.example)",
            (Self::German, "pick_env") => "Wähle eine existierende .env",
            (Self::German, "save_title") => ".env Speichern",

            // ---------------- English ----------------
            (Self::English, "title") => "🔒 OxiCloud .env Configuration Editor",
            (Self::English, "load_template") => "📁 1. Load Template",
            (Self::English, "load_env") => "🔄 2. Merge existing .env",
            (Self::English, "save") => "💾 Save as...",
            (Self::English, "filter_hint") => "🔍 Filter by Key or Description...",
            (Self::English, "template_lbl") => "Template",
            (Self::English, "target_lbl") => "Target",
            (Self::English, "none") => "None",
            (Self::English, "status_start") => "Please load a template file (e.g., example.env / env.example)...",
            (Self::English, "status_template_err") => "Error: Please load a template file first!",
            (Self::English, "status_template_ok") => "Template successfully loaded!",
            (Self::English, "status_env_ok") => "Existing .env merged!",
            (Self::English, "grid_desc") => "📝 Description / Help Text",
            (Self::English, "grid_key") => "🔑 Key",
            (Self::English, "grid_val") => "✍️ Value",
            (Self::English, "no_desc") => "No description available.",
            (Self::English, "gen_secret") => "🔑 Generate & Copy Secret",
            (Self::English, "secret_copied") => "Cryptographic key generated and copied to clipboard!",
            (Self::English, "line_endings") => "Line endings for:",
            (Self::English, "secret_len_lbl") => "Length:",
            (Self::English, "err_open") => "Error opening file: {}",
            (Self::English, "err_read") => "Error reading file: {}",
            (Self::English, "err_create") => "Error creating file: {}",
            (Self::English, "err_write") => "Error writing file: {}",
            (Self::English, "err_backup") => "Backup failed: {}",
            (Self::English, "saved_to") => "Saved to: {}",
            (Self::English, "count_vars") => "{} variables found.",
            (Self::English, "count_merged") => "{} values merged.",
            (Self::English, "toggle_visibility") => "Toggle visibility",
            (Self::English, "secret_hover") => "Generates a secure crypto secret and copies it automatically.",
            (Self::English, "pick_template") => "Select a template file (example.env / env.example)",
            (Self::English, "pick_env") => "Select an existing .env",
            (Self::English, "save_title") => "Save .env",

            // ---------------- Français ----------------
            (Self::French, "title") => "🔒 Éditeur de configuration .env OxiCloud",
            (Self::French, "load_template") => "📁 1. Charger le modèle",
            (Self::French, "load_env") => "🔄 2. Fusionner le .env existant",
            (Self::French, "save") => "💾 Enregistrer sous...",
            (Self::French, "filter_hint") => "🔍 Filtrer par clé ou description...",
            (Self::French, "template_lbl") => "Modèle",
            (Self::French, "target_lbl") => "Cible",
            (Self::French, "none") => "Aucun",
            (Self::French, "status_start") => "Veuillez charger un fichier modèle (ex. example.env / env.example)...",
            (Self::French, "status_template_err") => "Erreur : Veuillez d'abord charger un fichier modèle !",
            (Self::French, "status_template_ok") => "Modèle chargé avec succès !",
            (Self::French, "status_env_ok") => ".env existant fusionné !",
            (Self::French, "grid_desc") => "📝 Description / Aide",
            (Self::French, "grid_key") => "🔑 Clé",
            (Self::French, "grid_val") => "✍️ Valeur",
            (Self::French, "no_desc") => "Aucune description disponible.",
            (Self::French, "gen_secret") => "🔑 Générer & copier un secret",
            (Self::French, "secret_copied") => "Clé cryptographique générée et copiée dans le presse-papier !",
            (Self::French, "line_endings") => "Sauts de ligne pour :",
            (Self::French, "secret_len_lbl") => "Longueur :",
            (Self::French, "err_open") => "Erreur d'ouverture : {}",
            (Self::French, "err_read") => "Erreur de lecture : {}",
            (Self::French, "err_create") => "Erreur de création du fichier : {}",
            (Self::French, "err_write") => "Erreur d'écriture : {}",
            (Self::French, "err_backup") => "Échec de la sauvegarde : {}",
            (Self::French, "saved_to") => "Enregistré sous : {}",
            (Self::French, "count_vars") => "{} variables trouvées.",
            (Self::French, "count_merged") => "{} valeurs fusionnées.",
            (Self::French, "toggle_visibility") => "Basculer la visibilité",
            (Self::French, "secret_hover") => "Génère un secret cryptographique sûr et le copie automatiquement.",
            (Self::French, "pick_template") => "Sélectionner un fichier modèle (example.env / env.example)",
            (Self::French, "pick_env") => "Sélectionner un .env existant",
            (Self::French, "save_title") => "Enregistrer .env",

            // ---------------- Español ----------------
            (Self::Spanish, "title") => "🔒 Editor de configuración .env OxiCloud",
            (Self::Spanish, "load_template") => "📁 1. Cargar plantilla",
            (Self::Spanish, "load_env") => "🔄 2. Fusionar .env existente",
            (Self::Spanish, "save") => "💾 Guardar como...",
            (Self::Spanish, "filter_hint") => "🔍 Filtrar por clave o descripción...",
            (Self::Spanish, "template_lbl") => "Plantilla",
            (Self::Spanish, "target_lbl") => "Destino",
            (Self::Spanish, "none") => "Ninguno",
            (Self::Spanish, "status_start") => "Cargue un archivo de plantilla (p. ej., example.env / env.example)...",
            (Self::Spanish, "status_template_err") => "Error: ¡Cargue primero un archivo de plantilla!",
            (Self::Spanish, "status_template_ok") => "¡Plantilla cargada con éxito!",
            (Self::Spanish, "status_env_ok") => "¡.env existente fusionado!",
            (Self::Spanish, "grid_desc") => "📝 Descripción / Ayuda",
            (Self::Spanish, "grid_key") => "🔑 Clave",
            (Self::Spanish, "grid_val") => "✍️ Valor",
            (Self::Spanish, "no_desc") => "Sin descripción disponible.",
            (Self::Spanish, "gen_secret") => "🔑 Generar y copiar secreto",
            (Self::Spanish, "secret_copied") => "¡Clave criptográfica generada y copiada al portapapeles!",
            (Self::Spanish, "line_endings") => "Saltos de línea para:",
            (Self::Spanish, "secret_len_lbl") => "Longitud:",
            (Self::Spanish, "err_open") => "Error al abrir: {}",
            (Self::Spanish, "err_read") => "Error al leer: {}",
            (Self::Spanish, "err_create") => "Error al crear el archivo: {}",
            (Self::Spanish, "err_write") => "Error al escribir: {}",
            (Self::Spanish, "err_backup") => "Error en la copia de seguridad: {}",
            (Self::Spanish, "saved_to") => "Guardado en: {}",
            (Self::Spanish, "count_vars") => "{} variables encontradas.",
            (Self::Spanish, "count_merged") => "{} valores fusionados.",
            (Self::Spanish, "toggle_visibility") => "Alternar visibilidad",
            (Self::Spanish, "secret_hover") => "Genera un secreto criptográfico seguro y lo copia automáticamente.",
            (Self::Spanish, "pick_template") => "Seleccione un archivo de plantilla (example.env / env.example)",
            (Self::Spanish, "pick_env") => "Seleccione un .env existente",
            (Self::Spanish, "save_title") => "Guardar .env",

            // ---------------- Italiano ----------------
            (Self::Italian, "title") => "🔒 Editor di configurazione .env OxiCloud",
            (Self::Italian, "load_template") => "📁 1. Carica modello",
            (Self::Italian, "load_env") => "🔄 2. Unisci .env esistente",
            (Self::Italian, "save") => "💾 Salva come...",
            (Self::Italian, "filter_hint") => "🔍 Filtra per chiave o descrizione...",
            (Self::Italian, "template_lbl") => "Modello",
            (Self::Italian, "target_lbl") => "Destinazione",
            (Self::Italian, "none") => "Nessuno",
            (Self::Italian, "status_start") => "Carica un file modello (es. example.env / env.example)...",
            (Self::Italian, "status_template_err") => "Errore: caricare prima un file modello!",
            (Self::Italian, "status_template_ok") => "Modello caricato con successo!",
            (Self::Italian, "status_env_ok") => ".env esistente unito!",
            (Self::Italian, "grid_desc") => "📝 Descrizione / Aiuto",
            (Self::Italian, "grid_key") => "🔑 Chiave",
            (Self::Italian, "grid_val") => "✍️ Valore",
            (Self::Italian, "no_desc") => "Nessuna descrizione disponibile.",
            (Self::Italian, "gen_secret") => "🔑 Genera e copia segreto",
            (Self::Italian, "secret_copied") => "Chiave crittografica generata e copiata negli appunti!",
            (Self::Italian, "line_endings") => "Fine riga per:",
            (Self::Italian, "secret_len_lbl") => "Lunghezza:",
            (Self::Italian, "err_open") => "Errore di apertura: {}",
            (Self::Italian, "err_read") => "Errore di lettura: {}",
            (Self::Italian, "err_create") => "Errore di creazione del file: {}",
            (Self::Italian, "err_write") => "Errore di scrittura: {}",
            (Self::Italian, "err_backup") => "Backup non riuscito: {}",
            (Self::Italian, "saved_to") => "Salvato in: {}",
            (Self::Italian, "count_vars") => "{} variabili trovate.",
            (Self::Italian, "count_merged") => "{} valori uniti.",
            (Self::Italian, "toggle_visibility") => "Cambia visibilità",
            (Self::Italian, "secret_hover") => "Genera un segreto crittografico sicuro e lo copia automaticamente.",
            (Self::Italian, "pick_template") => "Seleziona un file modello (example.env / env.example)",
            (Self::Italian, "pick_env") => "Seleziona un .env esistente",
            (Self::Italian, "save_title") => "Salva .env",

            // ---------------- Default ----------------
            (_, _) => "?",
        }
    }
}

// =============================================================================
//  Domain-Enums
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TargetOsFormat {
    Linux,
    Windows,
}

impl TargetOsFormat {
    fn eol(&self) -> &'static str {
        match self {
            Self::Linux => "\n",
            Self::Windows => "\r\n",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Linux => "Linux / Docker (LF)",
            Self::Windows => "Windows (CRLF)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SecretLength {
    Bytes16,
    Bytes32,
}

impl SecretLength {
    fn byte_count(&self) -> usize {
        match self {
            Self::Bytes16 => 16,
            Self::Bytes32 => 32,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Bytes16 => "32 Chars (16B)",
            Self::Bytes32 => "64 Chars (32B)",
        }
    }
}

// =============================================================================
//  EnvItem: typsicher, ohne bool-Flag-Doppelbelegung
// =============================================================================

#[derive(Clone, Debug)]
enum EnvItem {
    Header {
        text: String,
    },
    Variable {
        key: String,
        value: String,
        /// Beschreibung als Liste von Absätzen (durch Leerzeilen im Quelltext
        /// getrennt). Der LETZTE Absatz ist typischerweise die konkrete,
        /// key-spezifische Erklärung; frühere Absätze sind meist allgemeiner
        /// Abschnittskontext. Die GUI stellt das optisch unterschiedlich dar.
        description: Vec<String>,
        is_commented_out: bool,
        show_password: bool,
    },
}

impl EnvItem {
    fn is_sensitive_key(key: &str) -> bool {
        let upper = key.to_ascii_uppercase();
        // Suffix-Matches: präziser als naives contains()
        const SUFFIXES: &[&str] = &[
            "_PASSWORD",
            "_PASS",
            "_SECRET",
            "_TOKEN",
            "_KEY",
            "_PRIVATE_KEY",
            "PASSWORD",
        ];
        const CONTAINS: &[&str] = &["JWT_", "API_KEY", "CLIENT_SECRET"];

        SUFFIXES.iter().any(|s| upper.ends_with(s))
            || CONTAINS.iter().any(|s| upper.contains(s))
    }
}

// =============================================================================
//  App
// =============================================================================

struct EnvEditorApp {
    items: Vec<EnvItem>,
    example_path: Option<PathBuf>,
    env_path: Option<PathBuf>,
    status_msg: String,
    search_query: String,
    current_lang: Language,
    target_os: TargetOsFormat,
    secret_len: SecretLength,

    // ---------- Remote (SSH/SFTP) ----------
    /// Steuert die Sichtbarkeit des "SSH-Verbindung"-Fensters.
    remote_window_open: bool,
    /// Aktive SSH/SFTP-Sitzung, sofern gerade verbunden.
    remote_session: Option<remote::RemoteSession>,
    /// Remote-Pfad der zuletzt geladenen/abgeglichenen .env-Datei. Ziel für
    /// "Auf Server speichern".
    remote_active_path: Option<String>,
    /// Gespeicherte Verbindungsprofile (ohne Geheimnisse, die liegen im
    /// OS-Schlüsselbund).
    remote_saved_connections: Vec<remote::SavedConnection>,
    /// Eingabefelder des Verbindungsdialogs.
    remote_form: RemoteForm,
    /// Im Remote-Verzeichnis gefundene Dateien aus
    /// {example.env, .env.example, .env}.
    remote_found_files: Vec<String>,
    /// Inhalt des aktuell angezeigten Remote-Verzeichnisses (zum
    /// Durchklicken, statt den Pfad blind eintippen zu müssen).
    remote_dir_entries: Vec<remote::RemoteDirEntry>,
    /// Status-/Fehlermeldungen speziell für den Remote-Dialog.
    remote_status: String,
}

/// Eingabefelder des "SSH-Verbindung"-Dialogs. Getrennt von den gespeicherten
/// Profilen (`remote::SavedConnection`), damit man ein Profil laden, die
/// Felder anpassen und trotzdem das Original unverändert lassen kann.
struct RemoteForm {
    profile_name: String,
    host: String,
    port: String,
    username: String,
    auth_kind: remote::AuthKind,
    password: String,
    show_password: bool,
    key_path: Option<PathBuf>,
    key_passphrase: String,
    remote_dir: String,
    /// Wenn aktiv, werden Profil + Zugangsdaten (im OS-Schlüsselbund) nach
    /// erfolgreichem Verbindungsaufbau gespeichert.
    remember: bool,
}

impl Default for RemoteForm {
    fn default() -> Self {
        Self {
            profile_name: String::new(),
            host: String::new(),
            port: "22".to_string(),
            username: String::new(),
            auth_kind: remote::AuthKind::Password,
            password: String::new(),
            show_password: false,
            key_path: None,
            key_passphrase: String::new(),
            remote_dir: ".".to_string(),
            remember: false,
        }
    }
}

impl Default for EnvEditorApp {
    fn default() -> Self {
        let default_lang = Language::German;
        Self {
            items: Vec::new(),
            example_path: None,
            env_path: None,
            status_msg: default_lang.translate("status_start").to_string(),
            search_query: String::new(),
            current_lang: default_lang,
            target_os: TargetOsFormat::Linux,
            secret_len: SecretLength::Bytes32,

            remote_window_open: false,
            remote_session: None,
            remote_active_path: None,
            remote_saved_connections: remote::load_connections(),
            remote_form: RemoteForm::default(),
            remote_found_files: Vec::new(),
            remote_dir_entries: Vec::new(),
            remote_status: String::new(),
        }
    }
}

impl EnvEditorApp {
    // ---------- Parsing ----------

    /// Erkennt kurze Legenden-/Listen-Einträge wie
    /// "587 = STARTTLS submission (default)", "starttls = port 587 ..." oder
    /// "1. a binary built with ...". Erkennungsmerkmal: ein kurzes Token
    /// (Zahl oder einzelnes Wort, keine Leerzeichen) direkt gefolgt von '='
    /// oder '.', typisch für Wertelisten/Legenden — im Gegensatz zu normalem
    /// Fließtext, bei dem vor einem '=' meist ein ganzer, mehrwortiger Satz
    /// steht (z.B. "Lower = fresher quota" mitten im Satz).
    fn looks_like_legend_entry(body: &str) -> bool {
        if let Some(eq_pos) = body.find('=') {
            let before = body[..eq_pos].trim();
            if !before.is_empty() && before.len() <= 20 && !before.contains(char::is_whitespace) {
                return true;
            }
        }
        // Nummerierte Liste: "1. ...", "2. ..."
        let mut rest = body;
        let mut has_digit = false;
        while let Some(c) = rest.chars().next() {
            if c.is_ascii_digit() {
                has_digit = true;
                rest = &rest[c.len_utf8()..];
            } else {
                break;
            }
        }
        has_digit && rest.trim_start().starts_with('.')
    }

    /// Prüft, ob eine Zeile eine REIN dekorative Trennlinie ist, z.B.
    /// "# -----------------------------" oder "# ═══════════════" oder
    /// "# ──────────────────────────────". Nach Entfernen von '#', '=', '-',
    /// '─', '━' und Whitespace bleibt bei einer reinen Trennlinie nichts übrig.
    fn is_pure_separator(trimmed: &str) -> bool {
        if !trimmed.starts_with('#') {
            return false;
        }
        let rest = &trimmed[1..];
        let has_decoration = rest.chars().any(|c| matches!(c, '=' | '-' | '─' | '━'));
        let only_decoration = rest
            .chars()
            .all(|c| matches!(c, '=' | '-' | '─' | '━') || c.is_whitespace());
        has_decoration && only_decoration
    }

    /// Prüft, ob eine Zeile ein Ein-Zeilen-Header im Stil
    /// "# --- Titel ---", "# ── Titel ──" oder "# ===== Titel =====" ist:
    /// ein Rahmen aus Trennzeichen, der noch echten Titeltext umschließt.
    fn is_inline_header(trimmed: &str) -> bool {
        if !trimmed.starts_with('#') || Self::is_pure_separator(trimmed) {
            return false;
        }
        let has_frame = trimmed.contains("===")
            || trimmed.contains("---")
            || trimmed.contains("──")
            || trimmed.contains("━━");
        if !has_frame {
            return false;
        }
        let inner = Self::strip_header_decoration(trimmed);
        !inner.is_empty()
    }

    /// Entfernt Rahmenzeichen ('#', '=', '-', '─', '━', Whitespace) von beiden
    /// Seiten, um aus einer Header-Zeile den reinen Titeltext zu extrahieren.
    fn strip_header_decoration(trimmed: &str) -> String {
        trimmed
            .trim_matches(|c: char| matches!(c, '#' | '=' | '-' | '─' | '━') || c.is_whitespace())
            .to_string()
    }

    /// Parst eine .env-Datei und liefert die strukturierten Items zurück.
    ///
    /// Section-Header werden generisch anhand ihrer Form erkannt (Trennlinie /
    /// Titel / Trennlinie im 3-Zeilen-Block, oder ein gerahmter Titel in einer
    /// einzelnen Zeile) — unabhängig vom konkreten Titeltext. Das macht das
    /// Erkennen robust gegenüber neuen/unbekannten Abschnittsnamen und
    /// verhindert, dass Unter-Header (z.B. "# --- Azure Blob Storage ---")
    /// stillschweigend verschluckt werden und ihr nachfolgender Hilfetext
    /// fälschlich der ersten Variable des Abschnitts zugeschlagen wird.
    fn parse(&self, path: &Path) -> Result<Vec<EnvItem>, String> {
        let mut file = File::open(path)
            .map_err(|e| self.current_lang.translate("err_open").replace("{}", &e.to_string()))?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| self.current_lang.translate("err_read").replace("{}", &e.to_string()))?;

        self.parse_str(&content)
    }

    /// Kern der .env-Parsing-Logik, unabhängig von der Quelle (lokale Datei
    /// oder per SFTP von einem Remote-Server gelesener Inhalt).
    fn parse_str(&self, content: &str) -> Result<Vec<EnvItem>, String> {
        let lines: Vec<&str> = content.lines().collect();
        let mut items: Vec<EnvItem> = Vec::new();
        // Jeder Eintrag ist ein zusammenhängender Absatz. Zeilen ohne
        // Leerzeile dazwischen werden mit Leerzeichen zu einem fließenden
        // Absatz verbunden statt hart umgebrochen (bessere Lesbarkeit).
        let mut paragraphs: Vec<String> = Vec::new();
        let mut paragraph_break = true;

        let mut i = 0;
        while i < lines.len() {
            let trimmed = lines[i].trim();

            // Leerzeile: nächste Kommentarzeile beginnt einen neuen Absatz
            if trimmed.is_empty() {
                paragraph_break = true;
                i += 1;
                continue;
            }

            // 3-Zeilen-Header-Block: Trennlinie / Titel / Trennlinie
            if Self::is_pure_separator(trimmed) {
                if i + 2 < lines.len() {
                    let mid = lines[i + 1].trim();
                    let after = lines[i + 2].trim();
                    if !Self::is_pure_separator(mid)
                        && mid.starts_with('#')
                        && Self::is_pure_separator(after)
                    {
                        let title = Self::strip_header_decoration(mid);
                        if !title.is_empty() {
                            items.push(EnvItem::Header { text: title });
                        }
                        paragraphs.clear();
                        paragraph_break = true;
                        i += 3;
                        continue;
                    }
                }
                // Trennlinie ohne erkennbaren Titel dahinter: reset als Grenze,
                // damit nichts über sie hinweg "durchsickert".
                paragraphs.clear();
                paragraph_break = true;
                i += 1;
                continue;
            }

            // Ein-Zeilen-Header: "# --- Titel ---" / "# ── Titel ──" / "# ===== Titel ====="
            if Self::is_inline_header(trimmed) {
                let title = Self::strip_header_decoration(trimmed);
                if !title.is_empty() {
                    items.push(EnvItem::Header { text: title });
                }
                paragraphs.clear();
                paragraph_break = true;
                i += 1;
                continue;
            }

            // Variablendeklaration
            if let Some((raw_key, raw_val)) = Self::extract_assignment(trimmed) {
                let key = raw_key.trim().to_string();
                let val = raw_val.trim().to_string();
                let description = paragraphs.clone();
                let is_sensitive = EnvItem::is_sensitive_key(&key);

                items.push(EnvItem::Variable {
                    key,
                    value: val,
                    description,
                    is_commented_out: Self::is_commented_out_flag(trimmed),
                    show_password: !is_sensitive,
                });
                paragraphs.clear();
                paragraph_break = true;
                i += 1;
                continue;
            }

            // Reine Kommentarzeile → Hilfetext
            if trimmed.starts_with('#') {
                let body = trimmed[1..].trim_start();
                // Copy-Hinweise filtern
                if body.contains("Copy this file") || body.contains("cp example.env") {
                    i += 1;
                    continue;
                }
                if body.is_empty() {
                    // Leerer Kommentar ("#") wirkt wie eine Leerzeile: Absatzgrenze
                    paragraph_break = true;
                } else {
                    let is_legend_entry = Self::looks_like_legend_entry(body);

                    // Beginnt die Zeile mit einem Kleinbuchstaben, ist sie mit hoher
                    // Wahrscheinlichkeit die reine Wortumbruch-Fortsetzung des vorigen
                    // Satzes (z.B. "...die Systemauslastung" / "regelmäßig neu.") und
                    // wird angehängt. Großbuchstabe/Ziffer/Sonderzeichen am Anfang
                    // deutet auf einen neuen Satz oder Aufzählungspunkt hin und
                    // beginnt einen neuen Absatz — so verschmelzen Aufzählungen
                    // (z.B. nummerierte Listen ohne Leerzeilen) nicht zu Fließtext.
                    let is_wrap_continuation = !is_legend_entry
                        && !paragraph_break
                        && !paragraphs.is_empty()
                        && body.chars().next().map_or(false, |c| c.is_lowercase());

                    if is_legend_entry && !paragraphs.is_empty() {
                        // Legenden-/Listeneintrag (z.B. "587 = STARTTLS submission",
                        // "starttls = port 587 ..."): als EIGENE Zeile im selben
                        // Absatz anhängen, statt als neuen Absatz abzuspalten
                        // (falsch als "je eigener Absatz" gewertet) oder mit
                        // Leerzeichen in einen Fließtext-Satz zu verschmelzen
                        // (falsch als reiner Wortumbruch gewertet).
                        if let Some(last) = paragraphs.last_mut() {
                            last.push('\n');
                            last.push_str(body);
                        }
                    } else if is_wrap_continuation {
                        if let Some(last) = paragraphs.last_mut() {
                            last.push(' ');
                            last.push_str(body);
                        }
                    } else {
                        paragraphs.push(body.to_string());
                    }
                    paragraph_break = false;
                }
            }

            i += 1;
        }

        Ok(items)
    }

    /// Liefert (key, value), falls die Zeile eine Variablenzuweisung ist.
    /// Behandelt sowohl aktive als auch auskommentierte Zuweisungen.
    fn extract_assignment(trimmed: &str) -> Option<(&str, &str)> {
        let line_for_key = if let Some(rest) = trimmed.strip_prefix('#') {
            // WICHTIG: Eine ECHTE auskommentierte Variable hat in dieser Datei
            // NIE ein Leerzeichen zwischen '#' und dem Key (z.B.
            // "#OXICLOUD_X=value"). Steht direkt nach '#' ein Leerzeichen
            // (z.B. "#   starttls = port 587 ..."), handelt es sich um einen
            // Hilfetext-/Legenden-Eintrag — auch wenn der Text zufällig wie
            // "kurzesWort = Erklärung" aussieht und sonst die Key-Validierung
            // bestehen würde (wie "starttls", "tls", "none").
            if rest.starts_with(char::is_whitespace) {
                return None;
            }
            rest
        } else {
            trimmed
        };

        // Nur als Variable werten, wenn der erste char ein Buchstabe oder _ ist
        let first_char = line_for_key.chars().next()?;
        if !(first_char.is_ascii_alphabetic() || first_char == '_') {
            return None;
        }
        if !line_for_key.contains('=') {
            return None;
        }

        let (k, v) = line_for_key.split_once('=')?;
        let k = k.trim();

        // FIX (Bug 4): Ein gültiger Env-Var-Key besteht ausschließlich aus
        // Buchstaben, Ziffern und Unterstrichen. Damit werden normale
        // Beschreibungssätze wie "# Beispiel: REDIS_URL=redis://..." nicht
        // mehr fälschlich als (auskommentierte) Variable erkannt.
        if k.is_empty() || !k.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return None;
        }

        Some((k, v.trim()))
    }

    fn is_commented_out_flag(trimmed: &str) -> bool {
        trimmed.starts_with('#')
    }

    fn load_template(&mut self, path: PathBuf) {
        match self.parse(&path) {
            Ok(items) => {
                let count = items
                    .iter()
                    .filter(|i| matches!(i, EnvItem::Variable { .. }))
                    .count();
                self.items = items;
                self.example_path = Some(path);
                self.status_msg = format!(
                    "{} {}",
                    self.current_lang.translate("status_template_ok"),
                    self.current_lang.translate("count_vars").replace("{}", &count.to_string())
                );
            }
            Err(msg) => {
                self.status_msg = msg;
            }
        }
    }

    fn merge_env(&mut self, path: PathBuf) {
        let new_items = match self.parse(&path) {
            Ok(v) => v,
            Err(msg) => {
                self.status_msg = msg;
                return;
            }
        };

        let match_count = self.merge_items(&new_items);
        self.env_path = Some(path);
        self.status_msg = format!(
            "{} {}",
            self.current_lang.translate("status_env_ok"),
            self.current_lang
                .translate("count_merged")
                .replace("{}", &match_count.to_string())
        );
    }

    /// Übernimmt Werte/Kommentar-Status aus `new_items` in `self.items`
    /// anhand des Keys. Gibt die Anzahl übernommener Werte zurück. Von
    /// `merge_env` (lokale Datei) und `remote_merge` (SFTP) gemeinsam
    /// genutzt.
    fn merge_items(&mut self, new_items: &[EnvItem]) -> usize {
        let mut match_count: usize = 0;
        for existing in self.items.iter_mut() {
            if let EnvItem::Variable {
                key,
                value,
                is_commented_out,
                ..
            } = existing
            {
                if let Some(EnvItem::Variable {
                    value: nv,
                    is_commented_out: nc,
                    ..
                }) = new_items.iter().find(|i| {
                    matches!(i, EnvItem::Variable { key: k, .. } if k == key)
                })
                {
                    *value = nv.clone();
                    *is_commented_out = *nc;
                    match_count += 1;
                }
            }
        }
        match_count
    }

    // ---------- Backup & Save ----------

    fn backup_existing(&self, path: &Path) -> Result<PathBuf, String> {
        let mut backup_path = path.to_path_buf();
        if let Some(ext) = backup_path.extension() {
            let mut new_ext = ext.to_os_string();
            new_ext.push(".bak");
            backup_path.set_extension(new_ext);
        } else {
            backup_path.set_extension("bak");
        }

        let mut old_f = File::open(path).map_err(|e| {
            self.current_lang
                .translate("err_open")
                .replace("{}", &e.to_string())
        })?;
        let mut bak_f = File::create(&backup_path).map_err(|e| {
            self.current_lang
                .translate("err_create")
                .replace("{}", &e.to_string())
        })?;

        let mut buffer = Vec::new();
        old_f
            .read_to_end(&mut buffer)
            .map_err(|e| self.current_lang.translate("err_read").replace("{}", &e.to_string()))?;
        bak_f
            .write_all(&buffer)
            .map_err(|e| self.current_lang.translate("err_write").replace("{}", &e.to_string()))?;

        Ok(backup_path)
    }

    /// Serialisiert die aktuellen Items als .env-Text (mit den gewählten
    /// Zeilenumbrüchen). Gemeinsam genutzt vom lokalen Speichern
    /// (`save_env_file`) und vom Speichern auf einen Remote-Server
    /// (`remote_save_active`).
    fn serialize_env(&self) -> String {
        use std::fmt::Write as _;
        let eol = self.target_os.eol();
        let mut out = String::new();

        for item in &self.items {
            match item {
                EnvItem::Header { text } => {
                    // Header werden im Ein-Zeilen-Format geschrieben, das der
                    // Parser (is_inline_header) generisch erkennt:
                    // "# ===== TITEL =====". Dadurch bleibt ein Speichern/Laden-
                    // Roundtrip konsistent, statt den Titel beim erneuten Einlesen
                    // als Beschreibungstext misszuinterpretieren.
                    let _ = write!(out, "# {:=^69}", format!(" {} ", text));
                    out.push_str(eol);
                }
                EnvItem::Variable {
                    key,
                    value,
                    description,
                    is_commented_out,
                    ..
                } => {
                    if !description.is_empty() {
                        for (idx, paragraph) in description.iter().enumerate() {
                            if idx > 0 {
                                // Leerzeile zwischen Absätzen, damit der Parser
                                // die Absatzgrenzen beim erneuten Laden wieder
                                // exakt so erkennt (Roundtrip-Konsistenz).
                                out.push('#');
                                out.push_str(eol);
                            }
                            // Ein Absatz kann interne Zeilenumbrüche enthalten
                            // (z.B. bei Legenden-/Listen-Einträgen) — jede davon
                            // braucht ihr eigenes "# "-Präfix.
                            for line in paragraph.lines() {
                                let _ = write!(out, "# {}", line);
                                out.push_str(eol);
                            }
                        }
                    }
                    if *is_commented_out {
                        let _ = write!(out, "#{}={}", key, value);
                    } else {
                        let _ = write!(out, "{}={}", key, value);
                    }
                    out.push_str(eol);
                    out.push_str(eol);
                }
            }
        }

        out
    }

    fn save_env_file(&mut self, path: &PathBuf) {
        // Backup nur, wenn die Datei existiert
        if path.exists() {
            match self.backup_existing(path) {
                Ok(_) => {}
                Err(e) => {
                    self.status_msg = e;
                    return;
                }
            }
        }

        let file = match File::create(path) {
            Ok(f) => f,
            Err(e) => {
                self.status_msg = self
                    .current_lang
                    .translate("err_create")
                    .replace("{}", &e.to_string());
                return;
            }
        };
        let mut writer = BufWriter::new(file);
        let content = self.serialize_env();

        if let Err(e) = writer.write_all(content.as_bytes()) {
            self.status_msg = self
                .current_lang
                .translate("err_write")
                .replace("{}", &e.to_string());
            return;
        }

        self.status_msg = self
            .current_lang
            .translate("saved_to")
            .replace("{}", &path.to_string_lossy());
    }

    // ---------- Remote (SSH/SFTP) ----------

    /// Beendet die aktuelle SSH/SFTP-Sitzung und setzt den zugehörigen
    /// Remote-Zustand zurück. Bereits geladene Items in der Tabelle bleiben
    /// unangetastet — nur die Verbindung und ihr "Speicherziel" fallen weg.
    fn remote_disconnect(&mut self) {
        self.remote_session = None;
        self.remote_active_path = None;
        self.remote_found_files.clear();
        self.remote_dir_entries.clear();
        self.remote_status = "Verbindung getrennt.".to_string();
    }

    /// Baut anhand der Formularfelder eine SSH/SFTP-Verbindung auf, listet
    /// anschließend {example.env, .env.example, .env} im gewählten
    /// Remote-Verzeichnis auf und speichert das Profil optional
    /// (Zugangsdaten im OS-Schlüsselbund, keine Klartext-Speicherung).
    fn remote_connect(&mut self) {
        let port: u16 = match self.remote_form.port.trim().parse() {
            Ok(p) => p,
            Err(_) => {
                self.remote_status = "Ungültiger Port.".to_string();
                return;
            }
        };

        if self.remote_form.host.trim().is_empty() || self.remote_form.username.trim().is_empty()
        {
            self.remote_status = "Bitte Host und Benutzername angeben.".to_string();
            return;
        }

        let auth = match self.remote_form.auth_kind {
            remote::AuthKind::Password => {
                remote::Auth::Password(self.remote_form.password.clone())
            }
            remote::AuthKind::Key => {
                let Some(path) = self.remote_form.key_path.clone() else {
                    self.remote_status = "Bitte eine Schlüsseldatei wählen.".to_string();
                    return;
                };
                let passphrase = if self.remote_form.key_passphrase.is_empty() {
                    None
                } else {
                    Some(self.remote_form.key_passphrase.clone())
                };
                remote::Auth::Key { path, passphrase }
            }
        };

        let params = remote::ConnectParams {
            host: self.remote_form.host.trim().to_string(),
            port,
            username: self.remote_form.username.trim().to_string(),
            auth,
        };

        match remote::RemoteSession::connect(&params) {
            Ok(session) => {
                let host_key_note = session.new_host_key_fingerprint.clone();
                self.remote_session = Some(session);
                self.remote_refresh(self.remote_form.remote_dir.trim().to_string().as_str());

                if let Some(fp) = host_key_note {
                    self.remote_status =
                        format!("Neuer Host-Key akzeptiert und gespeichert ({fp}). {}", self.remote_status);
                }

                if self.remote_form.remember {
                    let name = if self.remote_form.profile_name.trim().is_empty() {
                        format!("{}@{}", self.remote_form.username, self.remote_form.host)
                    } else {
                        self.remote_form.profile_name.trim().to_string()
                    };
                    let conn = remote::SavedConnection {
                        name,
                        host: self.remote_form.host.trim().to_string(),
                        port,
                        username: self.remote_form.username.trim().to_string(),
                        auth_kind: self.remote_form.auth_kind,
                        key_path: self.remote_form.key_path.clone(),
                        has_secret: false,
                    };
                    let secret = match self.remote_form.auth_kind {
                        remote::AuthKind::Password => Some(self.remote_form.password.as_str()),
                        remote::AuthKind::Key if !self.remote_form.key_passphrase.is_empty() => {
                            Some(self.remote_form.key_passphrase.as_str())
                        }
                        remote::AuthKind::Key => None,
                    };
                    match remote::save_connection(conn, secret) {
                        Ok(_) => self.remote_saved_connections = remote::load_connections(),
                        Err(e) => {
                            self.remote_status =
                                format!("Verbunden, aber Profil-Speichern fehlgeschlagen: {e}")
                        }
                    }
                }
            }
            Err(e) => {
                self.remote_session = None;
                self.remote_found_files.clear();
                self.remote_dir_entries.clear();
                self.remote_status = e;
            }
        }
    }

    /// Listet ein Remote-Verzeichnis neu (Unterordner zum Durchklicken +
    /// Suche nach {example.env, .env.example, .env}), OHNE die Verbindung
    /// neu aufzubauen. Wird beim Navigieren per Klick und beim manuellen
    /// erneuten Suchen nach Pfad-Änderung verwendet.
    fn remote_refresh(&mut self, dir: &str) {
        if self.remote_session.is_none() {
            self.remote_status = "Keine aktive Verbindung.".to_string();
            return;
        }
        self.remote_form.remote_dir = dir.to_string();

        match self.remote_session.as_ref().unwrap().list_dir(dir) {
            Ok(entries) => self.remote_dir_entries = entries,
            Err(e) => {
                self.remote_dir_entries.clear();
                self.remote_status = format!("Verzeichnis konnte nicht gelistet werden: {e}");
                return;
            }
        }

        match self.remote_session.as_ref().unwrap().find_env_files(dir) {
            Ok(files) => {
                self.remote_found_files = files;
                self.remote_status = format!(
                    "{} .env-Datei(en) gefunden, {} Einträge in „{}“.",
                    self.remote_found_files.len(),
                    self.remote_dir_entries.len(),
                    dir
                );
            }
            Err(e) => {
                self.remote_found_files.clear();
                self.remote_status = format!("Suche fehlgeschlagen: {e}");
            }
        }
    }

    /// Navigiert in der Ordner-Ansicht: `name == ".."` geht eine Ebene
    /// hoch, jeder andere Name wird als Unterordner angehängt.
    fn remote_navigate(&mut self, name: &str) {
        let current = self.remote_form.remote_dir.trim().to_string();
        let next = if name == ".." {
            match current.rsplit_once('/') {
                Some((parent, _)) if !parent.is_empty() => parent.to_string(),
                _ => "/".to_string(),
            }
        } else if current.is_empty() || current == "." {
            name.to_string()
        } else {
            format!("{}/{}", current.trim_end_matches('/'), name)
        };
        self.remote_refresh(&next);
    }

    /// Übernimmt Host/Port/Benutzer/Auth-Art eines gespeicherten Profils ins
    /// Formular und lädt — falls vorhanden — das zugehörige Geheimnis aus
    /// dem OS-Schlüsselbund.
    fn remote_apply_saved(&mut self, idx: usize) {
        if let Some(conn) = self.remote_saved_connections.get(idx).cloned() {
            self.remote_form.profile_name = conn.name.clone();
            self.remote_form.host = conn.host.clone();
            self.remote_form.port = conn.port.to_string();
            self.remote_form.username = conn.username.clone();
            self.remote_form.auth_kind = conn.auth_kind;
            self.remote_form.key_path = conn.key_path.clone();
            self.remote_form.remember = true;
            self.remote_form.password.clear();
            self.remote_form.key_passphrase.clear();

            if conn.has_secret {
                if let Some(secret) = remote::load_secret(&conn) {
                    match conn.auth_kind {
                        remote::AuthKind::Password => self.remote_form.password = secret,
                        remote::AuthKind::Key => self.remote_form.key_passphrase = secret,
                    }
                }
            }
        }
    }

    /// Löscht ein gespeichertes Profil samt Geheimnis im Schlüsselbund.
    fn remote_delete_saved(&mut self, idx: usize) {
        if idx < self.remote_saved_connections.len() {
            let conn = self.remote_saved_connections.remove(idx);
            if let Err(e) = remote::delete_connection(&conn) {
                self.remote_status = e;
            }
        }
    }

    /// Lädt eine Remote-Datei als neue Vorlage (analog zu `load_template`,
    /// nur dass der Inhalt per SFTP statt lokal gelesen wird).
    fn remote_load_as_template(&mut self, remote_path: String) {
        let Some(session) = &self.remote_session else {
            self.remote_status = "Keine aktive Verbindung.".to_string();
            return;
        };
        let content = match session.read_file(&remote_path) {
            Ok(c) => c,
            Err(e) => {
                self.remote_status = e;
                return;
            }
        };
        match self.parse_str(&content) {
            Ok(items) => {
                let count = items
                    .iter()
                    .filter(|i| matches!(i, EnvItem::Variable { .. }))
                    .count();
                self.items = items;
                self.example_path = None;
                self.env_path = None;
                self.remote_active_path = Some(remote_path.clone());
                self.status_msg = format!(
                    "{} {}",
                    self.current_lang.translate("status_template_ok"),
                    self.current_lang
                        .translate("count_vars")
                        .replace("{}", &count.to_string())
                );
                self.remote_status = format!("Als Vorlage geladen: {remote_path}");
                // Nach erfolgreichem Laden macht das Verbindungsfenster den
                // Blick auf die Tabelle nur noch frei — schließen, statt den
                // Nutzer es manuell verkleinern/verschieben zu lassen. Über
                // den Button lässt es sich jederzeit wieder öffnen, die
                // Verbindung bleibt dabei aktiv.
                self.remote_window_open = false;
            }
            Err(msg) => self.status_msg = msg,
        }
    }

    /// Gleicht eine Remote-Datei mit den bereits geladenen Items ab (analog
    /// zu `merge_env`) und merkt sich den Pfad als Ziel für
    /// "Auf Server speichern".
    fn remote_merge(&mut self, remote_path: String) {
        let Some(session) = &self.remote_session else {
            self.remote_status = "Keine aktive Verbindung.".to_string();
            return;
        };
        let content = match session.read_file(&remote_path) {
            Ok(c) => c,
            Err(e) => {
                self.remote_status = e;
                return;
            }
        };
        let new_items = match self.parse_str(&content) {
            Ok(v) => v,
            Err(msg) => {
                self.status_msg = msg;
                return;
            }
        };

        let match_count = self.merge_items(&new_items);
        self.remote_active_path = Some(remote_path.clone());
        self.remote_status = format!(
            "{} {}",
            self.current_lang.translate("status_env_ok"),
            self.current_lang
                .translate("count_merged")
                .replace("{}", &match_count.to_string())
        );
        // Gleicher Grund wie bei remote_load_as_template: nach dem Abgleich
        // will der Nutzer die Tabelle sehen, nicht den Dialog.
        self.remote_window_open = false;
    }

    /// Schreibt den aktuellen Stand auf den zuletzt geladenen/abgeglichenen
    /// Remote-Pfad zurück. Legt vorher — analog zum lokalen Speichern — ein
    /// `<datei>.bak` mit dem alten Inhalt an.
    fn remote_save_active(&mut self) {
        let Some(path) = self.remote_active_path.clone() else {
            self.remote_status =
                "Keine Remote-Zieldatei ausgewählt (erst laden/abgleichen).".to_string();
            return;
        };
        let Some(session) = &self.remote_session else {
            self.remote_status = "Keine aktive Verbindung.".to_string();
            return;
        };

        let content = self.serialize_env();
        match session.write_file(&path, &content) {
            Ok(_) => self.remote_status = format!("Auf Server gespeichert: {path}"),
            Err(e) => self.remote_status = e,
        }
    }

    // ---------- Secret-Generierung ----------

    fn generate_secret(&mut self, ctx: &egui::Context) {
        let mut key_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut key_bytes[..self.secret_len.byte_count()]);

        let mut hex = String::with_capacity(self.secret_len.byte_count() * 2);
        for b in &key_bytes[..self.secret_len.byte_count()] {
            use std::fmt::Write as _;
            let _ = write!(&mut hex, "{:02x}", b);
        }

        ctx.output_mut(|o| o.copied_text = hex);
        self.status_msg = self.current_lang.translate("secret_copied").to_string();
    }

    // ---------- Validierung ----------

    fn validate_value(key: &str, value: &str) -> Result<(), String> {
        if value.is_empty() {
            return Ok(());
        }
        let upper = key.to_ascii_uppercase();

        if upper.contains("URL") || upper.contains("HOST") {
            if !value.starts_with("http://")
                && !value.starts_with("https://")
                && !value.contains("://")
                && value != "localhost"
            {
                return Err(if upper.contains("URL") {
                    "URL should start with http:// or https://".to_string()
                } else {
                    "HOST should be a hostname or full URL".to_string()
                });
            }
        }
        if upper.contains("PORT") {
            if value.parse::<u16>().is_err() {
                return Err("Port must be a number between 0 and 65535".to_string());
            }
        }
        Ok(())
    }
}

// =============================================================================
//  eframe::App
// =============================================================================

/// Erzwingt unser dunkles High-Contrast-Theme. Wird bei JEDEM Frame
/// aufgerufen (nicht nur beim Start), da eframe/winit das Theme sonst anhand
/// der Systemeinstellung (hell/dunkel) automatisch zurücksetzen kann — das
/// würde unsere Einstellung sofort wieder überschreiben.
fn apply_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = egui::Color32::from_rgb(20, 20, 23);
    visuals.faint_bg_color = egui::Color32::from_rgb(34, 34, 38);
    visuals.extreme_bg_color = egui::Color32::from_rgb(14, 14, 16);
    visuals.window_fill = egui::Color32::from_rgb(20, 20, 23);

    let field_bg = egui::Color32::from_rgb(48, 48, 54);
    let field_bg_hover = egui::Color32::from_rgb(62, 62, 70);
    let field_bg_active = egui::Color32::from_rgb(70, 70, 80);
    let bright_text = egui::Color32::from_rgb(240, 240, 240);

    visuals.widgets.inactive.bg_fill = field_bg;
    visuals.widgets.inactive.weak_bg_fill = field_bg;
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, bright_text);
    visuals.widgets.hovered.bg_fill = field_bg_hover;
    visuals.widgets.hovered.weak_bg_fill = field_bg_hover;
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.2, egui::Color32::WHITE);
    visuals.widgets.active.bg_fill = field_bg_active;
    visuals.widgets.active.weak_bg_fill = field_bg_active;
    visuals.widgets.active.fg_stroke = egui::Stroke::new(1.2, egui::Color32::WHITE);
    visuals.widgets.open.bg_fill = field_bg_active;
    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, bright_text);

    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    // WICHTIG: Feste, absolute Schriftgrößen statt Multiplikation (`*= 1.3`).
    // apply_theme() läuft bei JEDEM Frame — eine Multiplikation der jeweils
    // aktuellen Größe würde die Schrift von Frame zu Frame explodieren
    // lassen. Absolute Werte sind dagegen beliebig oft wiederholbar (idempotent).
    use egui::{FontId, TextStyle};
    style.text_styles = [
        (TextStyle::Heading, FontId::proportional(26.0)),
        (TextStyle::Body, FontId::proportional(16.0)),
        (TextStyle::Monospace, FontId::monospace(15.0)),
        (TextStyle::Button, FontId::proportional(16.0)),
        (TextStyle::Small, FontId::proportional(13.0)),
    ]
    .into();
    style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    style.spacing.button_padding = egui::vec2(10.0, 6.0);
    style.spacing.interact_size.y = 30.0;
    ctx.set_style(style);
}

impl eframe::App for EnvEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        apply_theme(ctx);
        self.render_top_panel(ctx);
        self.render_bottom_panel(ctx);
        self.render_central_panel(ctx);
        self.render_remote_window(ctx);
    }
}

impl EnvEditorApp {
    fn render_top_panel(&mut self, ctx: &egui::Context) {
        let lang = self.current_lang;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.heading(format!("{} v{}", lang.translate("title"), env!("CARGO_PKG_VERSION")));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::ComboBox::from_id_salt("lang_box")
                        .selected_text(self.current_lang.name())
                        .show_ui(ui, |ui| {
                            for l in &[
                                Language::German,
                                Language::English,
                                Language::French,
                                Language::Spanish,
                                Language::Italian,
                            ] {
                                ui.selectable_value(&mut self.current_lang, *l, l.name());
                            }
                        });

                    ui.separator();

                    if ui
                        .button(lang.translate("gen_secret"))
                        .on_hover_text(lang.translate("secret_hover"))
                        .clicked()
                    {
                        self.generate_secret(ctx);
                    }

                    ui.label(lang.translate("secret_len_lbl"));
                    egui::ComboBox::from_id_salt("secret_len_box")
                        .selected_text(self.secret_len.label())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.secret_len,
                                SecretLength::Bytes16,
                                SecretLength::Bytes16.label(),
                            );
                            ui.selectable_value(
                                &mut self.secret_len,
                                SecretLength::Bytes32,
                                SecretLength::Bytes32.label(),
                            );
                        });
                });
            });
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                if ui.button(lang.translate("load_template")).clicked() {
                    if let Some(path) = FileDialog::new()
                        .set_title(lang.translate("pick_template"))
                        .add_filter("Env-Dateien", &["env", "example"])
                        .pick_file()
                    {
                        self.load_template(path);
                    }
                }

                if ui.button(lang.translate("load_env")).clicked() {
                    if self.items.is_empty() {
                        self.status_msg = lang.translate("status_template_err").to_string();
                    } else if let Some(path) = FileDialog::new()
                        .set_title(lang.translate("pick_env"))
                        .add_filter("Env-Dateien", &["env"])
                        .pick_file()
                    {
                        self.merge_env(path);
                    }
                }

                ui.separator();

                if ui.button(lang.translate("save")).clicked() {
                    let app_version = env!("CARGO_PKG_VERSION");
                    let suggested = format!(".env_v{}", app_version);
                    if let Some(path) = FileDialog::new()
                        .set_title(lang.translate("save_title"))
                        .set_file_name(&suggested)
                        .save_file()
                    {
                        self.save_env_file(&path);
                    }
                }

                ui.separator();

                if ui.button("🌐 SSH-Verbindung...").clicked() {
                    self.remote_window_open = true;
                }

                let has_remote_target =
                    self.remote_session.is_some() && self.remote_active_path.is_some();
                if ui
                    .add_enabled(has_remote_target, egui::Button::new("☁ Auf Server speichern"))
                    .on_hover_text(
                        "Speichert die aktuelle Konfiguration auf die zuletzt geladene \
                         Remote-Datei zurück (legt vorher ein .bak an).",
                    )
                    .clicked()
                {
                    self.remote_save_active();
                }

                if self.remote_session.is_some() {
                    if ui
                        .button("🔌 Trennen")
                        .on_hover_text("Beendet die SSH-Verbindung zum Server.")
                        .clicked()
                    {
                        self.remote_disconnect();
                    }
                }

                ui.label(lang.translate("line_endings"));
                egui::ComboBox::from_id_salt("os_box")
                    .selected_text(self.target_os.label())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.target_os,
                            TargetOsFormat::Linux,
                            TargetOsFormat::Linux.label(),
                        );
                        ui.selectable_value(
                            &mut self.target_os,
                            TargetOsFormat::Windows,
                            TargetOsFormat::Windows.label(),
                        );
                    });
            });

            ui.horizontal(|ui| {
                let t_lbl = lang.translate("template_lbl");
                let g_lbl = lang.translate("target_lbl");
                let none_lbl = lang.translate("none");
                ui.small(format!(
                    "{}: {} | {}: {}",
                    t_lbl,
                    self.example_path
                        .as_ref()
                        .and_then(|p| p.file_name())
                        .map(|s| s.to_string_lossy().into_owned())
                        .unwrap_or_else(|| none_lbl.to_string()),
                    g_lbl,
                    self.env_path
                        .as_ref()
                        .and_then(|p| p.file_name())
                        .map(|s| s.to_string_lossy().into_owned())
                        .unwrap_or_else(|| none_lbl.to_string())
                ));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.search_query)
                            .hint_text(lang.translate("filter_hint")),
                    );
                });
            });

            ui.add_space(5.0);
        });
    }

    fn render_bottom_panel(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(&self.status_msg).color(egui::Color32::LIGHT_BLUE));
            });
        });
    }

    fn render_central_panel(&mut self, ctx: &egui::Context) {
        let lang = self.current_lang;

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.items.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label(lang.translate("status_start"));
                });
                return;
            }

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    let total_width = ui.available_width() - 30.0;
                    let desc_width = total_width * 0.45;
                    let key_width = total_width * 0.25;
                    let val_width = total_width * 0.30;

                    egui::Grid::new("env_grid")
                        .num_columns(3)
                        .spacing([15.0, 18.0])
                        .striped(true)
                        .min_row_height(24.0)
                        .show(ui, |ui| {
                            ui.heading(lang.translate("grid_desc"));
                            ui.heading(lang.translate("grid_key"));
                            ui.heading(lang.translate("grid_val"));
                            ui.end_row();

                            let search = self.search_query.to_lowercase();

                            for i in 0..self.items.len() {
                                // Header-Zeile
                                if let EnvItem::Header { text } = &self.items[i] {
                                    if !search.is_empty() {
                                        continue;
                                    }
                                    // Hinweis: vorherige Zeile hat bereits end_row() aufgerufen
                                    ui.horizontal(|ui| {
                                        ui.add_space(5.0);
                                        ui.label(
                                            egui::RichText::new(text)
                                                .heading()
                                                .strong()
                                                .color(egui::Color32::LIGHT_GREEN),
                                        );
                                    });
                                    ui.label("");
                                    ui.label("");
                                    ui.end_row();
                                    continue;
                                }

                                // Variable
                                let EnvItem::Variable {
                                    key,
                                    value,
                                    description,
                                    is_commented_out,
                                    show_password,
                                } = &mut self.items[i]
                                else {
                                    continue;
                                };

                                if !search.is_empty()
                                    && !key.to_lowercase().contains(&search)
                                    && !description
                                        .iter()
                                        .any(|p| p.to_lowercase().contains(&search))
                                {
                                    continue;
                                }

                                // Spalte 1: Beschreibung
                                let is_empty_desc = description.is_empty();

                                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                    ui.allocate_ui(egui::vec2(desc_width, 0.0), |ui| {
                                        if is_empty_desc {
                                            ui.add(
                                                egui::Label::new(
                                                    egui::RichText::new(lang.translate("no_desc"))
                                                        .font(egui::FontId::proportional(12.5))
                                                        .color(egui::Color32::GRAY),
                                                )
                                                .wrap(),
                                            );
                                        } else {
                                            let last_idx = description.len() - 1;
                                            ui.vertical(|ui| {
                                                for (idx, paragraph) in description.iter().enumerate() {
                                                    let is_key_specific = idx == last_idx;
                                                    // Der letzte Absatz ist die konkrete
                                                    // Erklärung zu diesem Key: normal &
                                                    // heller. Frühere Absätze sind
                                                    // allgemeiner Abschnittskontext:
                                                    // kursiv & gedimmt — aber immer noch
                                                    // gut lesbar (kein zu schwacher Kontrast).
                                                    let mut text = egui::RichText::new(paragraph)
                                                        .font(egui::FontId::proportional(14.0));
                                                    text = if is_key_specific {
                                                        text.color(egui::Color32::from_rgb(230, 230, 230))
                                                    } else {
                                                        text.italics()
                                                            .color(egui::Color32::from_rgb(165, 165, 170))
                                                    };
                                                    ui.add(egui::Label::new(text).wrap());
                                                    if idx != last_idx {
                                                        ui.add_space(4.0);
                                                    }
                                                }
                                            });
                                        }
                                    });
                                });

                                // Spalte 2: Key & Checkbox
                                let mut active = !*is_commented_out;
                                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                    ui.allocate_ui(egui::vec2(key_width, 0.0), |ui| {
                                        ui.horizontal(|ui| {
                                            if ui.checkbox(&mut active, "").changed() {
                                                *is_commented_out = !active;
                                            }
                                            if *is_commented_out {
                                                ui.label(
                                                    egui::RichText::new(key.as_str())
                                                        .strikethrough()
                                                        .weak(),
                                                );
                                            } else {
                                                ui.label(
                                                    egui::RichText::new(key.as_str()).strong(),
                                                );
                                            }
                                        });
                                    });
                                });

                                // Spalte 3: Wert
                                let item_key = key.clone();
                                let show_pwd = *show_password;
                                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                                    ui.allocate_ui(egui::vec2(val_width, 0.0), |ui| {
                                        ui.horizontal(|ui| {
                                            let validation = Self::validate_value(&item_key, value);
                                            let has_error = validation.is_err();

                                            let text_edit_width =
                                                (val_width - 45.0).max(100.0);
                                            let mut text_edit =
                                                egui::TextEdit::singleline(value)
                                                    .desired_width(text_edit_width)
                                                    .password(!show_pwd);

                                            if has_error {
                                                text_edit =
                                                    text_edit.text_color(egui::Color32::LIGHT_RED);
                                            }

                                            let response = ui.add(text_edit);
                                            if let Err(msg) = &validation {
                                                response.on_hover_text(
                                                    egui::RichText::new(msg)
                                                        .color(egui::Color32::LIGHT_RED),
                                                );
                                            }

                                            let icon = if show_pwd { "👁" } else { "🙈" };
                                            if ui
                                                .button(icon)
                                                .on_hover_text(lang.translate("toggle_visibility"))
                                                .clicked()
                                            {
                                                *show_password = !*show_password;
                                            }
                                        });
                                    });
                                });

                                ui.end_row();
                            }
                        });
                });
        });
    }
}

impl EnvEditorApp {
    /// Zeigt den Dialog zum Verbinden mit einem Server per SSH/SFTP, das
    /// Verwalten gespeicherter Profile sowie die Auswahl von
    /// {example.env, .env.example, .env} im gewählten Remote-Verzeichnis.
    fn render_remote_window(&mut self, ctx: &egui::Context) {
        if !self.remote_window_open {
            return;
        }

        let mut open = self.remote_window_open;
        egui::Window::new("🌐 SSH-Verbindung")
            .open(&mut open)
            .default_width(480.0)
            .default_pos(egui::pos2(20.0, 40.0))
            .show(ctx, |ui| {
                // ---------- Gespeicherte Profile ----------
                if !self.remote_saved_connections.is_empty() {
                    ui.label("Gespeicherte Verbindungen:");
                    let mut apply_idx: Option<usize> = None;
                    let mut delete_idx: Option<usize> = None;
                    ui.horizontal_wrapped(|ui| {
                        for (idx, conn) in self.remote_saved_connections.iter().enumerate() {
                            ui.group(|ui| {
                                if ui
                                    .button(format!("{} ({}@{})", conn.name, conn.username, conn.host))
                                    .clicked()
                                {
                                    apply_idx = Some(idx);
                                }
                                if ui.small_button("🗑").on_hover_text("Profil löschen").clicked() {
                                    delete_idx = Some(idx);
                                }
                            });
                        }
                    });
                    if let Some(idx) = apply_idx {
                        self.remote_apply_saved(idx);
                    }
                    if let Some(idx) = delete_idx {
                        self.remote_delete_saved(idx);
                    }
                    ui.separator();
                }

                // ---------- Verbindungsdaten ----------
                egui::Grid::new("remote_conn_grid")
                    .num_columns(2)
                    .spacing([8.0, 6.0])
                    .show(ui, |ui| {
                        ui.label("Host:");
                        ui.text_edit_singleline(&mut self.remote_form.host);
                        ui.end_row();

                        ui.label("Port:");
                        ui.text_edit_singleline(&mut self.remote_form.port);
                        ui.end_row();

                        ui.label("Benutzername:");
                        ui.text_edit_singleline(&mut self.remote_form.username);
                        ui.end_row();

                        ui.label("Authentifizierung:");
                        ui.horizontal(|ui| {
                            ui.radio_value(
                                &mut self.remote_form.auth_kind,
                                remote::AuthKind::Password,
                                "Passwort",
                            );
                            ui.radio_value(
                                &mut self.remote_form.auth_kind,
                                remote::AuthKind::Key,
                                "SSH-Key",
                            );
                        });
                        ui.end_row();

                        match self.remote_form.auth_kind {
                            remote::AuthKind::Password => {
                                ui.label("Passwort:");
                                ui.horizontal(|ui| {
                                    ui.add(
                                        egui::TextEdit::singleline(&mut self.remote_form.password)
                                            .password(!self.remote_form.show_password)
                                            .desired_width(200.0),
                                    );
                                    let icon =
                                        if self.remote_form.show_password { "👁" } else { "🙈" };
                                    if ui.button(icon).clicked() {
                                        self.remote_form.show_password =
                                            !self.remote_form.show_password;
                                    }
                                });
                                ui.end_row();
                            }
                            remote::AuthKind::Key => {
                                ui.label("Schlüsseldatei:");
                                ui.horizontal(|ui| {
                                    let label = self
                                        .remote_form
                                        .key_path
                                        .as_ref()
                                        .map(|p| p.to_string_lossy().to_string())
                                        .unwrap_or_else(|| "Keine ausgewählt".to_string());
                                    ui.label(label);
                                    if ui.button("Datei wählen...").clicked() {
                                        if let Some(path) = FileDialog::new()
                                            .set_title("Privaten SSH-Key wählen")
                                            .pick_file()
                                        {
                                            self.remote_form.key_path = Some(path);
                                        }
                                    }
                                });
                                ui.end_row();

                                ui.label("Passphrase (optional):");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.remote_form.key_passphrase)
                                        .password(true)
                                        .desired_width(200.0),
                                );
                                ui.end_row();
                            }
                        }

                        ui.label("Remote-Verzeichnis:");
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(&mut self.remote_form.remote_dir);
                            if ui
                                .add_enabled(
                                    self.remote_session.is_some(),
                                    egui::Button::new("🔎"),
                                )
                                .on_hover_text(
                                    "Diesen Ordner durchsuchen / neu einlesen (ohne neu zu \
                                     verbinden)",
                                )
                                .clicked()
                            {
                                let dir = self.remote_form.remote_dir.trim().to_string();
                                self.remote_refresh(&dir);
                            }
                        });
                        ui.end_row();
                    });

                ui.add_space(6.0);
                ui.checkbox(
                    &mut self.remote_form.remember,
                    "Zugangsdaten speichern (im System-Schlüsselbund)",
                );
                if self.remote_form.remember {
                    ui.horizontal(|ui| {
                        ui.label("Profilname:");
                        let mut hint = self.remote_form.profile_name.clone();
                        if hint.is_empty() {
                            hint = format!(
                                "{}@{}",
                                self.remote_form.username, self.remote_form.host
                            );
                        }
                        ui.add(
                            egui::TextEdit::singleline(&mut self.remote_form.profile_name)
                                .hint_text(hint),
                        );
                    });
                    ui.label(
                        egui::RichText::new(
                            "Host/Benutzer werden lokal als JSON gespeichert, das Passwort bzw. \
                             die Passphrase liegt verschlüsselt im Schlüsselbund des Betriebssystems.",
                        )
                        .small()
                        .color(egui::Color32::GRAY),
                    );
                }

                ui.add_space(8.0);
                if ui.button("🔌 Verbinden").clicked() {
                    self.remote_connect();
                }

                if !self.remote_status.is_empty() {
                    ui.add_space(6.0);
                    ui.label(&self.remote_status);
                }

                // ---------- Ordner-Browser ----------
                if self.remote_session.is_some() {
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label(format!("📂 {}", self.remote_form.remote_dir));
                        if ui.small_button("⬆ Übergeordneter Ordner").clicked() {
                            self.remote_navigate("..");
                        }
                    });

                    let dirs: Vec<&remote::RemoteDirEntry> = self
                        .remote_dir_entries
                        .iter()
                        .filter(|e| e.is_dir)
                        .collect();

                    if dirs.is_empty() {
                        ui.label(
                            egui::RichText::new("(keine Unterordner)")
                                .small()
                                .color(egui::Color32::GRAY),
                        );
                    } else {
                        let mut navigate_to: Option<String> = None;
                        egui::ScrollArea::vertical().max_height(140.0).show(ui, |ui| {
                            for entry in &dirs {
                                if ui.button(format!("📁 {}", entry.name)).clicked() {
                                    navigate_to = Some(entry.name.clone());
                                }
                            }
                        });
                        if let Some(name) = navigate_to {
                            self.remote_navigate(&name);
                        }
                    }
                }

                // ---------- Gefundene Dateien ----------
                if self.remote_session.is_some() {
                    ui.separator();
                    if self.remote_found_files.is_empty() {
                        ui.label(
                            "Keine der Dateien example.env / .env.example / .env im \
                             angegebenen Verzeichnis gefunden.",
                        );
                    } else {
                        ui.label("Gefundene Dateien:");
                        let mut load_template: Option<String> = None;
                        let mut load_merge: Option<String> = None;
                        for path in &self.remote_found_files {
                            ui.horizontal(|ui| {
                                ui.label(path);
                                if ui
                                    .button("Als Vorlage laden")
                                    .on_hover_text(
                                        "Ersetzt die aktuelle Ansicht komplett durch diese Datei.",
                                    )
                                    .clicked()
                                {
                                    load_template = Some(path.clone());
                                }
                                if ui
                                    .add_enabled(
                                        !self.items.is_empty(),
                                        egui::Button::new("Mit dieser abgleichen"),
                                    )
                                    .on_hover_text(
                                        "Übernimmt Werte aus dieser Datei in die bereits \
                                         geladene Vorlage (wie \"2. Bestehende .env dazuladen\").",
                                    )
                                    .clicked()
                                {
                                    load_merge = Some(path.clone());
                                }
                            });
                        }
                        if let Some(path) = load_template {
                            self.remote_load_as_template(path);
                        }
                        if let Some(path) = load_merge {
                            self.remote_merge(path);
                        }
                    }

                    if let Some(active) = &self.remote_active_path {
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(format!("Speicherziel auf Server: {active}"))
                                .italics()
                                .color(egui::Color32::LIGHT_GREEN),
                        );
                    }
                }
            });

        self.remote_window_open = open;
    }
}

// =============================================================================
//  main
// =============================================================================

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1350.0, 900.0]),
        ..Default::default()
    };

    let app_title = format!(".env Konfigurator v{}", env!("CARGO_PKG_VERSION"));

    eframe::run_native(
        &app_title,
        native_options,
        Box::new(|_cc| Ok(Box::new(EnvEditorApp::default()))),
    )
}
