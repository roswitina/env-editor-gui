# 🔒 OxiCloud .env Konfigurations-Editor

Ein grafischer Editor (GUI) für Windows, entwickelt in **Rust** mit dem `eframe`/`egui`-Framework. Das Tool liest Umgebungsvariablen aus Vorlagen (z. B. `example.env`), parst die dazugehörigen Beschreibungen und ermöglicht ein sicheres, komfortables Bearbeiten und Verwalten von Konfigurationen direkt über eine Benutzeroberfläche.

https://gemini.google.com hat den Großteil der Arbeit erledigt.

## ✨ Features

* **Intelligenter Parser:** Erkennt sowohl aktivierte Variablen als auch auskommentierte Platzhalter (z. B. `#OXICLOUD_BASE_URL=`) und ordnet die direkt darüberliegenden Kommentare automatisch als Hilfetext zu.
* **Übersichtliche GUI:** Strukturiert die Variablen visuell nach Sektionen (Überschriften) in einem sauberen 3-Spalten-Layout (Beschreibung | Schlüssel | Wert).
* **Aktivieren/Deaktivieren per Klick:** Über Checkboxen lassen sich Variablen intuitiv ein- oder auskommentieren, ohne die Datei manuell bearbeiten zu müssen.
* **Sicherheits-Maskierung:** Felder, die sensible Daten wie `PASSWORD`, `SECRET` oder `KEY` im Namen tragen, werden in der Benutzeroberfläche automatisch maskiert (Punkte-Ansicht).
* **Abgleich-Funktion:** Du kannst eine bestehende, private `.env`-Datei dazuladen. Das Programm gleicht die Werte ab und übernimmt sie direkt in die Template-Struktur.
* **Echtzeit-Filter:** Schnelle Suche nach bestimmten Variablen über ein integriertes Suchfeld.

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

---

## Screenshot

<img width="2215" height="1247" alt="grafik" src="https://github.com/user-attachments/assets/ffab22d9-1580-4dec-94e3-eb707746ebaa" />

