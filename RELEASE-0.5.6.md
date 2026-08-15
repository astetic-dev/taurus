## ✋ Twee keer om hulp vragen — en de agent van hiernaast openen

### Een tweede hulpvraag komt nu aan

0.5.5 maakte een verzoek aanklikbaar. Maar vroeg je er een tweede keer om — intrekken,
opnieuw vragen — dan bleef de andere kant het *eerste* token en de *eerste* agentnaam
tonen. Voor mDNS was dat namelijk dezelfde dienst onder dezelfde naam, dus er viel
niets te melden en niets opnieuw op te halen. Klikte je dan toch, dan was het token al
op.

De naam van de aankondiging draagt nu de *vraag* in plaats van de machine: intrekken
meldt die dienst af, en opnieuw vragen komt binnen als een echt nieuwe. De machinenaam
reist mee als eigenschap, want een token is geen naam om aan een mens te laten zien.

En aan jouw kant loog de balk de andere kant op. Zodra iemand meekomt neemt Taurus de
vraag in — het token is eenmalig — maar het venster luisterde daar niet naar, dus je
hand bleef staan terwijl er niets meer werd aangekondigd. De balk gaat nu vanzelf naar
beneden en zegt dat er iemand is meegekomen. Bij het opstarten klopt hij met wat er
werkelijk nog openstaat.

### De agent die in de Taurus daar draait, open je nu gewoon

Het machinescherm liet die agents al zien — niet de herdr-sessies, niet de shells. Maar
ze waren alleen te *zien*: je zag precies de agent die je zocht en kon hem niet openen,
terwijl een herdr-sessie ernaast wel gewoon opende. Terwijl juist de Taurus daar weet
welke terminal je bedoelt.

Klik je zo'n agent nu aan, dan deelt die Taurus de terminal die hij al open heeft. Er
wordt niets gestart en er verhuist niets; de tab sluiten stopt alleen het meekijken.
Twee dingen blijven anders dan bij een hulpvraag: dit is *niet* uitgenodigd, dus er
verschijnt aan die kant de gewone toestemmingsvraag (en geen antwoord binnen de tijd is
een nee), en wie op een hulptoken binnen is krijgt hier niets mee — dat token geldt
alleen voor de aangeboden sessie.

Doorgemeten tussen twee instanties: gekoppelde sleutel, toestemmingsvraag aan de
overkant, en daarna verschijnt wat je in het ene venster typt in het andere.
