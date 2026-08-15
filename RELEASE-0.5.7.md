Alles uit 0.5.6 — een tweede hulpvraag die wél aankomt, een balk die vanzelf naar
beneden gaat, en de agent van hiernaast die je gewoon kunt openen — plus één ding dat
pas zichtbaar werd toen ik 0.5.6 tegen de echte opstelling hield.

## De Taurus-route wordt afgeleid, niet gevraagd

`ursu` staat in `hosts.json` als sshd en als WSL. Zijn Taurus luisterde al die tijd op
8287, maar er is geen route-regel op die poort — en "deel die terminal" kent alleen de
Taurus aan de overkant. De verse knop uit 0.5.6 zou dus huiswerk hebben teruggegeven
("voeg eerst de Taurus-route toe") voor precies de machine waarvoor hij gebouwd is.

Die route valt niets aan te kiezen: de poort ligt vast, en het adres en de sleutel zijn
dezelfde die je al hebt ingevuld. Heeft een machine geen eigen Taurus-route, dan wordt
hij nu afgeleid uit de route die je aanklikte. Draait daar geen Taurus, dan weigert de
verbinding — een eerlijker fout dan een klusje.

Doorgemeten met een host-regel op poort 22: de join komt gewoon aan, de
toestemmingsvraag verschijnt aan de overkant, en wat je typt staat in beide vensters.

---

## Uit 0.5.6

**Een tweede hulpvraag komt nu aan.** Intrekken en opnieuw vragen liet de andere kant
het *eerste* token en de *eerste* agentnaam zien: voor mDNS was dat dezelfde dienst
onder dezelfde naam. De aankondiging draagt nu de vraag in plaats van de machine, dus
opnieuw vragen komt binnen als een echt nieuwe.

**De balk gaat vanzelf naar beneden.** Zodra iemand meekomt neemt Taurus de vraag in —
het token is eenmalig — maar het venster luisterde daar niet naar. Nu wel, met een
melding dat er iemand is meegekomen, en bij het opstarten klopt de balk met wat er
werkelijk nog openstaat.

**De agent die in de Taurus daar draait, open je nu gewoon.** Er wordt niets gestart:
die Taurus deelt de terminal die hij al open heeft. Anders dan bij een hulpvraag is dit
niet uitgenodigd, dus er verschijnt aan die kant de gewone toestemmingsvraag — en wie
op een hulptoken binnen is krijgt hier niets mee.
