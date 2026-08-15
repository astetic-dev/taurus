## Je ziet meteen waar je binnenkomt

Meekijken begon stil. De fan-out stuurt alleen *nieuwe* bytes door, dus kom je binnen bij
een agent die staat te wachten, dan komt er niets — en een lege tab ziet er precies zo uit
als een mislukte verbinding. Je gaat dan typen om te kijken of het werkt, op de computer
van iemand anders.

Elke lokale sessie houdt nu zijn laatste 64 KB vast, en wie meekijkt krijgt dat eerst, vóór
de nieuwe bytes. Er wordt op een escape-grens gesneden, anders tekent de eerste regel als
rommel. **Er gaat geen toetsaanslag de andere kant op**: een redraw afdwingen met Ctrl-L
zou de terminal verstoren van degene die je juist komt helpen, en dat is niet aan de
bezoeker.

Daarbij één gedempte regel in je eigen tab: dat je binnen bent, dat wat je typt daar
aankomt, en dat een stil scherm een stille agent is en geen kapotte verbinding. Die regel
staat alleen bij jou — de sessie daar merkt er niets van.

Dat betekent wel dat wie meekomt het recente scherm van die sessie ziet, en niet alleen wat
er vanaf nu gebeurt. Dat is de bedoeling en geen bijeffect: iemand die niet kan zien waar je
vastloopt, kan je ook niet helpen.

Doorgemeten tussen twee instanties: A laat een regel op het scherm staan en doet daarna
niets meer; B komt binnen en ziet die regel zonder ook maar iets te typen.
