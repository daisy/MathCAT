use crate::common::*;
use anyhow::Result;

#[test]
fn one() -> Result<()> {
    let expr = r#"<math>    <semantics>
    <mrow>
    <mi>ln</mi>
    <mi>χ</mi>
    <mo>=</mo>
    <mn>2</mn>
    <mo>{</mo>
    
    <mrow>
    <mo>(</mo>
    <mfrac>
    <mrow>
    <mi>χ</mi>
    <mo>−</mo>
    <mn>1</mn>
    </mrow>
    <mrow>
    <mi>χ</mi>
    <mo>+</mo>
    <mn>1</mn>
    </mrow>
    </mfrac>
    <mo>)</mo>
    </mrow>
    
    <mo>+</mo>
    
    <mfrac>
    <mn>1</mn>
    <mn>3</mn>
    </mfrac>
    
    <msup>
    <mrow>
    <mo>(</mo>
    <mfrac>
    <mrow>
    <mi>χ</mi>
    <mo>−</mo>
    <mn>1</mn>
    </mrow>
    <mrow>
    <mi>χ</mi>
    <mo>+</mo>
    <mn>1</mn>
    </mrow>
    </mfrac>
    <mo>)</mo>
    </mrow>
    <mn>3</mn>
    </msup>
    
    <mo>+</mo>
    
    <mfrac>
    <mn>1</mn>
    <mn>5</mn>
    </mfrac>
    
    <msup>
    <mrow>
    <mo>(</mo>
    <mfrac>
    <mrow>
    <mi>χ</mi>
    <mo>−</mo>
    <mn>1</mn>
    </mrow>
    <mrow>
    <mi>χ</mi>
    <mo>+</mo>
    <mn>1</mn>
    </mrow>
    </mfrac>
    <mo>)</mo>
    </mrow>
    <mn>5</mn>
    </msup>
    
    <mo>+</mo>
    <mo>⋯</mo>
    
    <mo>}</mo>
    </mrow>
    </semantics>
    </math>"#;
    test("el", "ClearSpeak", expr, "l n χ, ισούται με; 2; ανοίγει άγκιστρο; ανοίγει παρένθεση; το κλάσμα με αριθμητή; χ μείον 1; και παρονομαστή χ συν 1; κλείνει παρένθεση; συν; 1 τρίτο; ανοίγει παρένθεση; το κλάσμα με αριθμητή; χ μείον 1; και παρονομαστή χ συν 1; κλείνει παρένθεση στον κύβο; συν; 1 πέμπτο; ανοίγει παρένθεση; το κλάσμα με αριθμητή; χ μείον 1; και παρονομαστή χ συν 1; κλείνει παρένθεση στην πέμπτη δύναμη; συν αποσιωπητικά; κλείνει άγκιστρο")?;
    return Ok(());
    
}


#[test]
fn two() -> Result<()> {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msup><mi>sin</mi><mrow><mn>2</mn><mi>&#x3BD;</mi><mo>&#x2212;</mo><mn>1</mn></mrow></msup><mi>&#x391;</mi><mo>=</mo><mfrac><msup><mrow><mo>(</mo><mo>&#x2212;</mo><mn>1</mn><mo>)</mo></mrow><mrow><mi>&#x3BD;</mi><mo>&#x2212;</mo><mn>1</mn></mrow></msup><msup><mn>2</mn><mrow><mn>2</mn><mi>&#x3BD;</mi><mo>&#x2212;</mo><mn>2</mn></mrow></msup></mfrac><mo>{</mo><mrow><mi>sin</mi><mo>(</mo><mn>2</mn><mi>&#x3BD;</mi><mo>&#x2212;</mo><mn>1</mn><mo>)</mo><mi>&#x391;</mi><mo>&#x2212;</mo><mo>(</mo><mfrac><mrow><mn>2</mn><mi>&#x3BD;</mi><mo>&#x2212;</mo><mn>1</mn></mrow><mn>1</mn></mfrac><mo>)</mo><mi>sin</mi><mo>(</mo><mn>2</mn><mi>&#x3BD;</mi><mo>&#x2212;</mo><mn>3</mn><mo>)</mo><mi>&#x391;</mi></mrow><mspace linebreak="newline"/><mspace width="1.2em"/><mrow><mo>+</mo><mo>&#x2026;</mo><msup><mrow><mo>(</mo><mo>&#x2212;</mo><mn>1</mn><mo>)</mo></mrow><mrow><mi>&#x3BD;</mi><mo>&#x2212;</mo><mn>1</mn></mrow></msup><mo>(</mo><mfrac><mrow><mn>2</mn><mi>&#x3BD;</mi><mo>&#x2212;</mo><mn>1</mn></mrow><mrow><mi>&#x3BD;</mi><mo>&#x2212;</mo><mn>1</mn></mrow></mfrac><mo>)</mo><mi>sin</mi><mi>&#x391;</mi></mrow><mo>}</mo></math>"#;
    test("el", "ClearSpeak", expr, "η 2 νί, μείον 1 δύναμη του, ημίτονο; του κεφαλαίο άλφα; ισούται με; το κλάσμα με αριθμητή; ανοίγει παρένθεση, μείον 1, κλείνει παρένθεση υψωμένο στην νί μείον 1 τέλος δύναμης; και παρονομαστή 2 υψωμένο στην 2 νί, μείον 2 τέλος δύναμης; ανοίγει άγκιστρο; ημίτονο του; ανοίγει παρένθεση; 2 νί, μείον 1; κλείνει παρένθεση; κεφαλαίο άλφα; μείον; ανοίγει παρένθεση; το κλάσμα με αριθμητή; 2 νί, μείον 1; και παρονομαστή 1; κλείνει παρένθεση; ημίτονο του; ανοίγει παρένθεση; 2 νί, μείον 3; κλείνει παρένθεση; κεφαλαίο άλφα; συν αποσιωπητικά; ανοίγει παρένθεση, μείον 1, κλείνει παρένθεση υψωμένο στην νί μείον 1 τέλος δύναμης; ανοίγει παρένθεση; το κλάσμα με αριθμητή; 2 νί, μείον 1; και παρονομαστή νί μείον 1; κλείνει παρένθεση; ημίτονο κεφαλαίο άλφα; κλείνει άγκιστρο")?;
    return Ok(());

    
}

#[test]
fn three() -> Result<()> {
    let expr = r#" <math>
    <semantics>
    <mrow>
    
    <msubsup>
    <mi>Β</mi>
    <mi>Τ</mi>
    <mrow>
    <mn>1</mn>
    <mo>,</mo>
    <mi>π</mi>
    <mo>(</mo>
    <mi>τ</mi>
    <mo>)</mo>
    </mrow>
    </msubsup>
    
    <mo>=</mo>
    
    <msubsup>
    <mover accent="true">
    <mi>Β</mi>
    <mo>˜</mo>
    </mover>
    <mi>Τ</mi>
    <mrow>
    <mn>1</mn>
    <mo>,</mo>
    <mi>π</mi>
    <mo>(</mo>
    <mi>τ</mi>
    <mo>)</mo>
    </mrow>
    </msubsup>
    
    <mo>⊕</mo>
    
    <msup>
    <mi>ℝ</mi>
    <mi>Ν</mi>
    </msup>
    
    </mrow>
    </semantics>
    </math>"#;
    test("el", "ClearSpeak", expr, "κεφαλαίο βήτα δείκτης, κεφαλαίο τάαφ, υψωμένο στην 1 κόμμα; π του τάαφ τέλος δύναμης; ισούται με; κεφαλαίο βήτα κυματοειδής γραμμή, δείκτης, κεφαλαίο τάαφ, υψωμένο στην 1 κόμμα; π του τάαφ τέλος δύναμης; κυκλωμένο συν; οι πραγματικοί αριθμοί στην κεφαλαίο νί οστή δύναμη")?;
    return Ok(());
    
}

#[test]
fn four() -> Result<()> {
    let expr = r#" <math>    <semantics>
    <mrow>
    
    <msubsup>
    <mi>ℑ</mi>
    <mrow>
    <mi>w</mi>
    <mi>s</mi>
    <mi>e</mi>
    <mi>p</mi>
    <mo>,</mo>
    <mn>1</mn>
    </mrow>
    <mrow>
    <mi>μ</mi>
    <mo>∼</mo>
    <mi>κ</mi>
    </mrow>
    </msubsup>
    
    <mo>=</mo>
    
    <mo>{</mo>
    
    <mrow>
    
    <mo>(</mo>
    
    <mrow>
    <mi>ε</mi>
    <mo>,</mo>
    
    <msubsup>
    <mrow>
    <mo>{</mo>
    <msub>
    <msup>
    <mi>ε</mi>
    <mo>′</mo>
    </msup>
    <mi>ξ</mi>
    </msub>
    <mo>}</mo>
    </mrow>
    
    <mrow>
    <mi>ξ</mi>
    <mo>=</mo>
    <mn>1</mn>
    </mrow>
    
    <mi>μ</mi>
    </msubsup>
    </mrow>
    
    <mo>)</mo>
    
    <mo>∈</mo>
    
    <msubsup>
    <mi>ℑ</mi>
    <mrow>
    <mi>w</mi>
    <mi>s</mi>
    <mi>e</mi>
    <mi>p</mi>
    </mrow>
    <mrow>
    <mi>μ</mi>
    <mo>∼</mo>
    <mi>κ</mi>
    </mrow>
    </msubsup>
    
    <mo>:</mo>
    
    <mfrac>
    <mrow>
    <msup>
    <mi>ε</mi>
    <mn>2</mn>
    </msup>
    </mrow>
    
    <mrow>
    <msub>
    <mi>ε</mi>
    <mi>μ</mi>
    </msub>
    <msup>
    <mrow></mrow>
    <mo>′</mo>
    </msup>
    </mrow>
    </mfrac>
    
    <mo>→</mo>
    <mn>0</mn>
    
    </mrow>
    
    <mo>}</mo>
    
    </mrow>
    </semantics>
    </math>"#;
    test("el", "ClearSpeak", expr, "φράκτουρ κεφαλαίο i δείκτης, wsep κόμμα, 1 τέλος δείκτη, υψωμένο στην μί κυματοειδής γραμμή, καπα τέλος δύναμης; ισούται με; το σύνολο όλων των ανοίγει παρένθεση; έψιλον κόμμα; ανοίγει άγκιστρο; έψιλον τόνος, δείκτης ξ; κλείνει άγκιστρο δείκτης, ξ ισούται με 1 τέλος δείκτη, στην μί οστή δύναμη; κλείνει παρένθεση; ανήκει; φράκτουρ κεφαλαίο i δείκτης wsep τέλος δείκτη, υψωμένο στην μί κυματοειδής γραμμή, καπα τέλος δύναμης τέτοια ώστε το κλάσμα με αριθμητή; έψιλον στο τετράγωνο; και παρονομαστή έψιλον με 2 μετατεταγμένα μέρη, δείκτης μί εκθέτης τόνος; βέλος προς τα δεξιά 0")?;
    return Ok(());
    
}

#[test]
fn five() -> Result<()> {
    let expr = r#"  <math>    <semantics>
    <mrow>
    
    <msubsup>
    <mi>Β</mi>
    
    <mrow>
    <msub>
    <mi>δ</mi>
    <mn>0</mn>
    </msub>
    <mo>−</mo>
    <mn>1</mn>
    <mo>,</mo>
    <mi>η</mi>
    </mrow>
    
    <mrow>
    <msub>
    <mi>λ</mi>
    <mn>2</mn>
    </msub>
    <mo>−</mo>
    <mn>2</mn>
    <mi>μ</mi>
    <mo>,</mo>
    <mn>0</mn>
    </mrow>
    </msubsup>
    
    <mo>(</mo>
    <mrow>
    
    <msup>
    <mi>ε</mi>
    <mrow>
    <mo>−</mo>
    <mi>γ</mi>
    <mi>τ</mi>
    </mrow>
    </msup>
    
    <mo>,</mo>
    
    <msub>
    <mi>Κ</mi>
    <mi>∞</mi>
    </msub>
    
    </mrow>
    <mo>)</mo>
    
    <mo>⊂</mo>
    
    <msubsup>
    <mi>Β</mi>
    
    <mrow>
    <msub>
    <mi>δ</mi>
    <mn>1</mn>
    </msub>
    <mo>,</mo>
    <mi>η</mi>
    </mrow>
    
    <mrow>
    <msub>
    <mi>λ</mi>
    <mn>2</mn>
    </msub>
    <mo>−</mo>
    <mn>2</mn>
    <mi>μ</mi>
    <mo>,</mo>
    <mn>0</mn>
    </mrow>
    </msubsup>
    
    <mo>(</mo>
    <mrow>
    
    <msup>
    <mi>ε</mi>
    <mrow>
    <mo>−</mo>
    <mi>γ</mi>
    <mi>τ</mi>
    </mrow>
    </msup>
    
    <mo>,</mo>
    
    <msub>
    <mi>Κ</mi>
    <mi>∞</mi>
    </msub>
    
    </mrow>
    <mo>)</mo>
    
    </mrow>
    </semantics>
    </math>"#;
    test("el", "ClearSpeak", expr, "κεφαλαίο βήτα δείκτης; δέλτα δείκτης 0; μείον 1; κόμμα, ήτα τέλος δείκτη, υψωμένο στην λάμδα δείκτης 2; μείον 2 μί; κόμμα, 0 τέλος δύναμης; ανοίγει παρένθεση έψιλον υψωμένο στην μείον γάμμα τάαφ τέλος δύναμης, κόμμα κεφαλαίο καπα δείκτης άπειρο, κλείνει παρένθεση; είναι ένα υποσύνολο του; κεφαλαίο βήτα δείκτης; δέλτα δείκτης 1; κόμμα, ήτα τέλος δείκτη, υψωμένο στην λάμδα δείκτης 2; μείον 2 μί; κόμμα, 0 τέλος δύναμης; ανοίγει παρένθεση έψιλον υψωμένο στην μείον γάμμα, τάαφ τέλος δύναμης, κόμμα κεφαλαίο καπα δείκτης άπειρο; κλείνει παρένθεση")?;
    return Ok(());
    //theodora. fails. recognizes parenthesis as an interval but it is not. 
    //Now reads: κεφαλαίο βήτα δείκτης; δέλτα δείκτης 0; μείον 1; κόμμα, ήτα τέλος δείκτη, υψωμένο στην λάμδα δείκτης 2; μείον 2 μί; κόμμα, 0 τέλος δύναμης; το διάστημα από έψιλον υψωμένο στην μείον γάμμα, τάαφ τέλος δύναμης ως κεφαλαίο καπα δείκτης άπειρο; δεν περιέχει έψιλον υψωμένο στην μείον γάμμα, τάαφ τέλος δύναμης ή κεφαλαίο καπα δείκτης άπειρο; είναι ένα υποσύνολο του; κεφαλαίο βήτα δείκτης; δέλτα δείκτης 1; κόμμα, ήτα τέλος δείκτη, υψωμένο στην λάμδα δείκτης 2; μείον 2 μί; κόμμα, 0 τέλος δύναμης; το διάστημα από έψιλον υψωμένο στην μείον γάμμα, τάαφ τέλος δύναμης ως κεφαλαίο καπα δείκτης άπειρο; δεν περιέχει έψιλον υψωμένο στην μείον γάμμα, τάαφ τέλος δύναμης ή κεφαλαίο καπα δείκτης άπειρο
}

#[test]
fn six() -> Result<()> {
    let expr = r#"  <math>    <semantics>
    <mrow>
    
    <msup>
    <mi>α</mi>
    <mrow>
    <mn>1</mn>
    <mo>,</mo>
    <mn>1</mn>
    </mrow>
    </msup>
    
    <mo>+</mo>
    
    <msup>
    <mi>α</mi>
    <mrow>
    <mn>2</mn>
    <mo>,</mo>
    <mn>2</mn>
    </mrow>
    </msup>
    
    </mrow>
    </semantics>
    </math>"#;
    test("el", "ClearSpeak", expr, "άλφα στην 1,1 δύναμη, συν άλφα στην 2,2 δύναμη")?;
    return Ok(());
    //theodora. For some reason in en it expicitely reads the comma: alpha raised to the 1 comma, 1 power; plus, alpha raised to the 2 comma, 2 power
    //listened to the example using NVDA, no issues occur with plain "," but I am not sure if it was supposed to appear in greek tests too.
    
 }

#[test]
fn seven() -> Result<()> {
    let expr = r#"  <math>    <semantics>
    <mrow>
    
    <msub>
    <mi>ε</mi>
    <mn>1</mn>
    </msub>
    
    <mo>=</mo>
    
    <mo>{</mo>
    
    <mrow>
    
    <mi>ε</mi>
    
    <mo>,</mo>
    
    <mo>{</mo>
    
    <mrow>
    
    <msup>
    <mi>ε</mi>
    <mrow>
    <mn>0</mn>
    <mo>,</mo>
    <mn>2</mn>
    </mrow>
    </msup>
    
    <mo>,</mo>
    
    <msup>
    <mi>ε</mi>
    <mrow>
    <mn>0</mn>
    <mo>,</mo>
    <mn>5</mn>
    </mrow>
    </msup>
    
    <mo>,</mo>
    
    <mi>ε</mi>
    
    <mo>,</mo>
    
    <msup>
    <mi>ε</mi>
    <mrow>
    <mn>1</mn>
    <mo>,</mo>
    <mn>2</mn>
    </mrow>
    </msup>
    
    <mo>,</mo>
    
    <mfrac>
    <mrow>
    <msup>
    <mi>ε</mi>
    <mrow>
    <mn>1</mn>
    <mo>,</mo>
    <mn>5</mn>
    </mrow>
    </msup>
    </mrow>
    
    <mrow>
    <mo>|</mo>
    <mi>log</mi>
    <mi>ε</mi>
    <mo>|</mo>
    </mrow>
    </mfrac>
    
    </mrow>
    
    <mo>}</mo>
    
    </mrow>
    
    <mo>}</mo>
    
    </mrow>
    </semantics>
    </math>"#;
    test("el", "ClearSpeak", expr, "έψιλον δείκτης 1; ισούται με; το σύνολο έψιλον κόμμα; το σύνολο έψιλον στην 0,2 δύναμη, κόμμα; έψιλον στην 0,5 δύναμη, κόμμα, έψιλον κόμμα; έψιλον στην 1,2 δύναμη, κόμμα; το κλάσμα με αριθμητή; έψιλον στην 1,5 δύναμη; και παρονομαστή απόλυτη τιμή του λογάριθμος, έψιλον")?;
    return Ok(());
    
}

#[test]
fn eight() -> Result<()> {
    let expr = r#"  <math>    <semantics>
    <mrow>
    
    <mo>(</mo>
    
    <mtable>
    
    <mtr>
    <mtd>
    <msub>
    <mi>Σ</mi>
    <mrow>
    <mn>1</mn>
    <mo>,</mo>
    <mn>1</mn>
    </mrow>
    </msub>
    </mtd>
    
    <mtd>
    <msub>
    <mi>Σ</mi>
    <mrow>
    <mn>1</mn>
    <mo>,</mo>
    <mn>2</mn>
    </mrow>
    </msub>
    </mtd>
    </mtr>
    
    <mtr>
    <mtd>
    <msub>
    <mi>Σ</mi>
    <mrow>
    <mn>2</mn>
    <mo>,</mo>
    <mn>1</mn>
    </mrow>
    </msub>
    </mtd>
    
    <mtd>
    <msub>
    <mi>Σ</mi>
    <mrow>
    <mn>2</mn>
    <mo>,</mo>
    <mn>2</mn>
    </mrow>
    </msub>
    </mtd>
    </mtr>
    
    </mtable>
    
    <mo>)</mo>
    
    </mrow>
    </semantics>
    </math>"#;
    test("el", "ClearSpeak", expr, "2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα δείκτης 2,1; στήλη 2; κεφαλαίο σίγμα δείκτης 2,2")?;
    return Ok(());

    
}

#[test]
fn nine() -> Result<()> {
    let expr = r#"  <math>    <semantics>
    <mrow>
    
    <mo>[</mo>
    
    <!-- First Matrix -->
    <mrow>
    <mo>(</mo>
    
    <mtable>
    
    <mtr>
    <mtd>
    <msub>
    <mi>Χ</mi>
    <mn>1</mn>
    </msub>
    </mtd>
    
    <mtd>
    <mn>0</mn>
    </mtd>
    </mtr>
    
    <mtr>
    <mtd>
    
    <mo>−</mo>
    
    <msub>
    <mi>Σ</mi>
    <mrow>
    <mn>2</mn>
    <mo>,</mo>
    <mn>1</mn>
    </mrow>
    </msub>
    
    <msubsup>
    <mi>Σ</mi>
    <mrow>
    <mn>1</mn>
    <mo>,</mo>
    <mn>1</mn>
    </mrow>
    <mrow>
    <mo>−</mo>
    <mn>1</mn>
    </mrow>
    </msubsup>
    
    <msub>
    <mi>Χ</mi>
    <mn>1</mn>
    </msub>
    
    </mtd>
    
    <mtd>
    <msub>
    <mi>Χ</mi>
    <mn>2</mn>
    </msub>
    </mtd>
    </mtr>
    
    </mtable>
    
    <mo>)</mo>
    </mrow>
    
    <!-- Vector -->
    <mrow>
    <mo>(</mo>
    
    <mtable>
    
    <mtr>
    <mtd>
    <msub>
    <mi>β</mi>
    <mn>1</mn>
    </msub>
    </mtd>
    </mtr>
    
    <mtr>
    <mtd>
    <msub>
    <mi>β</mi>
    <mn>2</mn>
    </msub>
    </mtd>
    </mtr>
    
    </mtable>
    
    <mo>)</mo>
    </mrow>
    
    <mo>,</mo>
    
    <!-- Second Matrix -->
    <mrow>
    <mo>(</mo>
    
    <mtable>
    
    <mtr>
    <mtd>
    <msub>
    <mi>Σ</mi>
    <mrow>
    <mn>1</mn>
    <mo>,</mo>
    <mn>1</mn>
    </mrow>
    </msub>
    </mtd>
    
    <mtd>
    <mn>0</mn>
    </mtd>
    </mtr>
    
    <mtr>
    <mtd>
    <mn>0</mn>
    </mtd>
    
    <mtd>
    <msub>
    <mi>Σ</mi>
    <mrow>
    <mn>22.1</mn>
    </mrow>
    </msub>
    </mtd>
    </mtr>
    
    </mtable>
    
    <mo>)</mo>
    </mrow>
    
    <mo>]</mo>
    
    </mrow>
    </semantics>
    </math>"#;
    test("el", "ClearSpeak", expr, "ανοίγει αγκύλη; 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο χ δείκτης 1; στήλη 2; 0; γραμμή 2; στήλη 1; μείον κεφαλαίο σίγμα δείκτης 2,1; κεφαλαίο σίγμα δείκτης 1,1, στην μείον 1 δύναμη; κεφαλαίο χ δείκτης 1; στήλη 2; κεφαλαίο χ δείκτης 2; 2 επί ένα πίνακας-στήλη; γραμμή 1; βήτα δείκτης 1; γραμμή 2; βήτα δείκτης 2; κόμμα; ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 22.1; κλείνει αγκύλη")?;
    return Ok(());
    // theodora. now reads: ανοίγει αγκύλη; 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο χ δείκτης 1; στήλη 2; 0; γραμμή 2; στήλη 1; μείον κεφαλαίο σίγμα δείκτης 2,1; κεφαλαίο σίγμα δείκτης 1,1, στην μείον 1 δύναμη; κεφαλαίο χ δείκτης 1; στήλη 2; κεφαλαίο χ δείκτης 2; 2 επί ένα πίνακας-στήλη; γραμμή 1; βήτα δείκτης 1; γραμμή 2; βήτα δείκτης 2; κόμμα; 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 221; κλείνει αγκύλη
    // converts 22.1 --> 221
    // Issue is at src/prefs (set_separator, decimal_separator)
    
}

#[test]
fn ten() -> Result<()> {
    let expr = r#"  <math>    <semantics>
    <mrow>
    
    <mi>max</mi>
    
    <mo>{</mo>
    
    <mrow>
    <mn>1</mn>
    <mo>,</mo>
    <mn>2</mn>
    <mo>,</mo>
    <mn>3</mn>
    </mrow>
    
    <mo>}</mo>
    
    </mrow>
    </semantics>
    </math>"#;
    test("el", "ClearSpeak", expr, "μέγιστο του, συνόλου 1 κόμμα, 2 κόμμα, 3")?;
    return Ok(());
    
}


#[test]
fn eleven() -> Result<()> {
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mi>&#x3C6;</mi><mo>(</mo><msubsup><mi>&#x3A6;</mi><mi>&#x3C3;</mi><mi>&#x3BA;</mi></msubsup><mo>)</mo><mo>=</mo><munder><munder><mrow><mi>&#x3C6;</mi><mo>(</mo><msubsup><mi>&#x3A6;</mi><mi>&#x3C3;</mi><mi>&#x3BA;</mi></msubsup><mo>)</mo><mo>&#x2212;</mo><msub><mi>&#x3C6;</mi><mi>&#x3BA;</mi></msub><mo>(</mo><msubsup><mi>&#x3A6;</mi><mi>&#x3C3;</mi><mi>&#x3BA;</mi></msubsup><mo>)</mo></mrow><mo>&#x23DF;</mo></munder><mpadded lspace="-1px"><mo>&#x21C9;</mo><mn>0</mn></mpadded></munder><mo>+</mo><munder><munder><mrow><msub><mi>&#x3C6;</mi><mi>&#x3BA;</mi></msub><mo>(</mo><msubsup><mi>&#x3A6;</mi><mi>&#x3C3;</mi><mi>&#x3BA;</mi></msubsup><mo>)</mo></mrow><mo>&#x23DF;</mo></munder><msup><mi>&#x3C7;</mi><mi>&#x3BA;</mi></msup></munder><mo>&#x2192;</mo><mi>&#x3C7;</mi></math>"#;
    test("el", "ClearSpeak", expr, "φ του; ανοίγει παρένθεση; κεφαλαίο φ δείκτης σίγμα, στην καπα οστή δύναμη; κλείνει παρένθεση; ισούται με; παράσταση παράσταση φ του; ανοίγει παρένθεση; κεφαλαίο φ δείκτης σίγμα, στην καπα οστή δύναμη; κλείνει παρένθεση; μείον; φ δείκτης καπα; του; ανοίγει παρένθεση; κεφαλαίο φ δείκτης σίγμα, στην καπα οστή δύναμη; κλείνει παρένθεση με κάτω άγκιστρο από κάτω με ζεύγος βελών προς τα δεξιά 0 από κάτω; συν; παράσταση παράσταση φ δείκτης καπα; του; ανοίγει παρένθεση; κεφαλαίο φ δείκτης σίγμα, στην καπα οστή δύναμη; κλείνει παρένθεση με κάτω άγκιστρο από κάτω με χ στην καπα οστή δύναμη από κάτω; βέλος προς τα δεξιά χ")?;
    return Ok(());

}

#[test]
fn twelve() -> Result<()> {
    let expr = r#"  <math>    <semantics>
    <mrow>
    
    <msup>
    <mi>Γ</mi>
    <mn>2</mn>
    </msup>
    
    <mo>[</mo>
    
    <mrow>
    <mn>0</mn>
    <mo>,</mo>
    <mn>1</mn>
    </mrow>
    
    <mo>]</mo>
    
    </mrow>
    </semantics>
    </math>"#;
    test("el", "ClearSpeak", expr, "κεφαλαίο γάμμα στο τετράγωνο; το διάστημα από 0 ως 1, περιέχει 0 και 1")?;
    return Ok(());
    
}

#[test]
fn thirteen() -> Result<()> {
    let expr = r#"  <math>    <semantics>
    <mrow>
    
    <mi>η</mi>
    
    <mo>:</mo>
    
    <mo>(</mo>
    <mrow>
    <mn>0</mn>
    <mo>,</mo>
    <mn>1</mn>
    </mrow>
    <mo>)</mo>
    
    <mo>→</mo>
    
    <mo>[</mo>
    <mrow>
    <mn>0</mn>
    <mo>,</mo>
    <mi>∞</mi>
    </mrow>
    <mo>)</mo>
    
    </mrow>
    </semantics>
    </math>"#;
    test("el", "ClearSpeak", expr, "ήτα άνω κάτω τελεία; ανοίγει παρένθεση; 0 κόμμα, 1, κλείνει παρένθεση; βέλος προς τα δεξιά; το διάστημα από 0 ως άπειρο, περιέχει 0")?;
    return Ok(());
    
}

#[test]
fn fourteen() -> Result<()> {
    let expr = r#"<math>    <semantics>
    <mrow>
    
    <msup>
    
    <mrow>
    
    <mo>{</mo>
    
    <mrow>
    
    <!-- First M subscript -->
    <msub>
    
    <mi>Μ</mi>
    
    <mrow>
    <mo>(</mo>
    
    <mtable>
    
    <mtr>
    <mtd><mi>Χ</mi></mtd>
    <mtd><mn>0</mn></mtd>
    </mtr>
    
    <mtr>
    <mtd><mn>0</mn></mtd>
    <mtd><mi>Χ</mi></mtd>
    </mtr>
    
    </mtable>
    
    <mo>)</mo>
    </mrow>
    
    </msub>
    
    <!-- Bracketed tensor product -->
    <mo>[</mo>
    
    <mrow>
    
    <mo>(</mo>
    
    <mtable>
    
    <mtr>
    <mtd>
    <msubsup>
    <mi>σ</mi>
    <mrow>
    <mn>1</mn>
    <mo>,</mo>
    <mn>0</mn>
    </mrow>
    <mn>2</mn>
    </msubsup>
    </mtd>
    
    <mtd>
    <msub>
    <mi>γ</mi>
    <mn>0</mn>
    </msub>
    </mtd>
    </mtr>
    
    <mtr>
    <mtd>
    <msub>
    <mi>γ</mi>
    <mn>0</mn>
    </msub>
    </mtd>
    
    <mtd>
    <msubsup>
    <mi>σ</mi>
    <mrow>
    <mn>2</mn>
    <mo>,</mo>
    <mn>0</mn>
    </mrow>
    <mn>2</mn>
    </msubsup>
    </mtd>
    </mtr>
    
    </mtable>
    
    <mo>)</mo>
    
    <mo>⊗</mo>
    
    <msub>
    <mi>Ι</mi>
    <mi>ν</mi>
    </msub>
    
    </mrow>
    
    <mo>]</mo>
    
    <!-- Second M subscript -->
    <msub>
    
    <mi>Μ</mi>
    
    <mrow>
    <mo>(</mo>
    
    <mtable>
    
    <mtr>
    <mtd><mi>Χ</mi></mtd>
    <mtd><mn>0</mn></mtd>
    </mtr>
    
    <mtr>
    <mtd><mn>0</mn></mtd>
    <mtd><mi>Χ</mi></mtd>
    </mtr>
    
    </mtable>
    
    <mo>)</mo>
    </mrow>
    
    </msub>
    
    </mrow>
    
    <mo>}</mo>
    
    </mrow>
    
    <mo>+</mo>
    
    </msup>
    
    </mrow>
    </semantics>
    </math>"#;
    test("el", "ClearSpeak", expr, "το σύνολο κεφαλαίο μί δείκτης; ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο χ; στήλη 2; κεφαλαίο χ; τέλος δείκτη; ανοίγει αγκύλη; ο 2 επί 2 πίνακας; γραμμή 1; στήλη 1; σίγμα δείκτης 1,0, στο τετράγωνο, στήλη 2; γάμμα δείκτης 0; γραμμή 2; στήλη 1; γάμμα δείκτης 0; στήλη 2; σίγμα δείκτης 2,0, στο τετράγωνο; κυκλωμένο επί; κεφαλαίο ιότα δείκτης νί; κλείνει αγκύλη; επί; κεφαλαίο μί δείκτης; 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο χ; στήλη 2; κεφαλαίο χ; τέλος δείκτη, εκθέτης συν")?;
    return Ok(());
    //theodora. Now reads: Το σύνολο κεφαλαίο μί δείκτης; TEMP NAME του ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο χ; στήλη 2; κεφαλαίο χ; τέλος δείκτη; ανοίγει αγκύλη; ο 2 επί 2 πίνακας; γραμμή 1; στήλη 1; σίγμα δείκτης 1,0, στο τετράγωνο, στήλη 2; γάμμα δείκτης 0; γραμμή 2; στήλη 1; γάμμα δείκτης 0; στήλη 2; σίγμα δείκτης 2,0, στο τετράγωνο; κυκλωμένο επί; κεφαλαίο ιότα δείκτης νί; κλείνει αγκύλη; επί; κεφαλαίο μί δείκτης; TEMP NAME του 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο χ; στήλη 2; κεφαλαίο χ; τέλος δείκτη, εκθέτης συν
}

#[test]
fn fifteen() -> Result<()> {
    let expr = r#"  <math>    <semantics>
    <mrow>
    
    <!-- Left vector -->
    <mo>(</mo>
    
    <mtable>
    
    <mtr>
    <mtd>
    <msub>
    <mi>Υ</mi>
    <mn>1</mn>
    </msub>
    </mtd>
    </mtr>
    
    <mtr>
    <mtd>
    <msub>
    <mi>Υ</mi>
    <mn>2</mn>
    </msub>
    </mtd>
    </mtr>
    
    </mtable>
    
    <mo>)</mo>
    
    <mo>∼</mo>
    
    <mo>[</mo>
    
    <!-- First matrix -->
    <mo>(</mo>
    
    <mtable>
    
    <mtr>
    <mtd><mi>Χ</mi></mtd>
    <mtd><mn>0</mn></mtd>
    </mtr>
    
    <mtr>
    <mtd><mn>0</mn></mtd>
    <mtd><mi>Χ</mi></mtd>
    </mtr>
    
    </mtable>
    
    <mo>)</mo>
    
    <!-- Beta vector -->
    <mo>(</mo>
    
    <mtable>
    
    <mtr>
    <mtd>
    <msub>
    <mi>β</mi>
    <mn>1</mn>
    </msub>
    </mtd>
    </mtr>
    
    <mtr>
    <mtd>
    <msub>
    <mi>β</mi>
    <mn>2</mn>
    </msub>
    </mtd>
    </mtr>
    
    </mtable>
    
    <mo>)</mo>
    
    <mo>,</mo>
    
    <!-- Covariance matrix -->
    <mo>(</mo>
    
    <mtable>
    
    <mtr>
    <mtd>
    <msub>
    <mi>Σ</mi>
    <mrow>
    <mn>1</mn>
    <mo>,</mo>
    <mn>1</mn>
    </mrow>
    </msub>
    </mtd>
    
    <mtd>
    <msub>
    <mi>Σ</mi>
    <mrow>
    <mn>1</mn>
    <mo>,</mo>
    <mn>2</mn>
    </mrow>
    </msub>
    </mtd>
    </mtr>
    
    <mtr>
    <mtd>
    <msub>
    <mi>Σ</mi>
    <mrow>
    <mn>2</mn>
    <mo>,</mo>
    <mn>1</mn>
    </mrow>
    </msub>
    </mtd>
    
    <mtd>
    <msub>
    <mi>Σ</mi>
    <mrow>
    <mn>2</mn>
    <mo>,</mo>
    <mn>2</mn>
    </mrow>
    </msub>
    </mtd>
    </mtr>
    
    </mtable>
    
    <mo>)</mo>
    
    <mo>]</mo>
    
    </mrow>
    </semantics>
    </math>"#;
    test("el", "ClearSpeak", expr, "2 επί ένα πίνακας-στήλη; γραμμή 1; κεφαλαίο ύψιλον δείκτης 1; γραμμή 2; κεφαλαίο ύψιλον δείκτης 2; κυματοειδής γραμμή; ανοίγει αγκύλη; ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο χ; στήλη 2; κεφαλαίο χ; 2 επί ένα πίνακας-στήλη; γραμμή 1; βήτα δείκτης 1; γραμμή 2; βήτα δείκτης 2; κόμμα; 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα δείκτης 2,1; στήλη 2; κεφαλαίο σίγμα δείκτης 2,2; κλείνει αγκύλη")?;
    return Ok(());
 
}

#[test]
fn sixteen() -> Result<()> {
    let expr = r#"  <math>    <semantics>
    
    <mtable columnalign="left">
    
    <!-- First row -->
    <mtr>
    <mtd>
    
    <mfrac>
    <mi>d</mi>
    <mrow>
    <mi>d</mi>
    <mi>τ</mi>
    </mrow>
    </mfrac>
    
    <mo>(</mo>
    
    <mrow>
    
    <msup>
    <mrow>
    <mo>‖</mo>
    <mrow>
    <mo>∇</mo>
    <mi>υ</mi>
    </mrow>
    <mo>‖</mo>
    </mrow>
    <mn>2</mn>
    </msup>
    
    <mo>+</mo>
    
    <mn>2</mn>
    
    <mstyle displaystyle="true">
    <mrow>
    
    <munder>
    <mo>∫</mo>
    <mi>Ω</mi>
    </munder>
    
    <mrow>
    <mi>Φ</mi>
    <mo>(</mo>
    <mi>υ</mi>
    <mo>)</mo>
    <mi>d</mi>
    <mi>χ</mi>
    </mrow>
    
    </mrow>
    </mstyle>
    
    </mrow>
    
    <mo>)</mo>
    
    <mo>+</mo>
    
    <mn>2</mn>
    
    <msup>
    <mrow>
    <mo>‖</mo>
    
    <mrow>
    <mfrac>
    <mrow>
    <mo>∂</mo>
    <mi>υ</mi>
    </mrow>
    <mrow>
    <mo>∂</mo>
    <mi>τ</mi>
    </mrow>
    </mfrac>
    </mrow>
    
    <mo>‖</mo>
    </mrow>
    
    <mn>2</mn>
    </msup>
    
    </mtd>
    </mtr>
    
    <!-- Second row -->
    <mtr>
    <mtd>
    
    <mo>=</mo>
    
    <mn>2</mn>
    
    <mo>(</mo>
    
    <mrow>
    
    <mo>(</mo>
    
    <mrow>
    
    <mfrac>
    <mrow>
    <mo>∂</mo>
    <mi>α</mi>
    </mrow>
    <mrow>
    <mo>∂</mo>
    <mi>τ</mi>
    </mrow>
    </mfrac>
    
    <mo>,</mo>
    
    <mfrac>
    <mrow>
    <mo>∂</mo>
    <mi>υ</mi>
    </mrow>
    <mrow>
    <mo>∂</mo>
    <mi>τ</mi>
    </mrow>
    </mfrac>
    
    </mrow>
    
    <mo>)</mo>
    
    </mrow>
    
    <mo>)</mo>
    
    </mtd>
    </mtr>
    
    </mtable>
    
    </semantics>
    </math>"#;
    test("el", "ClearSpeak", expr, "πίνακας με 2 γραμμές και 1 στήλες; γραμμή 1; στήλη 1; το κλάσμα με αριθμητή d; και παρονομαστή d τάαφ; ανοίγει παρένθεση; νόρμα του ανάδελτα του ύψιλον στο τετράγωνο; συν; 2; ολοκλήρωμα επί του συνόλου κεφαλαίο ωμέγα του; κεφαλαίο φ του ύψιλον; d χ; κλείνει παρένθεση; συν; 2; νόρμα του κλάσματος με αριθμητή; μερικό διαφορικό, ύψιλον; και παρονομαστή μερικό διαφορικό, τάαφ; στο τετράγωνο; γραμμή 2; στήλη 1; ισούται με; 2; ανοίγει παρένθεση; ανοίγει παρένθεση; το κλάσμα με αριθμητή; μερικό διαφορικό, άλφα; και παρονομαστή μερικό διαφορικό, τάαφ; κόμμα; το κλάσμα με αριθμητή; μερικό διαφορικό, ύψιλον; και παρονομαστή μερικό διαφορικό, τάαφ; κλείνει παρένθεση; κλείνει παρένθεση")?;
    return Ok(());
    //theodora. fails with current rules. Should be fine when introducing genitive
    
}

#[test]
fn seventeen() -> Result<()> {
    let expr = r#"<math display="block">
    <mrow>
    <msup>
    <mo>&#x2202;</mo>
    <mn>2</mn>
    </msup>
    
    <mrow>
    <mo>(</mo>
    <mrow>
    <msup>
    <mi>&#x0394;</mi>
    <mi>&#x03B1;</mi>
    </msup>
    <mi>u</mi>
    
    <mrow>
    <mo>(</mo>
    <mrow>
    <mi>&#x03C7;</mi>
    <mo>,</mo>
    <msub>
    <mi>&#x03C4;</mi>
    <mi>&#x03BD;</mi>
    </msub>
    </mrow>
    <mo>)</mo>
    </mrow>
    </mrow>
    <mo>)</mo>
    </mrow>
    
    <mo>=</mo>
    
    <mfrac>
    <mn>1</mn>
    <mrow>
    <msup>
    <mi>&#x03BA;</mi>
    <mn>2</mn>
    </msup>
    </mrow>
    </mfrac>
    
    <mstyle displaystyle="true">
    <mrow>
    
    <munderover>
    <mo>&#x222B;</mo>
    <mrow>
    <msub>
    <mi>&#x03C4;</mi>
    <mrow>
    <mi>&#x03BD;</mi>
    <mo>&#x2212;</mo>
    <mn>1</mn>
    </mrow>
    </msub>
    </mrow>
    <mrow>
    <msub>
    <mi>&#x03C4;</mi>
    <mi>&#x03BD;</mi>
    </msub>
    </mrow>
    </munderover>
    
    <mrow>
    
    <mstyle displaystyle="true">
    <mrow>
    
    <munderover>
    <mo>&#x222B;</mo>
    <mrow>
    <mi>&#x03C4;</mi>
    <mo>&#x2212;</mo>
    <mi>&#x03B7;</mi>
    </mrow>
    <mi>&#x03C4;</mi>
    </munderover>
    
    <mrow>
    
    <msub>
    <mrow>
    <mo>(</mo>
    <mrow>
    <msup>
    <mi>&#x0394;</mi>
    <mi>&#x03B1;</mi>
    </msup>
    <mi>u</mi>
    </mrow>
    <mo>)</mo>
    </mrow>
    
    <mrow>
    <mi>&#x03C4;</mi>
    <mi>&#x03C4;</mi>
    </mrow>
    </msub>
    
    <mrow>
    <mo>(</mo>
    <mrow>
    <mi>&#x03C7;</mi>
    <mo>,</mo>
    <mi>&#x03C4;</mi>
    </mrow>
    <mo>)</mo>
    </mrow>
    
    <mi>d</mi>
    <mi>&#x03C3;</mi>
    
    </mrow>
    
    </mrow>
    </mstyle>
    
    <mi>d</mi>
    <mi>&#x03C4;</mi>
    
    </mrow>
    
    </mrow>
    </mstyle>
    
    </mrow>
    </math>"#;
    test("el", "ClearSpeak", expr, "μερικό διαφορικό στο τετράγωνο; ανοίγει παρένθεση; κεφαλαίο δέλτα στην άλφα οστή δύναμη; u; ανοίγει παρένθεση; χ κόμμα; τάαφ δείκτης νί; κλείνει παρένθεση; κλείνει παρένθεση; ισούται με; το κλάσμα με αριθμητή 1; και παρονομαστή καπα στο τετράγωνο; ολοκλήρωμα από τάαφ δείκτης, νί μείον 1 τέλος δείκτη, ως τάαφ δείκτης νί, του; ολοκλήρωμα από τάαφ μείον ήτα ως τάαφ του; ανοίγει παρένθεση; κεφαλαίο δέλτα στην άλφα οστή δύναμη; u; κλείνει παρένθεση δείκτης, τάαφ τάαφ τέλος δείκτη; ανοίγει παρένθεση; χ κόμμα, τάαφ; κλείνει παρένθεση; d σίγμα; d τάαφ")?;
    return Ok(());

}

#[test]
fn eighteen() -> Result<()> {
    let expr = r#"<math display="block">
    <mrow>
    
    <mfrac>
    <mn>1</mn>
    <mrow>
    <msup>
    <mi>&#x03BA;</mi>
    <mn>2</mn>
    </msup>
    </mrow>
    </mfrac>
    
    <mstyle displaystyle="true">
    <mrow>
    
    <!-- Integral over K -->
    <munder>
    <mo>&#x222B;</mo>
    <mi>&#x039A;</mi>
    </munder>
    
    <mrow>
    
    <!-- Integral tau_{nu-1} to tau_nu -->
    <mstyle displaystyle="true">
    <mrow>
    
    <munderover>
    <mo>&#x222B;</mo>
    
    <mrow>
    <msub>
    <mi>&#x03C4;</mi>
    <mrow>
    <mi>&#x03BD;</mi>
    <mo>&#x2212;</mo>
    <mn>1</mn>
    </mrow>
    </msub>
    </mrow>
    
    <mrow>
    <msub>
    <mi>&#x03C4;</mi>
    <mi>&#x03BD;</mi>
    </msub>
    </mrow>
    </munderover>
    
    <mrow>
    
    <!-- Integral tau_{nu-1} to tau -->
    <mstyle displaystyle="true">
    <mrow>
    
    <munderover>
    <mo>&#x222B;</mo>
    
    <mrow>
    <msub>
    <mi>&#x03C4;</mi>
    <mrow>
    <mi>&#x03BD;</mi>
    <mo>&#x2212;</mo>
    <mn>1</mn>
    </mrow>
    </msub>
    </mrow>
    
    <mi>&#x03C4;</mi>
    </munderover>
    
    <mrow>
    
    <!-- Integral sigma to sigma+kappa -->
    <mstyle displaystyle="true">
    <mrow>
    
    <munderover>
    <mo>&#x222B;</mo>
    <mi>&#x03C3;</mi>
    
    <mrow>
    <mi>&#x03C3;</mi>
    <mo>+</mo>
    <mi>&#x03BA;</mi>
    </mrow>
    </munderover>
    
    <mrow>
    
    <mi>&#x0394;</mi>
    
    <mfrac>
    <mrow>
    <msup>
    <mi>d</mi>
    <mn>2</mn>
    </msup>
    <mi>&#x03C5;</mi>
    </mrow>
    
    <mrow>
    <mi>d</mi>
    <msup>
    <mi>&#x03BB;</mi>
    <mn>2</mn>
    </msup>
    </mrow>
    </mfrac>
    
    <mrow>
    <mo>(</mo>
    <mrow>
    <mi>&#x03C7;</mi>
    <mo>,</mo>
    <mi>&#x03BB;</mi>
    </mrow>
    <mo>)</mo>
    </mrow>
    
    <mi>d</mi>
    <mi>&#x03BB;</mi>
    
    </mrow>
    
    </mrow>
    </mstyle>
    
    <mi>d</mi>
    <mi>&#x03C3;</mi>
    
    </mrow>
    
    </mrow>
    </mstyle>
    
    <mi>d</mi>
    <mi>&#x03C4;</mi>
    
    </mrow>
    
    </mrow>
    </mstyle>
    
    <mi>d</mi>
    <mi>&#x03C7;</mi>
    
    </mrow>
    
    </mrow>
    </mstyle>
    
    </mrow>
    </math>"#;
    test("el", "ClearSpeak", expr, "το κλάσμα με αριθμητή 1; και παρονομαστή καπα στο τετράγωνο; ολοκλήρωμα επί του συνόλου κεφαλαίο καπα του; ολοκλήρωμα από τάαφ δείκτης, νί μείον 1 τέλος δείκτη, ως τάαφ δείκτης νί, του; ολοκλήρωμα από τάαφ δείκτης, νί μείον 1 τέλος δείκτη, ως τάαφ του; ολοκλήρωμα από σίγμα ως σίγμα συν καπα του; κεφαλαίο δέλτα; το κλάσμα με αριθμητή; d στο τετράγωνο, ύψιλον; και παρονομαστή d, λάμδα στο τετράγωνο; ανοίγει παρένθεση; χ κόμμα, λάμδα; κλείνει παρένθεση; d λάμδα; d σίγμα; d τάαφ; d χ")?;
    return Ok(());

    
}

#[test]
fn nineteen() -> Result<()> {
    let expr = r#"<math display="block">
    <mrow>
    
    <mi>inf</mi>
    
    <mrow>
    <mo>{</mo>
    
    <mrow>
    
    <!-- ||β||_{H^λ_β(Ω)} -->
    <msub>
    <mrow>
    <mo>‖</mo>
    <mi>&#x03B2;</mi>
    <mo>‖</mo>
    </mrow>
    
    <mrow>
    <msubsup>
    <mi>&#x0397;</mi>
    <mi>&#x03B2;</mi>
    <mi>&#x03BB;</mi>
    </msubsup>
    
    <mrow>
    <mo>(</mo>
    <mi>&#x03A9;</mi>
    <mo>)</mo>
    </mrow>
    </mrow>
    </msub>
    
    <mo>:</mo>
    
    <!-- β ∈ H^λ_β(Ω) -->
    <mi>&#x03B2;</mi>
    
    <mo>&#x2208;</mo>
    
    <msubsup>
    <mi>&#x0397;</mi>
    <mi>&#x03B2;</mi>
    <mi>&#x03BB;</mi>
    </msubsup>
    
    <mrow>
    <mo>(</mo>
    <mi>&#x03A9;</mi>
    <mo>)</mo>
    </mrow>
    
    <mo>,</mo>
    
    <!-- β|Γ = υ -->
    <mi>&#x03B2;</mi>
    
    <mrow>
    <mo>|</mo>
    <mi>&#x0393;</mi>
    </mrow>
    
    <mo>=</mo>
    
    <mi>&#x03C5;</mi>
    
    </mrow>
    
    <mo>}</mo>
    </mrow>
    
    </mrow>
    </math>"#;
    test("el", "ClearSpeak", expr, "ινφίμουμ του; συνόλου νόρμα του βήτα δείκτης; η λάμδα δύναμη του, κεφαλαίο ήτα δείκτης βήτα; του κεφαλαίο ωμέγα τέλος δείκτη; άνω κάτω τελεία; βήτα ανήκει; η λάμδα δύναμη του, κεφαλαίο ήτα δείκτης βήτα; του κεφαλαίο ωμέγα; κόμμα; βήτα, κάθετη γραμμή, κεφαλαίο γάμμα; ισούται με, ύψιλον")?;
    return Ok(());
    //theodora. not wrong but seems that it is ambiguous about where the sub ends. The way it's curretly read, the reader could assume sub is b(Ω) and not b. Could it also be a problem in english?
}

#[test]
fn twenty() -> Result<()> {
    let expr = r#"<math display="block">
    <mrow>
    
    <!-- K^β(ρ) -->
    <msup>
    <mi>&#x039A;</mi>
    <mi>&#x03B2;</mi>
    </msup>
    
    <mrow>
    <mo>(</mo>
    <mi>&#x03C1;</mi>
    <mo>)</mo>
    </mrow>
    
    <mo>=</mo>
    
    <mrow>
    <mo>{</mo>
    
    <mrow>
    
    <!-- υ ∈ K^β -->
    <mi>&#x03C5;</mi>
    
    <mo>&#x2208;</mo>
    
    <msup>
    <mi>&#x039A;</mi>
    <mi>&#x03B2;</mi>
    </msup>
    
    <mo>:</mo>
    
    <!-- ||υ|| < ρ -->
    <mrow>
    <mo>‖</mo>
    <mi>&#x03C5;</mi>
    <mo>‖</mo>
    </mrow>
    
    <mo>&lt;</mo>
    
    <mi>&#x03C1;</mi>
    
    </mrow>
    
    <mo>}</mo>
    </mrow>
    
    </mrow>
    </math>"#;
    test("el", "ClearSpeak", expr, "η βήτα δύναμη του, κεφαλαίο καπα; του ρ; ισούται με; το σύνολο όλων των ύψιλον ανήκει, κεφαλαίο καπα στην βήτα οστή δύναμη τέτοια ώστε νόρμα του ύψιλον; είναι μικρότερο από ρ")?;
    return Ok(());
    
}

#[test]
fn twentyone() -> Result<()> {
    let expr = r#"<math display="block">
    <mrow>
    
    <!-- φ₀ -->
    <msub>
    <mi>&#x03C6;</mi>
    <mn>0</mn>
    </msub>
    
    <mo>&#x2208;</mo>
    
    <!-- Γ̃^∞ -->
    <msup>
    <mover accent="true">
    <mi>&#x0393;</mi>
    <mo>&#x02D9;</mo>
    </mover>
    <mi>&#x221E;</mi>
    </msup>
    
    <!-- (Υ₀) -->
    <mrow>
    <mo>(</mo>
    <mrow>
    <msub>
    <mi>&#x03A5;</mi>
    <mn>0</mn>
    </msub>
    </mrow>
    <mo>)</mo>
    </mrow>
    
    </mrow>
    </math>
    "#;
    test("el", "ClearSpeak", expr, "φ δείκτης 0; ανήκει; η απειροστή δύναμη του, κεφαλαίο γάμμα τελεία; του; ανοίγει παρένθεση; κεφαλαίο ύψιλον δείκτης 0; κλείνει παρένθεση")?;
    return Ok(());
    
}


#[test]
fn twentytwo() -> Result<()> {
    let expr = r#"  <math>    <mrow>
    <mi>α</mi>
    <mo>‖</mo>
    <mrow>
    <msub>
    <mo>∇</mo>
    <mi>Δ</mi>
    </msub>
    <msubsup>
    <mi>η</mi>
    <mi>Δ</mi>
    <mi>ν</mi>
    </msubsup>
    </mrow>
    <mo>‖</mo>
    </mrow>
    </math>"#;
    test("el", "ClearSpeak", expr, "άλφα; νόρμα του ανάδελτα δείκτης, κεφαλαίο δέλτα; ήτα δείκτης, κεφαλαίο δέλτα, στην νί οστή δύναμη")?;
    return Ok(());
    //theodora. fails with current rules. should be fine when adding genitive clause
   // Now reads: άλφα; νόρμα του το ανάδελτα δείκτης, κεφαλαίο δέλτα; ήτα δείκτης, κεφαλαίο δέλτα, στην νί οστή δύναμη
}
#[test]
fn twentythree() -> Result<()> {
    let expr = r#"  <math>    <mrow>
    <msqrt>
    <mrow>
    <mn>1</mn>
    <mo>+</mo>
    
    <msqrt>
    <mrow>
    <mn>2</mn>
    <mo>+</mo>
    
    <msqrt>
    <mrow>
    <mn>2</mn>
    <mo>+</mo>
    
    <msqrt>
    <mrow>
    <mn>2</mn>
    <mo>+</mo>
    <mo>…</mo>
    </mrow>
    </msqrt>
    
    </mrow>
    </msqrt>
    
    </mrow>
    </msqrt>
    
    </mrow>
    </msqrt>
    </mrow>
    </math>"#;
    test("el", "ClearSpeak", expr, "η τετραγωνική ρίζα του 1 συν; η τετραγωνική ρίζα του 2 συν; η τετραγωνική ρίζα του 2 συν; η τετραγωνική ρίζα του 2 συν αποσιωπητικά")?;
    return Ok(());
    //theodora. fails. Now reads: η τετραγωνική ρίζα του 1 συν; η τετραγωνική ρίζα του 2 συν; η τετραγωνική ρίζα του 2 συν; η τετραγωνική ρίζα του 2 συν αποσιωπητικά
}
#[test]
fn twentyfour() -> Result<()> {
    let expr = r#"<math>    <mrow>
    <mi>tan</mi>
    <mo stretchy="false">(</mo>
    <mn>22.5</mn>
    <mo>°</mo>
    <mo stretchy="false">)</mo>
    <mo>=</mo>
    
    <mfrac>
    <mrow>
    <msqrt>
    <mrow>
    <mn>2</mn>
    <mo>−</mo>
    <msqrt>
    <mn>2</mn>
    </msqrt>
    </mrow>
    </msqrt>
    </mrow>
    
    <mrow>
    <msqrt>
    <mrow>
    <mn>2</mn>
    <mo>+</mo>
    <msqrt>
    <mn>2</mn>
    </msqrt>
    </mrow>
    </msqrt>
    </mrow>
    </mfrac>
    
    <mo>=</mo>
    
    <msqrt>
    <mn>2</mn>
    </msqrt>
    
    <mo>−</mo>
    <mn>1</mn>
    </mrow>
    </math>"#;
    test("el", "ClearSpeak", expr, "εφαπτομένη του; ανοίγει παρένθεση, 22.5 μοίρες; κλείνει παρένθεση; ισούται με; το κλάσμα με αριθμητή; η τετραγωνική ρίζα του 2 μείον, η τετραγωνική ρίζα του 2; και παρονομαστή η τετραγωνική ρίζα του 2 συν, η τετραγωνική ρίζα του 2; ισούται με; την τετραγωνική ρίζα του 2; μείον 1")?;
    return Ok(());
    //theodora. 22.5 --> 225 also fails accusative rule
}
#[test]
fn twentyfive() -> Result<()> {
    let expr = r#"  <math>    <mrow>
    
    <mfenced open="(" close=")">
    <mtable>
    <mtr>
    <mtd>
    <msup>
    <mi>Σ</mi>
    <mrow>
    <mn>1,1</mn>
    </mrow>
    </msup>
    </mtd>
    
    <mtd>
    <msup>
    <mi>Σ</mi>
    <mrow>
    <mn>1,2</mn>
    </mrow>
    </msup>
    </mtd>
    </mtr>
    
    <mtr>
    <mtd>
    <msup>
    <mi>Σ</mi>
    <mrow>
    <mn>2,1</mn>
    </mrow>
    </msup>
    </mtd>
    
    <mtd>
    <msup>
    <mi>Σ</mi>
    <mrow>
    <mn>2,2</mn>
    </mrow>
    </msup>
    </mtd>
    </mtr>
    </mtable>
    </mfenced>
    
    <mo>=</mo>
    
    <msup>
    <mfenced open="(" close=")">
    <mtable>
    <mtr>
    <mtd>
    <msub>
    <mi>Σ</mi>
    <mrow>
    <mn>1,1</mn>
    </mrow>
    </msub>
    </mtd>
    
    <mtd>
    <msub>
    <mi>Σ</mi>
    <mrow>
    <mn>1,2</mn>
    </mrow>
    </msub>
    </mtd>
    </mtr>
    
    <mtr>
    <mtd>
    <msub>
    <mi>Σ</mi>
    <mrow>
    <mn>2,1</mn>
    </mrow>
    </msub>
    </mtd>
    
    <mtd>
    <msub>
    <mi>Σ</mi>
    <mrow>
    <mn>2,2</mn>
    </mrow>
    </msub>
    </mtd>
    </mtr>
    </mtable>
    </mfenced>
    
    <mrow>
    <mo>−</mo>
    <mn>1</mn>
    </mrow>
    </msup>
    
    </mrow>
    </math>"#;
    test("el", "ClearSpeak", expr, "2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα στην 1,1 δύναμη, στήλη 2; κεφαλαίο σίγμα στην 1,2 δύναμη; γραμμή 2; στήλη 1; κεφαλαίο σίγμα στην 2,1 δύναμη, στήλη 2; κεφαλαίο σίγμα στην 2,2 δύναμη; ισούται με; 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα δείκτης 2,1; στήλη 2; κεφαλαίο σίγμα δείκτης 2,2; στην μείον 1 δύναμη")?;
    return Ok(());
    //theodora. fails. Now read: 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα στην 1,1 δύναμη, στήλη 2; κεφαλαίο σίγμα στην 1,2 δύναμη; γραμμή 2; στήλη 1; κεφαλαίο σίγμα στην 2,1 δύναμη, στήλη 2; κεφαλαίο σίγμα στην 2,2 δύναμη; ισούται με; TEMP NAME του 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα δείκτης 2,1; στήλη 2; κεφαλαίο σίγμα δείκτης 2,2; στην μείον 1 δύναμη
}
#[test]
fn twentysix() -> Result<()> {
    let expr = r#"  <math>    <mrow>
    
    <mstyle displaystyle="true">
    <munder>
    <mo>∑</mo>
    
    <mrow>
    <mi>κ</mi>
    <mo>∈</mo>
    <mi>ℝ</mi>
    </mrow>
    </munder>
    
    <mrow>
    <msub>
    <mi>α</mi>
    <mi>κ</mi>
    </msub>
    </mrow>
    </mstyle>
    
    </mrow>
    </math>"#;
    test("el", "ClearSpeak", expr, "άθροισμα για καπα ανήκει, στους πραγματικούς αριθμούς του; άλφα δείκτης καπα")?;
    return Ok(());
    
}

// Some rules for Greek nominative, accusative, genitive
#[test]
fn test_sup_set_with_curly_braces_genitive() -> Result<()> {
    let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mi>sup</mi><mo>{</mo><mn>1</mn><mo>,</mo><mn>2</mn><mo>,</mo><mn>3</mn><mo>}</mo></math>";
    test("el", "ClearSpeak", expr, "σουπρέμουμ του, συνόλου 1 κόμμα, 2 κόμμα, 3")?;
    return Ok(());
}

#[test]
fn test_log_after_operator_accusative() -> Result<()> {
    let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mfrac><mn>1</mn><mi>y</mi></mfrac><mo>=</mo><mi>log</mi><mfenced><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow></mfenced></math>";
    test("el", "ClearSpeak", expr, "x στο τετράγωνο, συν 1 προς y; ισούται με; το λογάριθμο, του; ανοίγει παρένθεση, x συν 1, κλείνει παρένθεση")?;
    return Ok(());
}
#[test]
fn test_ln_after_operator_accusative() -> Result<()> {
    let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mfrac><mn>1</mn><mi>y</mi></mfrac><mo>=</mo><mi>ln</mi><mfenced><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow></mfenced></math>";
    test("el", "ClearSpeak", expr, "x στο τετράγωνο, συν 1 προς y; ισούται με; το l n του; ανοίγει παρένθεση, x συν 1, κλείνει παρένθεση")?;
    return Ok(());
}
#[test]
fn test_root_after_operator_accusative() -> Result<()> {
    let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mfrac><mn>1</mn><mi>y</mi></mfrac><mo>&#x2264;</mo><msqrt><mn>2</mn></msqrt></math>";
    test("el", "ClearSpeak", expr, "x στο τετράγωνο, συν 1 προς y; είναι μικρότερο από ή ίσο με; την τετραγωνική ρίζα του 2")?;
    return Ok(());
}
#[test]
fn test_root_after_operator_frac_nominative() -> Result<()> {
    let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mfrac><mn>1</mn><mi>y</mi></mfrac><mo>=</mo><mfrac><msqrt><mn>2</mn></msqrt><mn>3</mn></mfrac></math>";
    test("el", "ClearSpeak", expr, "x στο τετράγωνο, συν 1 προς y; ισούται με; το κλάσμα με αριθμητή; η τετραγωνική ρίζα του 2; και παρονομαστή 3")?;
    return Ok(());
}

#[test]
fn test_root_after_operator_frac_accusative2() -> Result<()> {
    let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mfrac><mn>1</mn><mi>y</mi></mfrac><mo>=</mo><msqrt><mn>2</mn></msqrt></math>";
    test("el", "ClearSpeak", expr, "x στο τετράγωνο, συν 1 προς y; ισούται με, την τετραγωνική ρίζα του 2")?;
    return Ok(());
}
#[test]
fn test_root_after_operator_nominative2() -> Result<()> {
    let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mfrac><mn>1</mn><mi>y</mi></mfrac><mo>=</mo><mn>5</mn><msqrt><mn>2</mn></msqrt></math>";
    test("el", "ClearSpeak", expr, "x στο τετράγωνο, συν 1 προς y; ισούται με; 5, η τετραγωνική ρίζα του 2")?;
    return Ok(());
}
#[test]
fn test_log_after_operator_frac_nominative() -> Result<()> {
    let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><msup><mi>x</mi><mn>2</mn></msup><mo>+</mo><mfrac><mn>1</mn><mi>y</mi></mfrac><mo>=</mo><mfrac><mrow><mi>log</mi><mfenced><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow></mfenced><mo>+</mo><mi>log</mi><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow><mn>2</mn></mfrac></math>";
    test("el", "ClearSpeak", expr, "x στο τετράγωνο, συν 1 προς y; ισούται με; το κλάσμα με αριθμητή; ο λογάριθμος, του; ανοίγει παρένθεση, x συν 1, κλείνει παρένθεση; συν; ο λογάριθμος, του; ανοίγει παρένθεση, x συν 1, κλείνει παρένθεση; και παρονομαστή 2")?;
    return Ok(());
}
#[test]
fn test_abs_value_after_operator_accusative() -> Result<()> {
    let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mrow><mo>|</mo><mo>&#x2207;</mo><mi>&#x397;</mi><mrow><mo>(</mo><mi>&#x3C4;</mi><mo>,</mo><mi>&#x3C7;</mi><mo>)</mo></mrow><mo>|</mo></mrow><mo>&#x2264;</mo><mrow><mo>|</mo><mi>&#x3C7;</mi><mo>|</mo></mrow><mo>+</mo><mi>&#x3B3;</mi><mrow><mo>(</mo><mi>&#x3C4;</mi><mo>)</mo></mrow></math>";
    test("el", "ClearSpeak", expr, "απόλυτη τιμή του ανάδελτα του κεφαλαίο ήτα; ανοίγει παρένθεση; τάαφ κόμμα, χ; κλείνει παρένθεση; είναι μικρότερο από ή ίσο με; απόλυτη τιμή του χ; συν, γάμμα του τάαφ")?;
    return Ok(());
}
#[test]
fn test_abs_value_after_operator_nominative() -> Result<()> {
    let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mrow><mo>|</mo><mo>&#x2207;</mo><mi>&#x397;</mi><mrow><mo>(</mo><mi>&#x3C4;</mi><mo>,</mo><mi>&#x3C7;</mi><mo>)</mo></mrow><mo>|</mo></mrow><mo>&#x2264;</mo><mfrac><mrow><mo>|</mo><mi>&#x3C7;</mi><mo>|</mo></mrow><mn>2</mn></mfrac><mo>+</mo><mi>&#x3B3;</mi><mrow><mo>(</mo><mi>&#x3C4;</mi><mo>)</mo></mrow></math>";
    test("el", "ClearSpeak", expr, "απόλυτη τιμή του ανάδελτα του κεφαλαίο ήτα; ανοίγει παρένθεση; τάαφ κόμμα, χ; κλείνει παρένθεση; είναι μικρότερο από ή ίσο με; το κλάσμα με αριθμητή; απόλυτη τιμή του χ; και παρονομαστή 2; συν, γάμμα του τάαφ")?;
    return Ok(());
}

#[test]
fn test_matrix_after_operator_accusative() -> Result<()> {
    let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mi>&#x3B1;</mi><mo>+</mo><mi>&#x3B2;</mi><mo>=</mo><mrow><mo>(</mo><mtable><mtr><mtd><msub><mi>&#x3A3;</mi><mn>1,1</mn></msub></mtd><mtd><msub><mi>&#x3A3;</mi><mn>1,2</mn></msub></mtd></mtr><mtr><mtd><msub><mi>&#x3A3;</mi><mn>2,1</mn></msub></mtd><mtd><msub><mi>&#x3A3;</mi><mn>2,2</mn></msub></mtd></mtr></mtable><mo>)</mo></mrow></math>";
    test("el", "ClearSpeak", expr, "άλφα συν βήτα, ισούται με; 2 επί 2 πίνακα; γραμμή 1; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα δείκτης 2,1; στήλη 2; κεφαλαίο σίγμα δείκτης 2,2")?;
    return Ok(());
}
#[test]
fn test_matrix_after_operator_nominative() -> Result<()> {
    let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mi>&#x3B1;</mi><mo>+</mo><mi>&#x3B2;</mi><mo>=</mo><mn>2</mn><mrow><mo>(</mo><mtable><mtr><mtd><msub><mi>&#x3A3;</mi><mn>1,1</mn></msub></mtd><mtd><msub><mi>&#x3A3;</mi><mn>1,2</mn></msub></mtd></mtr><mtr><mtd><msub><mi>&#x3A3;</mi><mn>2,1</mn></msub></mtd><mtd><msub><mi>&#x3A3;</mi><mn>2,2</mn></msub></mtd></mtr></mtable><mo>)</mo></mrow></math>";
    test("el", "ClearSpeak", expr, "άλφα συν βήτα, ισούται με; 2; 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα δείκτης 2,1; στήλη 2; κεφαλαίο σίγμα δείκτης 2,2")?;
    return Ok(());
    //theodora. fails due to unrelated to accusative reasons. Now reads: άλφα συν βήτα, ισούται με; 2; 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα δείκτης 2,1; στήλη 2; κεφαλαίο σίγμα δείκτης 2,2
}
// #[test]
// fn test_matrix_after_operator_nominative2() -> Result<()> {
//     let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mi>&#x3B1;</mi><mo>+</mo><mi>&#x3B2;</mi><mo>=</mo><mfrac><mrow><mo>(</mo><mtable><mtr><mtd><msub><mi>&#x3A3;</mi><mn>1,1</mn></msub></mtd><mtd><msub><mi>&#x3A3;</mi><mn>1,2</mn></msub></mtd></mtr><mtr><mtd><msub><mi>&#x3A3;</mi><mn>2,1</mn></msub></mtd><mtd><msub><mi>&#x3A3;</mi><mn>2,2</mn></msub></mtd></mtr></mtable><mo>)</mo></mrow><mi>x</mi></mfrac></math>";
//     test("el", "ClearSpeak", expr, "άλφα συν βήτα, ισούται με; το κλάσμα με αριθμητή; ο 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα δείκτης 2,1; στήλη 2; κεφαλαίο σίγμα δείκτης 2,2; και παρονομαστή x")?;
//     return Ok(());
//     //theodora. fails due to unrelated to accusative reasons. Now reads: άλφα συν βήτα, ισούται με; το κλάσμα με αριθμητή; TEMP NAME του 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα δείκτης 2,1; στήλη 2; κεφαλαίο σίγμα δείκτης 2,2; και παρονομαστή x
// }
#[test]
fn matrix_nominative() -> Result<()> { 
    let expr = r#"<math>
 <mrow>
   <mrow><mo>(</mo>
    <mrow>
     <mtable>
      <mtr>
       <mtd>
        <mrow>
         <msub>
          <mi>Υ</mi>
          <mn>1</mn>
         </msub>
         </mrow>
       </mtd>
      </mtr>
      <mtr>
       <mtd>
        <mrow>
         <msub>
          <mi>Υ</mi>
          <mn>2</mn>
         </msub>
         </mrow>
       </mtd>
      </mtr>
      
     </mtable></mrow>
   <mo>)</mo></mrow><mo>∼</mo><mrow><mo>[</mo> <mrow>
    <mrow><mo>(</mo>
     <mrow>
      <mtable>
       <mtr>
        <mtd>
         <mi>Χ</mi>
        </mtd>
        <mtd>
         <mn>0</mn>
        </mtd>
       </mtr>
       <mtr>
        <mtd>
         <mn>0</mn>
        </mtd>
        <mtd>
         <mi>Χ</mi>
        </mtd>
       </mtr>
       
      </mtable></mrow>
    <mo>)</mo></mrow><mrow><mo>(</mo>
     <mrow>
      <mtable>
       <mtr>
        <mtd>
         <mrow>
          <msub>
           <mi>β</mi>
           <mn>1</mn>
          </msub>
          </mrow>
        </mtd>
       </mtr>
       <mtr>
        <mtd>
         <mrow>
          <msub>
           <mi>β</mi>
           <mn>2</mn>
          </msub>
          </mrow>
        </mtd>
       </mtr>
       
      </mtable></mrow>
    <mo>)</mo></mrow><mo>,</mo><mrow><mo>(</mo>
     <mrow>
      <mtable>
       <mtr>
        <mtd>
         <mrow>
          <msub>
           <mi>Σ</mi>
           <mrow>
            <mn>1,1</mn></mrow>
          </msub>
          </mrow>
        </mtd>
        <mtd>
         <mrow>
          <msub>
           <mi>Σ</mi>
           <mrow>
            <mn>1,2</mn></mrow>
          </msub>
          </mrow>
        </mtd>
       </mtr>
       <mtr>
        <mtd>
         <mrow>
          <msub>
           <mi>Σ</mi>
           <mrow>
            <mn>2,1</mn></mrow>
          </msub>
          </mrow>
        </mtd>
        <mtd>
         <mrow>
          <msub>
           <mi>Σ</mi>
           <mrow>
            <mn>2,2</mn></mrow>
          </msub>
          </mrow>
        </mtd>
       </mtr>
       
      </mtable></mrow>
    <mo>)</mo></mrow></mrow> <mo>]</mo></mrow></mrow>
  </math>"#;
    test("el", "ClearSpeak", expr, "2 επί ένα πίνακας-στήλη; γραμμή 1; κεφαλαίο ύψιλον δείκτης 1; γραμμή 2; κεφαλαίο ύψιλον δείκτης 2; κυματοειδής γραμμή; ανοίγει αγκύλη; ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο χ; στήλη 2; κεφαλαίο χ; 2 επί ένα πίνακας-στήλη; γραμμή 1; βήτα δείκτης 1; γραμμή 2; βήτα δείκτης 2; κόμμα; 2 επί 2 πίνακας; γραμμή 1; στήλη 1; κεφαλαίο σίγμα δείκτης 1,1; στήλη 2; κεφαλαίο σίγμα δείκτης 1,2; γραμμή 2; στήλη 1; κεφαλαίο σίγμα δείκτης 2,1; στήλη 2; κεφαλαίο σίγμα δείκτης 2,2; κλείνει αγκύλη")?;
    return Ok(());
}
#[test]
fn diagonal_matrix_accusative() -> Result<()> { 
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mrow><mo>(</mo><mtable><mtr><mtd><msub><mi>&#x3A5;</mi><mn>1</mn></msub></mtd></mtr><mtr><mtd><msub><mi>&#x3A5;</mi><mn>2</mn></msub></mtd></mtr></mtable><mo>)</mo></mrow><mo>=</mo><mo>(</mo><mtable><mtr><mtd><mi>&#x3A7;</mi></mtd><mtd><mn>0</mn></mtd></mtr><mtr><mtd><mn>0</mn></mtd><mtd><mi>&#x3A7;</mi></mtd></mtr></mtable><mo>)</mo></math>"#;
    test("el", "ClearSpeak", expr, "2 επί ένα πίνακας-στήλη; γραμμή 1; κεφαλαίο ύψιλον δείκτης 1; γραμμή 2; κεφαλαίο ύψιλον δείκτης 2; ισούται με; τον 2 επί 2 διαγώνιο πίνακα; στήλη 1; κεφαλαίο χ; στήλη 2; κεφαλαίο χ")?;
    return Ok(());
}
#[test]
fn diagonal_matrix_accusative_2() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mi>&#x391;</mi><mo>=</mo><mo>[</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr><mtr><mtd><mn>0</mn></mtd><mtd><mn>2</mn></mtd></mtr></mtable><mo>]</mo></math>";
  test("el", "SimpleSpeak", expr,
    "κεφαλαίο άλφα, ισούται με; τον 2 επί 2 διαγώνιο πίνακα; στήλη 1; 1; στήλη 2; 2")?;
    return Ok(());
  }

#[test]
fn diagonal_matrix_nominative() -> Result<()> { 
    let expr = r#"<math xmlns='http://www.w3.org/1998/Math/MathML'><mrow><mo>(</mo><mtable><mtr><mtd><msub><mi>&#x3A5;</mi><mn>1</mn></msub></mtd></mtr><mtr><mtd><msub><mi>&#x3A5;</mi><mn>2</mn></msub></mtd></mtr></mtable><mo>)</mo></mrow><mo>=</mo><mn>2</mn><mo>(</mo><mtable><mtr><mtd><mi>&#x3A7;</mi></mtd><mtd><mn>0</mn></mtd></mtr><mtr><mtd><mn>0</mn></mtd><mtd><mi>&#x3A7;</mi></mtd></mtr></mtable><mo>)</mo></math>"#;
    test("el", "ClearSpeak", expr, "2 επί ένα πίνακας-στήλη; γραμμή 1; κεφαλαίο ύψιλον δείκτης 1; γραμμή 2; κεφαλαίο ύψιλον δείκτης 2; ισούται με; 2; ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο χ; στήλη 2; κεφαλαίο χ")?;
    return Ok(());
}
// #[test]
// fn diagonal_matrix_frac_nominative() -> Result<()> { 
//     let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mrow><mo>(</mo><mtable><mtr><mtd><msub><mi>&#x3A5;</mi><mn>1</mn></msub></mtd></mtr><mtr><mtd><msub><mi>&#x3A5;</mi><mn>2</mn></msub></mtd></mtr></mtable><mo>)</mo></mrow><mo>=</mo><mfrac><mrow><mo>(</mo><mtable><mtr><mtd><mi>&#x3A7;</mi></mtd><mtd><mn>0</mn></mtd></mtr><mtr><mtd><mn>0</mn></mtd><mtd><mi>&#x3A7;</mi></mtd></mtr></mtable><mo>)</mo></mrow><mn>5</mn></mfrac></math>"#;
//     test("el", "ClearSpeak", expr, "2 επί ένα πίνακας-στήλη; γραμμή 1; κεφαλαίο ύψιλον δείκτης 1; γραμμή 2; κεφαλαίο ύψιλον δείκτης 2; ισούται με; το κλάσμα με αριθμητή; ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο χ; στήλη 2; κεφαλαίο χ; και παρονομαστή 5")?;
//     return Ok(());
//    //theodora. fails due to unrelated issues to accusative/nominative. Now reads: 2 επί ένα πίνακας-στήλη; γραμμή 1; κεφαλαίο ύψιλον δείκτης 1; γραμμή 2; κεφαλαίο ύψιλον δείκτης 2; ισούται με; το κλάσμα με αριθμητή; TEMP NAME του ο 2 επί 2 διαγώνιος πίνακας; στήλη 1; κεφαλαίο χ; στήλη 2; κεφαλαίο χ; και παρονομαστή 5
// }
#[test]
fn matrix_default_accusative() -> Result<()> { 
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mi>&#x3C7;</mi><mo>=</mo><mtable columnalign="left"><mtr><mtd><mfrac><mo>&#x2202;</mo><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mrow><mo>(</mo><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mo>)</mo></mrow><mo>&#x2212;</mo><mi>&#x394;</mi><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mo>+</mo><msup><mi>&#x3C6;</mi><mo>'</mo></msup><mrow><mo>(</mo><mi>&#x3C5;</mi><mo>)</mo></mrow><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac></mtd></mtr><mtr><mtd><mo>=</mo><mo>&#x2212;</mo><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3B1;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mo>+</mo><mi>&#x394;</mi><mi>&#x3B1;</mi><mo>&#x2212;</mo><mi>&#x3C5;</mi><mo>&#x2212;</mo><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac></mtd></mtr></mtable></math>"#;
    test("el", "ClearSpeak", expr, "χ ισούται με; πίνακα με 2 γραμμές και 1 στήλες; γραμμή 1; στήλη 1; το κλάσμα με αριθμητή; μερικό διαφορικό; και παρονομαστή μερικό διαφορικό, τάαφ; ανοίγει παρένθεση; το κλάσμα με αριθμητή; μερικό διαφορικό, ύψιλον; και παρονομαστή μερικό διαφορικό, τάαφ; κλείνει παρένθεση; μείον; κεφαλαίο δέλτα; το κλάσμα με αριθμητή; μερικό διαφορικό, ύψιλον; και παρονομαστή μερικό διαφορικό, τάαφ; συν; φ τόνος, του ύψιλον; το κλάσμα με αριθμητή; μερικό διαφορικό, ύψιλον; και παρονομαστή μερικό διαφορικό, τάαφ; γραμμή 2; στήλη 1; ισούται με; μείον το κλάσμα με αριθμητή; μερικό διαφορικό, άλφα; και παρονομαστή μερικό διαφορικό, τάαφ; συν, κεφαλαίο δέλτα, άλφα; μείον ύψιλον μείον; το κλάσμα με αριθμητή; μερικό διαφορικό, ύψιλον; και παρονομαστή μερικό διαφορικό, τάαφ")?;
    return Ok(());
}
#[test]
fn matrix_default_nominative() -> Result<()> { 
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mi>&#x3C7;</mi><mo>=</mo><mn>2</mn><mtable columnalign="left"><mtr><mtd><mfrac><mo>&#x2202;</mo><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mrow><mo>(</mo><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mo>)</mo></mrow><mo>&#x2212;</mo><mi>&#x394;</mi><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mo>+</mo><msup><mi>&#x3C6;</mi><mo>'</mo></msup><mrow><mo>(</mo><mi>&#x3C5;</mi><mo>)</mo></mrow><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac></mtd></mtr><mtr><mtd><mo>=</mo><mo>&#x2212;</mo><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3B1;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mo>+</mo><mi>&#x394;</mi><mi>&#x3B1;</mi><mo>&#x2212;</mo><mi>&#x3C5;</mi><mo>&#x2212;</mo><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac></mtd></mtr></mtable></math>"#;
    test("el", "ClearSpeak", expr, "χ ισούται με; 2; πίνακας με 2 γραμμές και 1 στήλες; γραμμή 1; στήλη 1; το κλάσμα με αριθμητή; μερικό διαφορικό; και παρονομαστή μερικό διαφορικό, τάαφ; ανοίγει παρένθεση; το κλάσμα με αριθμητή; μερικό διαφορικό, ύψιλον; και παρονομαστή μερικό διαφορικό, τάαφ; κλείνει παρένθεση; μείον; κεφαλαίο δέλτα; το κλάσμα με αριθμητή; μερικό διαφορικό, ύψιλον; και παρονομαστή μερικό διαφορικό, τάαφ; συν; φ τόνος, του ύψιλον; το κλάσμα με αριθμητή; μερικό διαφορικό, ύψιλον; και παρονομαστή μερικό διαφορικό, τάαφ; γραμμή 2; στήλη 1; ισούται με; μείον το κλάσμα με αριθμητή; μερικό διαφορικό, άλφα; και παρονομαστή μερικό διαφορικό, τάαφ; συν, κεφαλαίο δέλτα, άλφα; μείον ύψιλον μείον; το κλάσμα με αριθμητή; μερικό διαφορικό, ύψιλον; και παρονομαστή μερικό διαφορικό, τάαφ")?;
    return Ok(());
}
#[test]
fn matrix_default_without_accusative2() -> Result<()> { 
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><mi>&#x3C7;</mi><mo>=</mo><mfrac><mtable columnalign="left"><mtr><mtd><mfrac><mo>&#x2202;</mo><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mrow><mo>(</mo><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mo>)</mo></mrow><mo>&#x2212;</mo><mi>&#x394;</mi><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mo>+</mo><msup><mi>&#x3C6;</mi><mo>'</mo></msup><mrow><mo>(</mo><mi>&#x3C5;</mi><mo>)</mo></mrow><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac></mtd></mtr><mtr><mtd><mo>=</mo><mo>&#x2212;</mo><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3B1;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac><mo>+</mo><mi>&#x394;</mi><mi>&#x3B1;</mi><mo>&#x2212;</mo><mi>&#x3C5;</mi><mo>&#x2212;</mo><mfrac><mrow><mo>&#x2202;</mo><mi>&#x3C5;</mi></mrow><mrow><mo>&#x2202;</mo><mi>&#x3C4;</mi></mrow></mfrac></mtd></mtr></mtable><mn>4</mn></mfrac></math>"#;
    test("el", "ClearSpeak", expr, "χ ισούται με; το κλάσμα με αριθμητή; πίνακας με 2 γραμμές και 1 στήλες; γραμμή 1; στήλη 1; το κλάσμα με αριθμητή; μερικό διαφορικό; και παρονομαστή μερικό διαφορικό, τάαφ; ανοίγει παρένθεση; το κλάσμα με αριθμητή; μερικό διαφορικό, ύψιλον; και παρονομαστή μερικό διαφορικό, τάαφ; κλείνει παρένθεση; μείον; κεφαλαίο δέλτα; το κλάσμα με αριθμητή; μερικό διαφορικό, ύψιλον; και παρονομαστή μερικό διαφορικό, τάαφ; συν; φ τόνος, του ύψιλον; το κλάσμα με αριθμητή; μερικό διαφορικό, ύψιλον; και παρονομαστή μερικό διαφορικό, τάαφ; γραμμή 2; στήλη 1; ισούται με; μείον το κλάσμα με αριθμητή; μερικό διαφορικό, άλφα; και παρονομαστή μερικό διαφορικό, τάαφ; συν, κεφαλαίο δέλτα, άλφα; μείον ύψιλον μείον; το κλάσμα με αριθμητή; μερικό διαφορικό, ύψιλον; και παρονομαστή μερικό διαφορικό, τάαφ; και παρονομαστή 4")?;
    return Ok(());
}

#[test]
fn max_munder() -> Result<()> { //
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><msub><mover accent="true"><mi>&#x392;</mi><mo>~</mo></mover><mn>1</mn></msub><mrow><mo>(</mo><mi>&#x3A4;</mi><mo>)</mo></mrow><mo>=</mo><munder><mi>max</mi><mrow><mi>&#x3B9;</mi><mo>=</mo><mn>1,</mn><mo>&#x2026;</mo><mi>&#x3BD;</mi></mrow></munder><munder><mo>&#x222B;</mo><msub><mi>&#x393;</mi><mi>&#x3BE;</mi></msub></munder><mrow><mo>|</mo><msub><mi>&#x3C5;</mi><mi>&#x3B9;</mi></msub><mo>|</mo></mrow><mi>d</mi><mi>&#x3C4;</mi></math>"#;
    test("el", "ClearSpeak", expr, "κεφαλαίο βήτα κυματοειδής γραμμή, δείκτης 1; του κεφαλαίο τάαφ; ισούται με; μέγιστο για ιότα ισούται με; 1, αποσιωπητικά, νί; του; ολοκλήρωμα επί του συνόλου κεφαλαίο γάμμα δείκτης ξ, του; απόλυτη τιμή του ύψιλον δείκτης ιότα; d τάαφ")?;
    return Ok(());
    
  }
  #[test]
fn union_real_numbers_munder() -> Result<()> { //
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><munder><mo>&#x222A;</mo><mi mathvariant="normal">&#x211D;</mi></munder><mi>a</mi><mo>+</mo><mi>b</mi></math>"#;
    test("el", "ClearSpeak", expr, "ένωση για τους πραγματικούς αριθμούς; a συν b")?;
    return Ok(());

  }
#[test]
fn intersection_munder() -> Result<()> { //
    let expr = r#"<math xmlns="http://www.w3.org/1998/Math/MathML"><munder><mo>&#x2229;</mo><mrow><mi>a</mi><mo>&#x2208;</mo><mi mathvariant="normal">&#x211D;</mi></mrow></munder><mi>a</mi><mo>+</mo><mi>b</mi></math>"#;
    test("el", "ClearSpeak", expr, "τομή για a ανήκει, στους πραγματικούς αριθμούς; a συν b")?;
    return Ok(());
  }
#[test]
fn zero_matrix_accusative() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mi>&#x391;</mi><mo>=</mo><mo>[</mo><mtable><mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr><mtr><mtd><mn>0</mn></mtd><mtd><mn>0</mn></mtd></mtr></mtable><mo>]</mo></math>";
  test("el", "SimpleSpeak", expr,
    "κεφαλαίο άλφα, ισούται με, τον 2 επί 2 μηδενικό πίνακα")?;
    return Ok(());
  }

#[test]
fn identity_matrix_accusative() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mi>&#x391;</mi><mo>=</mo><mo>[</mo><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>0</mn></mtd></mtr><mtr><mtd><mn>0</mn></mtd><mtd><mn>1</mn></mtd></mtr></mtable><mo>]</mo></math>";
  test("el", "SimpleSpeak", expr,
    "κεφαλαίο άλφα, ισούται με, τον 2 επί 2 μοναδιαίο πίνακα")?;
    return Ok(());
  }

#[test]
fn column_2_by_1_matrix_accusative() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mi>&#x391;</mi><mo>=</mo><mfenced><mtable><mtr><mtd><mn>1</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr></mtable></mfenced></math>";
  test("el", "SimpleSpeak", expr,
    "κεφαλαίο άλφα, ισούται με; 2 επί ένα πίνακα-στήλη; 1; 2")?;
    return Ok(());
  }
#[test]
fn row_matrix_accusative() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mi>&#x391;</mi><mo>=</mo><mfenced><mtable><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr></mtable></mfenced></math>";
  test("el", "SimpleSpeak", expr,
    "κεφαλαίο άλφα, ισούται με; ένα επί 2 πίνακα-γραμμή; 1, 2")?;
    return Ok(());
  }
#[test]
fn column_3_by_1_matrix_accusative() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mi>&#x391;</mi><mo>=</mo><mfenced><mtable><mtr><mtd><mn>1</mn></mtd></mtr><mtr><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd></mtr></mtable></mfenced></math>";
  test("el", "SimpleSpeak", expr,
    "κεφαλαίο άλφα, ισούται με; 3 επί ένα πίνακα-στήλη; 1; 2; 3")?;
    return Ok(());
  }
#[test]
fn log_with_base_accusative() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mi>x</mi><mo>=</mo><msub><mi>log</mi><mi>&#x3B2;</mi></msub><mfenced><mi>&#x3B1;</mi></mfenced></math>";
  test("el", "SimpleSpeak", expr,
    "x ισούται με; το λογάριθμο με βάση βήτα; του άλφα")?;
    return Ok(());
  }
  // theodora. fails Now reads: x ισούται με; ο λογάριθμος με βάση βήτα; του άλφα
  //the old log with base and log base power don't work so we can't introduce rules for accisative like plain log or ln

#[test]
fn minus_root_after_operator_accusative() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mi>x</mi><mo>=</mo><mo>+</mo><msqrt><mi>y</mi></msqrt></math>";
  test("el", "SimpleSpeak", expr,
    "x ισούται με; συν την τετραγωνική ρίζα του y")?;
    return Ok(());
  }
  //theodora. fails: Now reads: x ισούται με, συν η τετραγωνική ρίζα του y
  //the accusative rule where there is a -+ before the root fails. 

#[test]
fn abs_value_after_operator_accusative() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mi>y</mi><mo>=</mo><mo>|</mo><mi>x</mi><mo>|</mo></math>";
  test("el", "SimpleSpeak", expr,
    "y ισούται με; την απόλυτη τιμή του x")?;
    return Ok(());
    //theodora. fails. can't introduce accusative rules for abs value
}
#[test]
fn square_root_with_fraction_genitive_clearspeak() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><msqrt><mfrac><mrow><mi>x</mi><mo>+</mo><mn>3</mn></mrow><mi>y</mi></mfrac></msqrt></math>";
  test("el", "ClearSpeak", expr,
    "η τετραγωνική ρίζα του κλάσματος με αριθμητή x συν 3; και παρονομαστή y")?;
    return Ok(());
}
#[test]
fn square_root_with_fraction_genitive_simplespeak() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><msqrt><mfrac><mrow><mi>x</mi><mo>+</mo><mn>3</mn></mrow><mi>y</mi></mfrac></msqrt></math>";
  test("el", "SimpleSpeak", expr,
    "η τετραγωνική ρίζα του κλάσματος, x συν 3, προς y, τέλος κλάσματος; τέλος ρίζας")?;
    return Ok(());
}
#[test]
fn integral_with_fraction_genitive_clearspeak() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mo>&#x222B;</mo><mfrac><mrow><mi>x</mi><mo>+</mo><mn>3</mn></mrow><mi>y</mi></mfrac></math>";
  test("el", "ClearSpeak", expr,
    "ολοκλήρωμα του κλάσματος με αριθμητή x συν 3; και παρονομαστή y")?;
    return Ok(());
}
#[test]
fn integral_with_underover_with_fraction_genitive_clearspeak() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><msubsup><mo>&#x222B;</mo><mi>&#x3B1;</mi><mi>&#x3B2;</mi></msubsup><mfrac><mrow><mi>x</mi><mo>+</mo><mn>3</mn></mrow><mi>y</mi></mfrac></math>";
  test("el", "ClearSpeak", expr,
    "ολοκλήρωμα από άλφα ως βήτα του; κλάσματος με αριθμητή x συν 3; και παρονομαστή y")?;
    return Ok(());
}
#[test]
fn integral_with_fraction_genitive_simplespeak() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mo>&#x222B;</mo><mfrac><mrow><mi>x</mi><mo>+</mo><mn>3</mn></mrow><mi>y</mi></mfrac></math>";
  test("el", "SimpleSpeak", expr,
    "ολοκλήρωμα του κλάσματος, x συν 3, προς y, τέλος κλάσματος")?;
    return Ok(());
}
#[test]
fn integral_with_underover_with_fraction_genitive_simplespeak() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><msubsup><mo>&#x222B;</mo><mi>&#x3B1;</mi><mi>&#x3B2;</mi></msubsup><mfrac><mrow><mi>x</mi><mo>+</mo><mn>3</mn></mrow><mi>y</mi></mfrac></math>";
  test("el", "SimpleSpeak", expr,
    "ολοκλήρωμα από άλφα ως βήτα του; κλάσματος, x συν 3, προς y, τέλος κλάσματος")?;
    return Ok(());
}
#[test]
fn sum_with_underover_with_fraction_genitive_clearspeak() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><munderover><mo>&#x2211;</mo><mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow><mo>&#x221E;</mo></munderover><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mn>3</mn></mfrac></math>";
  test("el", "ClearSpeak", expr,
    "άθροισμα από n ισούται με 1 ως άπειρο του; κλάσματος με αριθμητή x συν 1; και παρονομαστή 3")?;
    return Ok(());
}
#[test]
fn sum_with_underover_with_fraction_genitive_simplespeak() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><munderover><mo>&#x2211;</mo><mrow><mi>n</mi><mo>=</mo><mn>1</mn></mrow><mo>&#x221E;</mo></munderover><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mn>3</mn></mfrac></math>";
  test("el", "SimpleSpeak", expr,
    "άθροισμα από n ισούται με 1 ως άπειρο του; κλάσματος, x συν 1, προς 3, τέλος κλάσματος")?;
    return Ok(());
}
#[test]
fn absolute_value_with_fraction_genitive_clearspeak() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mo>|</mo><mfrac><mrow><mi>&#x3C7;</mi><mo>+</mo><mn>1</mn></mrow><mn>3</mn></mfrac><mo>|</mo></math>";
  test("el", "ClearSpeak", expr,
    "απόλυτη τιμή του κλάσματος με αριθμητή χ συν 1; και παρονομαστή 3")?;
    return Ok(());
}

#[test]
fn absolute_value_with_fraction_genitive_simplespeak() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><mo>|</mo><mfrac><mrow><mi>&#x3C7;</mi><mo>+</mo><mn>1</mn></mrow><mn>3</mn></mfrac><mo>|</mo></math>";
  test("el", "SimpleSpeak", expr,
    "η απόλυτη τιμή του κλάσματος, χ συν 1, προς 3, τέλος κλάσματος; τέλος απόλυτης τιμής")?;
    return Ok(());
}
//Δεν έχουν λυθεί
#[test]
fn lim_with_fraction_genitive_simplespeak() -> Result<()> {
  let expr = "<math xmlns='http://www.w3.org/1998/Math/MathML'><munder><mi>lim</mi><mrow><mi>x</mi><mo>&#x2192;</mo><mo>&#x221E;</mo></mrow></munder><mfrac><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow><mn>3</mn></mfrac></math>";
  test("el", "SimpleSpeak", expr,
    "το όριο όταν x προσεγγίζει, άπειρο; του; κλάσματος, χ συν 1, προς 3, τέλος κλάσματος; τέλος απόλυτης τιμής")?;
    return Ok(());
}