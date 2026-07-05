# 🔒 OxiCloud .env Konfigurations-Editor

Ein grafischer Editor (GUI) für Windows, entwickelt in **Rust** mit dem `eframe`/`egui`-Framework. Das Tool liest Umgebungsvariablen aus Vorlagen (z. B. `example.env`), parst die dazugehörigen Beschreibungen und ermöglicht ein sicheres, komfortables Bearbeiten und Verwalten von Konfigurationen direkt über eine Benutzeroberfläche.

Gemini hat einen Teil der Arbeit erledigt, Feinschliff und Fehlerbehebung erfolgten anschließend mit Claude.

## ✨ Features

* **Intelligenter Parser:** Erkennt sowohl aktivierte Variablen als auch auskommentierte Platzhalter (z. B. `#OXICLOUD_BASE_URL=`) und ordnet die direkt darüberliegenden Kommentare automatisch als Hilfetext zu. Echte auskommentierte Variablen (kein Leerzeichen nach `#`) werden dabei zuverlässig von reinem Hilfetext (Leerzeichen nach `#`) unterschieden.
* **Generische Abschnitts-Erkennung:** Erkennt Sektions-Header unabhängig vom Wortlaut — sowohl mehrzeilige Trennlinien-Blöcke (`# ----- \n # TITEL \n # -----`) als auch einzeilige Header (`# --- Titel ---`, `# ── Titel ──`).
* **Strukturierte Beschreibungen:** Hilfetexte werden in Absätze gegliedert. Der letzte Absatz (die konkrete Erklärung zum jeweiligen Schlüssel) wird hell hervorgehoben, frühere Absätze (allgemeiner Abschnittskontext) erscheinen kursiv/gedimmt. Legenden- und Listeneinträge (z. B. `587 = STARTTLS submission`, `1. ...`) bleiben dabei als ein zusammenhängender Block erhalten, statt in Einzelteile zu zerfallen.
* **Übersichtliche GUI:** Strukturiert die Variablen visuell nach Sektionen (Überschriften) in einem sauberen 3-Spalten-Layout (Beschreibung | Schlüssel | Wert).
* **Aktivieren/Deaktivieren per Klick:** Über Checkboxen lassen sich Variablen intuitiv ein- oder auskommentieren, ohne die Datei manuell bearbeiten zu müssen.
* **Sicherheits-Maskierung:** Felder, die sensible Daten wie `PASSWORD`, `SECRET`, `TOKEN` oder `KEY` im Namen tragen, werden in der Benutzeroberfläche automatisch maskiert (Punkte-Ansicht) und lassen sich per 👁️/🙈-Symbol ein- und ausblenden.
* **Eingabe-Validierung:** Felder mit `URL`, `HOST` oder `PORT` im Namen werden live validiert (z. B. muss ein Port zwischen 0 und 65535 liegen); ungültige Werte werden rot hervorgehoben.
* **Secret-Generator:** Erzeugt auf Knopfdruck ein kryptografisch sicheres Zufalls-Secret (wahlweise 16 oder 32 Byte, als Hex-String) über den Betriebssystem-Zufallsgenerator (`OsRng`) und kopiert es automatisch in die Zwischenablage.
* **Abgleich-Funktion:** Du kannst eine bestehende, private `.env`-Datei dazuladen. Das Programm gleicht die Werte ab und übernimmt sie direkt in die Template-Struktur.
* **Echtzeit-Filter:** Schnelle Suche nach bestimmten Variablen (Schlüssel oder Beschreibung) über ein integriertes Suchfeld.
* **Zeilenumbruch-Wahl:** Beim Speichern lässt sich wählen, ob die Datei mit Linux/Docker-typischen (LF) oder Windows-typischen (CRLF) Zeilenumbrüchen geschrieben wird.
* **Automatisches Backup:** Vor dem Überschreiben einer bestehenden Datei wird automatisch eine `.bak`-Sicherungskopie angelegt.
* **Mehrsprachig:** Komplette Benutzeroberfläche verfügbar in Deutsch, Englisch, Französisch, Spanisch und Italienisch.
* **Dunkles High-Contrast-Theme:** Bewusst dunkel gehaltenes Farbschema mit vergrößerter Schrift und deutlich sichtbaren Eingabefeldern — unabhängig vom System-Theme des Betriebssystems.

---

## 🚀 Download & Start unter Windows

Das Programm ist bereits vollständig kompiliert und einsatzbereit. Du musst keinerlei Programmierumgebungen installieren.

1. Lade dir die ausführbare Datei **`env_editor.exe`** hier aus dem Repository herunter.
2. Starte die Anwendung einfach per Doppelklick.
3. Das Terminal-Fenster im Hintergrund wird automatisch versteckt, und die grafische Oberfläche öffnet sich sofort.

---

## 🛠️ Technische Details

* **Betriebssystem:** Windows (Nativ, 64-Bit)
* **Programmiersprache:** Rust (Edition 2021)
* **GUI-Framework:** `eframe` / `egui` (Immediate Mode GUI)
* **Native Dialoge:** `rfd` (Rust File Dialog) für die Windows-Dateiauswahl
* **Kryptografie:** `rand` (`OsRng`) für die sichere Secret-Generierung

---

## Screenshot

<img width="1920" height="1044" alt="env_editor_v0 2 0" src="https://github.com/user-attachments/assets/e7e84dc0-9847-4198-aa4e-439ce4058e78" />


