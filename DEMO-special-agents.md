# Special agents — installeren en doorlopen

Versie 0.6.4, op `main`. Alles rondom dit thema zit erin; wat er bewust niet in
zit staat onderaan.

## 1. Draaien zonder je productie aan te raken

**Je hoeft niets af te sluiten.** Dit start de nieuwe build naast je draaiende
Taurus, met een eigen configmap (`C:\Users\AST\AppData\Roaming\Taurus-TEST`): eigen
`projects.json`, eigen `sessions.json`, eigen `roles.json`. Je lopende agents
blijven waar ze zijn en worden niet overgenomen.

Draait er al een testexemplaar, sluit dat dan eerst -- twee testexemplaren zouden
diezelfde Taurus-TEST-map delen en elkaar precies aandoen wat dit voorkomt. Daarna,
in een gewone PowerShell:

```powershell
cd C:\Users\AST\claude\Taurus
.\start-taurus-test.ps1
```

Het script kiest zelf de **nieuwste** build en drukt af welke versie hij start:

```
Configmap : C:\Users\AST\AppData\Roaming\Taurus-TEST
Binary    : C:\Users\AST\claude\Taurus\src-tauri\target\fixbuild\release\taurus.exe
Versie    : 0.6.3  (2026-08-17 21:31)
```

Klopt die versie niet met wat je verwacht, dan is dat meteen zichtbaar in plaats van
na een ronde uitzoeken. Een specifieke build forceren kan met `-Exe <volledig pad>`.

De titelbalk zegt **TEST**. Er staat nog geen enkele agent in, en dat is precies
wat je wilt om dit uit te proberen.

**Waarom er meerdere buildmappen zijn:** zodra een testexemplaar draait houdt het
zijn eigen exe vergrendeld, dus gaat de volgende build naar een andere map onder
`src-tauri\target\`. Het script zoekt ze allemaal (`target\*\release\taurus.exe`
plus `target\release\taurus.exe`) en pakt de nieuwste, dus je hoeft die mappen niet
te onthouden.

Eén ding is wel gedeeld: **de instellingen.** Die staan in localStorage van de
webview en niet in de configmap, dus taal, thema, "vragen voor afsluiten" en de
onthouden uitrolmap gelden voor beide exemplaren. Agents, sessies en rollen zijn
gescheiden; de voorkeuren niet.

Wil je hem later echt in gebruik nemen, dan moet Taurus wel dicht:

```powershell
Copy-Item "C:\Users\AST\claude\Taurus\src-tauri\target\fixbuild\release\taurus.exe" "C:\Tools\Taurus\taurus.exe" -Force
```

## 2. De zeven rollen zien

**Instellingen (⚙) → Agents.** Zeven regels, elk met de bron erin die je hebt
aangeleverd. Per rol twee vinkjes: *gebruik* (staat aan) en *ook als skill* (staat
uit). Haal een bron weg en het gebruik-vinkje gaat uit en wordt onklikbaar — geen
bron, geen vinkje, geen regel in de balk.

De bron is aanpasbaar. Wissel hem en je houdt dezelfde veldmap, dus de
geschiedenis van die rol blijft op één plek staan.

## 3. Een rol uitrollen

**＋** in de balk. Twee lijsten:

- **Wat voor agent?** — de zeven rollen, met een icoon, de **rol als titel** en de
  naam van de invulling eronder. Je installeert een Diagnosticus, niet een Mimir;
  wissel je later de bron, dan verandert die ondertitel en de titel niet. Plus
  *Ander ICM-adres* voor een specialist zonder rol.
- **Nieuw proces** — een map die je al hebt. Dat is wat voorheen een project heette:
  een werkproces dat je hiervandaan start.

Klik **Diagnosticus**. Dan:

- de kop zegt welke rol het is en met welke vraag je binnenkomt;
- *Waar werkt hij?* staat op **Staand** (je hebt er nog geen);
- *Waar gaat deze over?* mag leeg — dan heet de map `general`;
- de bron staat al ingevuld.

**Lezen.** Taurus kloont naar een wegwerpmap, leest wat er ligt, en meldt:

```
Mimir — names the one structural reason an agent folder stopped doing what you asked…

vak        Diagnosticus
vorm       werkmap
versie     main @ <sha>, 17 augustus 2026
bevat      eigen CLAUDE.md — die blijft ongemoeid
grootte    <n> KB
```

Dat "eigen CLAUDE.md" is echt: Mimir en Forseti leveren er een mee, de andere vier
niet. Bij Cassini staat er *geen CLAUDE.md — Taurus schrijft er een*.

Daaronder staat **Komt in** met het volledige pad — `<jouw map>\diagnosis\general`
— en eronder staat dat het een **voorstel** is. Pas het aan, of kies met 📁 een andere ouder -- dan rekent Taurus
de rest van het pad opnieuw uit onder die plek.

De map heet naar het **veld**, niet naar de invulling. Geen `Jake\blueprints`, want
dat zet je vast op Jake: wissel je de bron, dan staat je geschiedenis onder een naam
die er niet meer is. Het onderwerp blijft wel een eigen laag (`diagnosis\general`),
want twee instanties van dezelfde rol moeten naast elkaar kunnen staan -- zonder die
laag zou de tweede in de eerste komen, een clone in een clone.

In het testexemplaar stelt hij nog geen ouder voor: die wordt afgeleid uit de map
die je bestaande lokale agents delen, en dat lijstje is daar leeg. Kies dus zelf,
bijvoorbeeld `C:\Users\AST\claude\_taurus-test` -- dan blijft je echte `claude`-map
schoon terwijl je dit uitprobeert.

**Uitrollen.** De kaart verschijnt bovenin, en de toast zegt hoeveel bestanden waar
geland zijn. Hij wordt niet gestart — dat rapport is het ding dat je wilt lezen.

Doe hetzelfde met **Cassini** om de andere tak te zien (Taurus schrijft de
`CLAUDE.md`; open hem, hij wijst naar `identity.md`, `rules.md` en `reference/` en
zegt dat er geen skill nodig is).

## 4. Ingebed uitrollen

Klik **＋ → Vör**. *Waar werkt hij?* staat nu op **In: …** zodra je één werkproces
hebt — kies bijvoorbeeld `Taurus dev`. Vul bij het onderwerp `meetings` in.

Het voorstel wordt dan `<map van dat werkproces>\_coaching\meetings`: een submap in
de map van het proces waar hij over gaat. **Ook dat is een voorstel.** Wil je die
map niet in een productie-repo hebben, kies dan met 📁 een andere ouder -- Taurus
houdt de underscore-vorm aan, want ingebed blijft ingebed.

Na het uitrollen staat de kaart **ingesprongen onder zijn werkproces**, met een
streepje ernaartoe.

Doe het nog eens met onderwerp `projectmanagement`. Twee coaches in hetzelfde
werkproces, naast elkaar, elk met een eigen map onder `_coaching/`.

## 5. Een niet-ICM-repo wordt geweigerd

**＋ → Ander ICM-adres**, en vul `astetic-dev/taurus` in. Lezen geeft:

> Dit lijkt geen ICM-werkproces: geen identity.md en geen SKILL.md in de wortel.
> Wat er wel staat: .gitignore, LICENSE, README.md, package.json, src, src-tauri, …

Er is niets geschreven en er valt niets op te ruimen — de probe gebeurt in een
wegwerpmap.

## 6. Een lokale map als bron

Zet in **Instellingen → Agents** bij de Architect een pad in plaats van een adres,
bijvoorbeeld je eigen `nieuw-werkproces`-map. Rol hem dan uit: Taurus kopieert in
plaats van te klonen, met dezelfde nooit-overschrijven-regels, en het rapport zegt
*lokale bron: geen versiebewaking*. Die kaart krijgt dus nooit een pijltje.

## 7. Versiebewaking

Push iets naar een van je rol-repo's en herstart Taurus. Op de kaart staat een
groen **↑**. Klik erop:

```
Mimir — nieuwe versie
jij hebt     <sha>
er is        <sha>
dit blijft   alles wat hier al gemaakt is
             C:\Users\AST\claude\Mimir\diagnosis\general
```

**Bijwerken** doet `git pull --ff-only` in díe map, en niets anders. Geen
`diagnosis/` wordt heropend. **Laat maar** onthoudt precies die versie; een
volgende commit vraagt opnieuw.

Wil je de weigeringen zien: zet met de hand iets in een bestand in die map en klik
Bijwerken. Dan komt *"Je hebt in deze map zelf iets gewijzigd"* — en de map blijft
ongemoeid. Er draait nooit `reset`, `checkout --force` of `clean`.

## 8. Afsluiten

Sluit Taurus met een agent open. Nu komt er een vraag, met per sessie wat er
gebeurt: wat op een mux draait blijft doorlopen en is met ⇱ weer op te pikken, wat
lokaal draait stopt en is bij de volgende start als conversatie te hervatten. Zet
je de opstartkeuze op *schoon beginnen*, dan belooft de vraag dat laatste niet meer.

**Annuleer** laat alles draaien. **Niet meer vragen** komt terug als een vinkje in
Instellingen → Sessies.

## 9. De linkerbalk

Twee secties met **elk hun eigen lijst en eigen scrollbalk**: agents en processen
groeien onafhankelijk, dus een lange processenlijst schuift je agents niet uit
beeld. `PROCESSEN` is nu een echte sectiekop met dezelfde vorm en hetzelfde
formaat als `AGENTS` en `DROPZONE` -- geen streepje in de lijst meer.

- **Beide secties zijn in te klappen** met de `−` in hun kop; dat wordt een `+` en
  het blijft onthouden tussen starts.
- **Filter op de processen** achter de `⌕` in de processenkop, op naam of pad. Het
  veld blijft staan zolang er iets in staat, zodat je je filter niet kwijt bent
  door de knop per ongeluk aan te tikken. Dit is issue #42, en met elf processen is
  hij niet langer optioneel.
- **De machines-knop (🖥) staat rechtsboven** bij ⚙ en ⟳ -- die hoort bij de app,
  niet bij je agentlijst, en het geeft de sectiekop links de ruimte terug.
- **De knoppen zijn kleiner en staan dichter op elkaar.**
- **Geen kleuren.** Die blijven op de tabs, waar ze wel sessies onderscheiden.
- **De versie staat rechtsboven**, niet meer onderin de balk.
- **De twee voetlinks zijn weg.** Het opdrachtoverzicht is de **≡** in de agentkop;
  *Processen beheren* staat in Instellingen → Agents.
- **De dicteerknop is er alleen als dicteren kan** -- is de spraakengine of het
  model niet geïnstalleerd, dan staat hij er niet in plaats van er te staan en
  niets te doen.
- **De scrollbalken** zijn dezelfde onopvallende als die van de bestandslijst in de
  dropzone.

## 9b. Wat er ook nog is rechtgezet

- De **✎** op een kaart bewerkt nu díe agent. Hij opende hiervoor de hele lijst,
  ingeklapt, en negeerde waar je op klikte.
- **Verwijderen** doet overal hetzelfde. Een proces verwijderen neemt de kaarten van
  zijn ingebedde rollen mee, met het aantal in de bevestiging. Er gaat niets van
  schijf, en de toast noemt het pad dat blijft staan.
- Bij het aanmaken van een proces zegt Taurus **vóór** het opslaan of er een
  `CLAUDE.md` in de map staat. Ook als hij er wél is; dat zei de app nooit.

## 10. Een leesrol op iets anders richten

Rechtermuisknop op een kaart. Dat opende hiervoor meteen het verplaats/sync-scherm;
nu is dat één regel in een menu, met daaronder de leesrollen die je hebt staan —
Cassini, Mimir, Forseti, als je ze uitgerold hebt.

Klik **"Laat Cassini hiernaar kijken"** op een van je bestaande kaarten. Taurus
selecteert Cassini en zet het pad van die kaart in zijn taak. Starten doe je zelf,
zodat je nog ziet wat er gaat gebeuren.

Richten is niet hetzelfde als inbedden. Inbedden zet een submap in een map die van
jou is; richten laat een staande rol ergens naar kíjken zonder er iets neer te
zetten. Daarom gaat het pad in de taak en niet in een uitrol — en daarom kun je een
leesrol ook op een map richten die geen ICM-werkproces is.

## 11. Opdrachten worden bewaard

Start een rol met een taak erin. Taurus legt die taak vast in de werkplek van die
rol zelf, als `_assignments.md`:

```markdown
# Assignments

What was asked of this role, newest last. Written by Taurus.

## 2026-08-17 14:32

Kijk naar deze map en doe wat je hoort te doen: C:\Users\AST\claude\ontwikkelmap
```

De **≡** in de kop van de agentlijst opent het overzicht. Dat leest de bestanden van al je rollen en
zet ze per rol op een rij, nieuwste bovenaan, met erbij of die rol staand is of in
welk werkproces hij zit.

Het bestand staat in de map van de rol zelf, dus onder zijn eigen map in de
standaard uitrolmap, naast de uitkomst waar de vraag bij hoort. Het is ongetrackt
en staat in `.git/info/exclude`, dus het geldt niet als wijziging van de repo van
de eigenaar en het blokkeert **Bijwerken** niet.

Een sessie zonder taak legt niets vast: dat is geen opdracht.

---

## Bewust niet gebouwd

**Repositories downloaden voor onderzoek.** Kvasir regelt dat zelf, in zijn eigen
werkplek, en ruimt het daarna op. Taurus hoeft daar niets voor te doen -- en dat is
ook waarom de andere helft van het richten (een adres naar een wegwerpmap klonen)
er niet in zit.

**Een agent registreren die door een agent gemaakt is.** Als Jake of Heimdall een
map achterlaat, druk je zelf op + en kies je hem als map die je al hebt. Dat werkt
vandaag; de automatische variant wacht tot er een rol is die het afgesproken
bestandje schrijft, anders bouwen we een lezer voor iets wat niemand schrijft.

## Niet getest, en dat hoor je te weten

- **Uitrollen naar een remote host.** Het pad is er en is in de code gemarkeerd,
  maar heeft nooit tegen een echte machine gedraaid.
- **`ook als skill`** installeert naar `~/.claude/skills/icm-<rol>/`. Gebouwd, niet
  gedraaid.
- **De schermen zelf.** De Rust-kant heeft 124 tests plus vier die echt met GitHub
  praten; de frontend is statisch gecontroleerd (syntax, elk element bestaat, alle
  i18n-sleutels in nl én en). Maar ik kon de launcher niet starten — deze sessie
  draait erin.
