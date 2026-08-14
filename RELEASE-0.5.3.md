## 🖥 Taurus toont agents

Een machine is een plek. Een agent is het werk. 0.5.3 haalt alles wat daartussen
zat — routes, mux-sessies, sessienamen met een hash erachter — uit de weg, en
voegt één ding toe dat er nog niet was: je hand opsteken als je vastloopt.

**Het punt:** ssh, tmux en herdr maken de weg vrij zodat er een agent kan starten.
Ze zijn leidingwerk en nooit iets wat je kiest. En **geen agent betekent dat er
niets is om mee te verbinden** — geen keuze met een waarschuwingslabel erop, maar
geen keuze.

### Eén scherm voor je eigen machines (#124)

- **Eén regel per computer**, niet per route. Dezelfde machine stond er drie keer
  in zodra je hem over sshd, WSL en Taurus kon bereiken, en daardoor moest de
  *naam* het onderscheid dragen — "(Taurus-host)" lekte door naar tab-badges,
  dropdowns en elke agentkaart. Routes staan nu klein onder de machine, met een
  eigen signaalbolletje, en de Taurus-route heeft voorkeur: die vraagt geen
  sleuteluitwisseling en geen sshd aan de andere kant.
- Bestaande `hosts.json` hoeft niet herschreven: routes naar hetzelfde adres
  vallen vanzelf samen.
- **"Nieuwe agent…"** vervangt Connect. Die startte iets naamloos in een map die
  je niet had gekozen; nu opent het gewone startformulier met de werkmap van die
  machine ingevuld.

### Agents in plaats van sessies (#128)

De sessielijst somde herdr's boekhouding op. Een mux-sessie zonder agent is een
lege terminal, en een lege terminal is een cmd-prompt — dus je landde in een
shell op andermans machine terwijl je bij je werk wilde zijn.

- Het machinescherm toont nu **agents**, uit twee bronnen tegelijk: wat Taurus
  daar over ssh startte, én wat er in de Taurus óp die machine draait. Voor wie
  kijkt is dat hetzelfde ding.
- Lege sessies staan eronder als **opruimwerk**, met een knop. Ze bestaan echt,
  maar ze zijn geen keuze meer.

### Terug naar je werk (#129)

Na een herstart voor een driverinstallatie kwamen twee van vier sessies terug —
en de andere twee waren **helemaal weg uit Taurus**. Dat was geen pech: wat niet
hervat kon worden werd overgeslagen, en direct daarna werd het enige spoor dat
Taurus bijhield overschreven met wat er nog openstond.

- Er is nu een **geschiedenis** die daar niet aan meedoet: eraan toevoegen en
  bijwerken, nooit iets weghalen omdat een herstart mislukte.
- **Opstarten vraagt** in plaats van stilletjes te doen. Wat openstond staat
  voorgevinkt, de rest eronder, en wat nu niet kan hervatten houdt zijn regel
  mét de reden. "Niets openen" en Escape verliezen niets.
- Instelling: *vragen* (nieuw, standaard) / *stil hervatten* / *schoon beginnen*.
- De **⇱-knop** is de plek waar je terugkomt: eerst op deze computer, dan op je
  eigen machines. Met "jouw machines" in de kop, want een collega hoort daar niet.

### Vraagmodus (#125)

Vastgelopen, of je limiet is op? Rechtsklik op de tab → **Vraag om hulp**.

- Je machine kondigt dat **verzoek** aan over mDNS, zolang je hand omhoog staat —
  zoals een bluetooth-apparaat dat zichtbaar is terwijl het koppelt en de rest van
  de tijd niet bestaat. Trek je hem in, dan is er niets meer te zien.
- Een collega ziet *arjen op ursu vraagt hulp bij ZGV-SAML-debug* en doet mee met
  één klik. **Geen pairing-popup**: degene die die popup zou moeten beantwoorden
  is juist degene die het vroeg. De uitnodiging ís de toestemming.
- Hij landt in **jouw** terminal en typt daarin mee. Twee toetsenborden op één
  agent — en overnemen kan niet: het werk blijft waar het probleem zit.
- Die toegang is smal: met een hulptoken kan de verbinding niets anders dan
  meelezen. Al het andere wordt geweigerd en in het audit-spoor gezet.
- De aankondiging bindt aan **alleen de vertrouwde interface**, niet aan elk adres
  van elke adapter — anders stuur je je Hyper-V- en WSL-indeling het netwerk op.

### Macht volgt toezicht (#126)

- **Meekijken** geeft de agent zijn eigen modus: er zit iemand naast, en dat
  toezicht *is* de controle.
- **Onbeheerd toestaan** geeft `dontAsk`: vraagt niets en weigert wat niet vooraf
  is toegestaan. Dat is de goede kant om op te falen als er niemand kijkt — een
  popup die niemand beantwoordt is een hangende sessie, geen veiligheid.
- Een vinkje op de popup geeft vol beheer, met een waarschuwing die zegt wát het
  geeft in plaats van "weet je het zeker".
- Wees eerlijk over wat dit is: het permissiemodel van de **agent**, geen grens
  van het besturingssysteem.

### Alle zes de permissiemodi (#130)

Het startformulier bood er drie, in de oude woordenschat. Claude Code 2.1.232
accepteert er zes. Nu staan ze er allemaal, met labels die zeggen wat ze *doen*:

| | |
|---|---|
| Zoals in je eigen instellingen | geen vlag — je `settings.json` blijft gelden |
| Vraagt het per stap | `manual` |
| Bewerkt bestanden zelf, vraagt voor de rest | `acceptEdits` |
| Alleen plannen, voert niets uit | `plan` |
| Model beoordeelt elk verzoek | `auto` |
| Vraagt nooit; wat niet vooraf mag, gaat niet door | `dontAsk` |
| Geen enkele controle | `bypassPermissions` |

`acceptEdits` is de alledaagse vorm van het werk. `dontAsk` is de modus voor
sessies waar niemand naar kijkt.

### Twee bugs die alleen een echte opstelling vindt

- **Er werd nooit iets aangekondigd.** De Network List Manager schrijft een
  adapter-GUID zonder accolades, `GetAdaptersAddresses` mét. Een kale vergelijking
  matchte dus nooit, en de aankondiging bleef op *elke* machine achterwege —
  zonder foutmelding, want "geen vertrouwd netwerk" is een geldig antwoord.
- **Hulp bieden vroeg een eigen SSH-sleutel.** Zonder `~/.ssh/id_*` bood de client
  niets aan, werd de sleutel-auth nooit bereikt en kreeg het token geen kans. De
  host accepteert nu een geldig token zonder sleutel — niet zwakker, want op dat
  pad werd élke sleutel geaccepteerd zodra het token klopte.

Verder: de firewallcheck loog (hij koppelde poortfilters op een sleutel die niet
overeenkomt, en negeerde Defender-block-regels die van elke uitzondering winnen),
`herdr session delete` in plaats van een `server stop` die niets deed, en een
wegwerpsessie laat aan de andere kant geen herdr-sessie meer achter.

---

**Getoetst:** 95 unit-tests, plus de hele lus tussen twee echte Taurus-processen —
hand omhoog, gezien via mDNS binnen een halve seconde, meegedaan, aan beide kanten
getypt, token daarna geweigerd. En de spiegel-tab van #121, die tot nu toe
onbevestigd was, in beide richtingen.
