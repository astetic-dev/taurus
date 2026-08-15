## 🔎 De opstartvraag kent nu het antwoord

0.5.3 verving het stille hervatten door een vraag: *welke sessies wil je openen?*
Die vraag kende alleen wat Taurus zélf had opgeschreven — en dat was één regel,
terwijl er meerdere sessies liepen. Zo is vragen slechter dan gewoon doen.

### Alles wat Claude nog weet

Taurus leest nu Claude's eigen projectmappen. Gemeten op dit werkstation: 62
mappen, 192 gesprekken, de nieuwste 60 gelezen in 770 ms — buiten de hoofdthread,
en per gesprek stopt hij zodra hij genoeg weet.

- **De werkmap komt uit het gesprek zelf**, nooit uit de mapnaam. Die naam vervangt
  elk niet-alfanumeriek teken door een streepje en is niet terug te rekenen.
- **Het label** is Claude's eigen titel als die er is ("Verlorene Claude-sessie
  terugvinden"), anders de samenvatting, anders de eerste regel die jij typte,
  anders de mapnaam.
- Een weggeklikte melding, een slash-commando en de "This session is being
  continued from"-preambule staan óók als gebruikersregel in een transcript, en
  leverden titels op als *"A session-scoped Stop hook is now active with
  condition"*. Daar zit een structureel kenmerk op (`isMeta`, `isCompactSummary`),
  dus daarop wordt gefilterd — niet op de tekst.
- Staat er meer dan er gelezen is, dan zegt het scherm dat. Een stille afkapping
  leest als "meer is er niet".

Dezelfde lijst zit onder **⇱ → op deze computer**, dus je hoeft er niet op te
wachten tot de volgende keer opstarten.

### Ouderdom is geen slot meer

Een sessie ouder dan een dag werd overgeslagen. Dat was de oude
automatisch-hervatten-heuristiek, en die hoort hier niet: `claude --resume` doet
het prima op een gesprek van vorige week, en jij kiest zelf. Hoe oud hij is staat
rechts in de rij — informatie, geen slot. Een ontbrekend transcript of een machine
die niet meer bestaat blokkeert nog wel, want die kúnnen niet hervatten.

### De layout

Het vinkje nam ongeveer 80% van de rij en de tekst viel eraf: een `<input
type=checkbox>` in een flexrij rekt mee tenzij je het verbiedt. Vaste 15px, de
naamkolom mag krimpen, en de lijst scrollt in plaats van het venster voorbij zijn
eigen knoppen te duwen.
