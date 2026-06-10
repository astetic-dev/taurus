Taurus - Agent Launcher
=======================

Wat is dit?
-----------
Taurus start en beheert meerdere Claude Code-agents als terminal-tabs in een
venster. Elke agent draait in de map die jij kiest, zodat je nooit hoeft te
twijfelen of je lokaal (C:) of op het netwerk (X:) werkt.

Eerste start
------------
De projectenlijst is bij een verse installatie LEEG. Voeg je eigen projecten toe:
  1. Klik linksonder op "Projecten".
  2. Klik "Project toevoegen".
  3. Vul een naam in (de knop in het linkermenu), kies de werkmap via de
     bladeren-knop, en optioneel een standaard tabtitel en een taak.
  4. Klik "Opslaan".

Waar staat de configuratie?
---------------------------
  - Projecten: %APPDATA%\Taurus\projects.json
    (per gebruiker, wordt automatisch aangemaakt; mag je ook met de hand
    bewerken). Tip: typ %APPDATA% in de adresbalk van Verkenner.
  - Instellingen (taal, lettergrootte, comfort-opties): in de WebView2-opslag
    van de app, per gebruiker.

Taal
----
Instellingen -> Taal/Language: Nederlands of English.

Vereisten
---------
  - Claude Code CLI (claude.exe) bereikbaar via je PATH.
  - Windows Terminal en de WebView2-runtime (standaard aanwezig op Windows 11).

Handige bediening
-----------------
  - Rechtermuisklik op een tab: Herstart (resume gesprek), HTML-preview,
    Open map in Verkenner, Sluiten.
  - HTML-preview toont het nieuwste .html-bestand uit de werkmap, naast of in
    plaats van de terminal (instelbaar bij Instellingen). Klik op een .html-pad
    in de terminal om het direct te openen.
  - Modus per project (Standaard / Plan / Auto): kies bij het starten, of stel
    een standaard in via "Projecten".
  - "Vraag Claude volledige paden te tonen" (Instellingen, standaard aan) zorgt
    dat paden in de terminal klikbaar zijn naar de preview.
  - Sneltoetsen: Ctrl+Tab (wissel tab), Ctrl+1..9 (naar tab N), Ctrl+T (nieuw),
    Ctrl+W (sluit), Ctrl+= / Ctrl+- / Ctrl+0 (lettergrootte), Ctrl+Shift+F
    (zoeken in de terminal).
