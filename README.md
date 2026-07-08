# 🔒 OxiCloud .env Konfigurations-Editor

Ein grafischer **.env-Konfigurationseditor für Windows**, entwickelt in **Rust** mit **eframe/egui**.

Das Tool liest Umgebungsvariablen aus Vorlagen (z. B. `example.env`), erkennt automatisch Beschreibungen und Sektionen und ermöglicht das komfortable Bearbeiten von Konfigurationsdateien – sowohl **lokal** als auch **direkt auf einem Server per SSH/SFTP**.

> Gemini übernahm einen Teil der Grundimplementierung. Feinschliff, Fehlerbehebung sowie die vollständige SSH-/SFTP-Anbindung wurden anschließend mit Unterstützung von Claude umgesetzt.

---

## ✨ Features

### 📄 Intelligenter Parser

- Erkennt aktivierte Variablen und auskommentierte Platzhalter (z. B. `#OXICLOUD_BASE_URL=`)
- Ordnet Kommentare oberhalb automatisch als Beschreibung zu
- Unterscheidet zuverlässig zwischen:
  - echten auskommentierten Variablen (`#VARIABLE=`)
  - normalem Kommentartext (`# Beschreibung`)

### 📑 Automatische Abschnittserkennung

Unterstützt verschiedene Header-Formate, beispielsweise:

```text
# ------------------
# Datenbank
# ------------------
```

oder

```text
# --- Datenbank ---
# ── Datenbank ──
```

---

### 📝 Strukturierte Beschreibungen

- Beschreibungstexte werden automatisch in Absätze gegliedert
- Der eigentliche Hilfetext zur Variable wird hervorgehoben
- Allgemeine Abschnittsbeschreibungen erscheinen kursiv und dezenter
- Listen und Legenden bleiben als zusammenhängender Block erhalten

---

### 🖥️ Komfortable GUI

- Übersichtliches 3-Spalten-Layout
  - Beschreibung
  - Schlüssel
  - Wert
- Gruppierung nach Sektionen
- Sofortiges Bearbeiten aller Variablen

---

### ✅ Variablen aktivieren oder deaktivieren

Per Checkbox können Variablen ein- oder auskommentiert werden, ohne die Datei manuell bearbeiten zu müssen.

---

### 🔐 Automatische Maskierung sensibler Daten

Variablen mit Namen wie

- `PASSWORD`
- `SECRET`
- `TOKEN`
- `KEY`

werden automatisch verborgen.

Über das 👁️/🙈-Symbol lassen sie sich jederzeit ein- oder ausblenden.

---

### ✔ Live-Validierung

Folgende Felder werden automatisch geprüft:

- `URL`
- `HOST`
- `PORT`

Ungültige Werte werden sofort farblich hervorgehoben.

---

### 🔑 Secret-Generator

Erzeugt kryptographisch sichere Zufallswerte über `OsRng`.

Unterstützt:

- 16 Byte (Hex)
- 32 Byte (Hex)

Das Secret wird automatisch in die Zwischenablage kopiert.

---

### 🔄 Abgleich mit vorhandener `.env`

Eine bestehende private `.env` kann geladen werden.

Vorhandene Werte werden automatisch in die Template-Struktur übernommen.

---

### 🔍 Echtzeit-Suche

Filter nach

- Variablennamen
- Beschreibung

während der Eingabe.

---

### 💾 Flexible Speicherung

- Linux-/Docker-Zeilenumbrüche (LF)
- Windows-Zeilenumbrüche (CRLF)

---

### 🛡 Automatische Backups

Vor jedem Überschreiben wird automatisch eine `.bak`-Datei erzeugt.

Funktioniert sowohl

- lokal
- als auch auf dem Server.

---

### 🌍 Mehrsprachig

Die komplette Oberfläche ist verfügbar in

- 🇩🇪 Deutsch
- 🇬🇧 Englisch
- 🇫🇷 Französisch
- 🇪🇸 Spanisch
- 🇮🇹 Italienisch

---

### 🌙 High-Contrast Dark Theme

- dunkles Farbschema
- größere Schrift
- kontrastreiche Eingabefelder

Unabhängig vom Windows-Systemtheme.

---

# 🌐 Direktes Bearbeiten per SSH/SFTP

## SSH/SFTP-Verbindung

Direktes Öffnen von

- `example.env`
- `.env.example`
- `.env`

auf einem Server.

Unterstützt:

- Passwort
- SSH-Key
- verschlüsselten SSH-Key

Es wird **kein separater SFTP-Client** benötigt.

---

## 📁 Ordner-Browser

Nach erfolgreicher Verbindung kann der Zielordner bequem durchsucht werden.

Unterstützt:

- Unterordner
- Übergeordneten Ordner
- erneute Dateisuche ohne Neuverbindung

---

## 🔒 Sichere Zugangsdaten

Optional speicherbar:

- Host
- Port
- Benutzername
- Authentifizierungsmethode

Passwörter und Passphrasen werden **niemals im Klartext gespeichert**, sondern verschlüsselt im Betriebssystem-Schlüsselbund.

| Betriebssystem | Speicherung |
|---------------|------------|
| Windows | Credential Manager |
| macOS | Keychain |
| Linux | Secret Service |

---

## 🛡 Host-Key-Prüfung (TOFU)

Beim ersten Verbindungsaufbau wird der Server-Fingerabdruck gespeichert.

Ändert sich dieser später unerwartet, wird die Verbindung mit einer Warnung abgelehnt.

Dadurch werden mögliche **Man-in-the-Middle-Angriffe** erkannt.

---

## ☁ Direkt auf dem Server speichern

Änderungen können unmittelbar zurückgeschrieben werden.

Dabei wird automatisch eine `.bak`-Datei auf dem Server angelegt.

---

## 🔌 Verbindung trennen

Über **„🔌 Trennen“** kann die SSH-Verbindung jederzeit sauber beendet werden.

---

# 🚀 Installation

Es ist **keine Rust-Installation erforderlich**.

1. Lade die aktuelle `env_editor.exe` aus den **Releases** herunter.
2. Starte die Datei per Doppelklick.
3. Das Konsolenfenster bleibt verborgen.
4. Die grafische Oberfläche startet sofort.

---

# 🛠 Technische Details

| Komponente | Beschreibung |
|------------|--------------|
| Betriebssystem | Windows (64-Bit) |
| Sprache | Rust 2021 |
| GUI | `eframe` / `egui` |
| Dateidialoge | `rfd` |
| Kryptografie | `rand` (`OsRng`) |
| SSH/SFTP | `russh` + `russh-sftp` |
| Async Runtime | `tokio` |
| Credential Store | `keyring` |
| Konfigurationsdateien | `serde`, `serde_json` |
| Benutzerpfade | `dirs` |

---

# 📸 Screenshots

<img width="1920" height="1044" alt="env_editor_v0 2 0" src="https://github.com/user-attachments/assets/e7e84dc0-9847-4198-aa4e-439ce4058e78" />
<img width="1015" height="698" alt="env_editor_v0 3 0-with_ssh" src="https://github.com/user-attachments/assets/31672bb4-611e-4335-b328-d2f55d01aeb2" />

---

# 📄 Lizenz

Dieses Projekt steht unter der MIT-Lizenz.

---

# ❤️ Mitwirkende

- Eigenentwicklung
- Unterstützung durch Gemini (Grundimplementierung)
- Unterstützung durch Claude (Feinschliff, SSH/SFTP und Fehlerbehebung)


