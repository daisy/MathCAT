# Användarguide till MathCAT

MathCAT är ett verktyg som används ihop med en skärmläsare. Med hjälp av MathCAT kan du få matematiska uttryck upplästa. MathCAT gör också så att matematik kan visas på en punktskriftsskärm.

<!-- This part was written before MathCAT became native to NVDA, needs updating according to English version! -->
Just nu finns MathCAT på svenska bara tillgängligt om du använder skärmläsaren NVDA. [På NVDA:s hemsida kan du läsa om hur du laddar ner och använder NVDA (sidan är på engelska).](https://www.nvaccess.org/download/) Sedan behöver du ladda ner MathCAT som tillägg. [Här kan du ladda ner MathCAT som tillägg till NVDA (sidan är på engelska)](https://nvda-addons.org/).

Om du använder JAWS kan du just nu bara använda MathCAT på engelska. Då kommer MathCAT inbyggt, du behöver alltså inte intstallera det som tillägg.

## Börja använda MathCAT

Använd skärmläsaren på en hemsida eller i en e-bok som vanligt. När du kommer till ett matematiskt uttryck kommer skärmläsaren att läsa upp det automatiskt. Om du vill undersöka uttrycket närmare kan du aktivera navigeringsläget genom att trycka mellanslag. I NVDA kan du också använda NVDA-tangenten+Alt+M. Om du vill lämna navigeringsläget kan du trycka Escape.

### Här är de vanligaste tangentbordskommandona

- För att gå vänster, höger, upp eller ner inne i ett matematiskt uttryck, använd piltangenterna.
- Inne i en tabell, gå mellan cellerna genom att trycka CTRL+piltangenterna.
- För att gå till början av ett uttryck, tryck Home. För att gå till slutet av ett uttryck, tryck End.
- För att höra din nuvarande position, tryck mellanslag.
- För att ändra navigeringsläge, tryck Shift+upp/ner. De olika navigeringslägena finns beskrivna här: Navigering.

När du navigerar i ett uttryck kan du trycka CTRL+C för att kopiera MathML-koden för den aktuella delen av uttrycket.

Det finns många fler möjligheter när du navigerar i matematiska uttryck. Alla funktioner finns beskrivna här: Alla navigeringskommandon.

## Anpassa MathCAT efter dina behov

Det finns många möjligheter för att ställa in MathCAT utifrån dina behov. Du hittar inställningarna genom att trycka NVDA-tangenten+N sedan Inställningar, sedan MathCAT Settings. Under Kategorier finns tre valmöjligheter: Tal, Navigering och Punktskrift.

### Tal

Du kan välja mellan följande inställningar för tal. Under varje inställning hittar du en lista över alla valmöjligheter med en kort beskrivning. Standardinställningen, den som gäller om du inte gör något val, står inom hakparenteser.

- Funktionsnedsättning:
  - \[Blindhet.\] Utläsningen är helt entydig.
  - Synnedsättning. Utläsningen är kortare.
  - Inlärningssvårigheter. Utläsningen är kortare.
- Språk: (standardinställningen är samma som språket som skärmläsaren är inställd på)
  - Engelska (en)
  - Spanska (es)
  - Indonesiska (id)
  - \[Svenska (sv)\]
  - Vietnamesiska (vi)
  - Kinesiska, traditionell (zh-tw)
- Tal-läge:
  - \[ClearSpeak.\] Uttryck läses upp ungefär som läraren hade sagt dem i ett klassrum.
  - SimpleSpeak. Uttrycken läses upp mer kortfattat. Utläsningen är inte alltid entydig.
- Pratighet:
  - Kortfattad. ”Extra” ord som t.ex. ”ur” i ”kvadratroten ur x” har tagits bort. Utläsningen är inte alltid entydig och ibland blir grammatiken konstig.
  - \[Medium.\] En kompromiss mellan kortfattad och pratig.
  - Pratig. Allt läses ut. Utläsningen är entydig.
- Relativ hastighet:
  - \[100\], kan justeras mellan 1 och 1000. Ändrar hur snabbt matte läses upp jämfört med skärmläsarens hastighet. Siffran betyder procent. 100 motsvarar samma hastighet, mindre blir långsammare och mer blir snabbare.
- Pausfaktor:
  - \[1\], kan justeras mellan 0 och 10. Ändrar hur långa pauserna blir när matteuttryck läses upp.
- Matteljud:
  - \[Inget.\]
  - Pip. Ett pipljud spelas upp före och efter varje matteuttryck.
- Kemi:
  - \[Läs ut.\] Kemiska formler läses ut, t.ex. ”H två O” för $H_2O$.
  - Av. ”H nedsänkt 2 O” för $H_2O$.

### Navigering

Med hjälp av MathCAT kan du undersöka ett uttryck närmare genom att navigera i det, det vill säga gå runt och läsa en bit i taget. I inställningarna för Navigering kan du välja hur det ska gå till, och hur detaljerad information du vill ha.

Under varje inställning hittar du en lista över alla valmöjligheter med en kort beskrivning. Standardinställningen, den som gäller om du inte gör något val, står inom hakparenteser.

- Navigeringsläge som används när du börjar gå in i ett uttryck:
  - \[Översiktligt.\] Gå mellan matematiska bitar av ett uttryck (t.ex. täljare, nämnare, exponenter, uttryck inom parenteser).
  - Enkelt. Gå mellan ord, förutom när du kommer till ett avgränsat uttryck, t.ex. ett rotuttryck. Då läses hela det uttrycket upp.
  - Teckenläge. Gå mellan ord eller tal. Zooma in för att läsa upp varje bokstav eller siffra för sig.

Om du vill ändra navigeringsläge medan du går runt i ett uttryck kan du trycka Shift+Uppåtpil för att gå till Översiktligt från Enkelt (eller från Teckenläge till Enkelt). Om du trycker Shift+Nedåtpil går du från Översiktligt till Enkelt (eller från Enkelt till Teckenläge). Uppåt ger alltså mer översikt medan nedåt går djupare in i detalj.

Det finns också en ruta som du kan kryssa i för att navigeringsläget ska återställas varje gång du går in i ett uttryck. Som standard är den inte ikryssad.

- Utläsning som används när du börjar gå runt i ett uttryck:
  - \[Läs upp.\] Läser upp den del av uttrycket där du befinner dig.
  - Beskriv/översikt. Ger en översikt över det aktuella uttrycket.

Det finns också en ruta som du kan kryssa i för att utläsningsläget ska återställas varje gång du går in i ett uttryck. Som standard är den ikryssad.

- Zooma ut automatiskt när en bit av ett uttryck (t.ex. en rot) har lästs upp.
  - \[På.\] (rutan ikryssad)
  - Av. (inte ikryssad)
- Pratighet för navigering:
  - Kortfattad. ”Extra” ord som t.ex. ”ur” i ”kvadratroten ur x” har tagits bort. Utläsningen är inte alltid entydig och ibland blir grammatiken konstig.
  - \[Medium.\] En kompromiss mellan kortfattad och pratig.
  - Pratig. Allt läses ut. Utläsningen är entydig.

### Punktskrift

Du kan välja mellan följande inställningar för punktskrift. Under varje inställning hittar du en lista över alla valmöjligheter. Standardinställningen, den som gäller om du inte gör något val, står inom hakparenteser.

- Varianter av punktskriftsnotation för matematik som visas på punktskriftsskärmen:
  - CMU.
  - \[Nemeth.\]
  - Svenska.
  - UEB.
  - Vietnam.
- Punkt 7 & 8 markerar följande position i navigeringsläget:
  - Av.
  - Första tecknen.
  - \[Ändpunkterna.\]
  - Alla.

## Alla navigeringskommandon

Tabellen visar alla kommandon du kan använda för att navigera i matematiska uttryck. I första kolumnen står namnet på en tangent. I andra kolumnen beskrivs vad som händer om du trycker på tangenten. I tredje kolumnen beskrivs vad som händer om du håller ner Control och sedan trycker på tangenten. I fjärde kolumnen beskrivs vad som händer om du håller ner Shift och sedan trycker på tangenten. I femte kolumnen beskrivs vad som händer om du håller ner Control och Shift och sedan trycker på tangenten.

OBS: Instruktionen för matriser gäller också för ekvationssystem och liknande uppställningar. De går att navigera i som om de vore tabeller.

| Tangent | Enbart tangenten | +Ctrl | +Shift | +Ctrl+Shift |
| --- | --- | --- | --- | --- |
| Vänsterpil | Gå till föregående | I tabell: gå till föregående cell<br><br>I matris: gå till föregående element<br><br>(Ctrl+Alt+Vänsterpil kan också användas) | Läs föregående | Beskriv föregående |
| Högerpil | Gå till nästa | I tabell: gå till nästa cell<br><br>I matris: gå till nästa element<br><br>(Ctrl+Alt+Högerpil kan också användas) | Läs nästa | Beskriv nästa |
| Uppåtpil | Zooma ut | I tabell: gå till cellen över<br><br>I matris: gå till elementet över<br><br>(Ctrl+Alt+Uppåtpil kan också användas) | Ändra navigeringsläge till mer översiktligt | Zooma ut hela vägen |
| Nedåtpil | Zooma in | I tabell: gå till cell under  <br>I matris: gå till elementet under<br><br>(Ctrl+Alt+Nedåtpil kan också användas) | Ändra navigeringsläge till mer detaljerat | Zooma in hela vägen |
| Enter | Nuvarande position (aktuella delen av uttrycket) | Nuvarande position (hela uttrycket) |     |     |
| Siffror 1-10 (0 är 10) | Gå till markör | Placera ut markör | Läs upp markör | Beskriv markör |
| Mellanslag | Läs nuvarande | Läs nuvarande cell | Ändra tal-läge till Läs eller Beskriv | Beskriv nuvarande |
| Home | Gå till början av uttrycket | Gå till början av raden | I matris: Gå till början av kolumnen<br><br>I kolumn: Gå till översta elementet |     |
| End | Gå till slutet av uttrycket | Gå till slutet av raden | I matris: Gå till slutet av kolumnen<br><br>I kolumn: Gå till nedersta elementet |     |
| Backsteg | Gå tillbaka till senaste positionen |     |     |     |

## Har du synpunkter eller behöver du hjälp?

MathCAT är ett verktyg som från början är utvecklat på engelska. Myndigheten för tillgängliga medier (MTM) och Specialpedagogiska Skolmyndigheten (SPSM) har översatt det till svenska. Har du synpunkter på översättningen, eller behöver du hjälp med att börja använda MathCAT? Kontakta oss gärna på [info@mtm.se](mailto:info@mtm.se) eller [spsm@spsm.se](mailto:spsm@spsm.se). Skriv MathCAT i ärenderaden på ditt mejl.