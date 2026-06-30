/// Tests for rules shared between various speech styles:
/// *  modified var
use crate::common::*;
use anyhow::Result;

// The basic layout of the tests is:
// 1. Sweep through all the SI prefixes
// 2. Sweep through each group of SI units
//    a) with both singular and plural without prefixes
//    b) with both singular and plural with one prefix
// 3. Sweep through each group of units that don't take SI prefixes
// These are broken into chunks so it is easier to see errors, when there are errors

#[test]
fn prefix_sweep() -> Result<()> {
    let expr = r#"<math>
        <mi intent=":unit">Qg</mi><mo>,</mo>
        <mi intent=":unit">Rg</mi><mo>,</mo>
        <mi intent=":unit">Yg</mi><mo>,</mo>
        <mi intent=":unit">Zg</mi><mo>,</mo>
        <mi intent=":unit">Eg</mi><mo>,</mo>
        <mi intent=":unit">Pg</mi><mo>,</mo>
        <mi intent=":unit">Tg</mi><mo>,</mo>
        <mi intent=":unit">Gg</mi><mo>,</mo>
        <mi intent=":unit">Mg</mi><mo>,</mo>
        <mi intent=":unit">kg</mi><mo>,</mo>
        <mi intent=":unit">hg</mi><mo>,</mo>
        <mi intent=":unit">dag</mi><mo>,</mo>
        <mi intent=":unit">dg</mi><mo>,</mo>
        <mi intent=":unit">cg</mi><mo>,</mo>
        <mi intent=":unit">mg</mi><mo>,</mo>
        <mi intent=":unit">µg</mi><mo>,</mo>
        <mi intent=":unit">ng</mi><mo>,</mo>
        <mi intent=":unit">pg</mi><mo>,</mo>
        <mi intent=":unit">fg</mi><mo>,</mo>
        <mi intent=":unit">ag</mi><mo>,</mo>
        <mi intent=":unit">zg</mi><mo>,</mo>
        <mi intent=":unit">yg</mi><mo>,</mo>
        <mi intent=":unit">rg</mi><mo>,</mo>
        <mi intent=":unit">qg</mi>
        </math>"#;
    test("el", "SimpleSpeak", expr, 
        "κουέτα-γραμμάρια, κόμμα; \
                ρόνα-γραμμάρια, κόμμα; \
                γιότα-γραμμάρια, κόμμα; \
                ζἐτα-γραμμάρια, κόμμα; \
                έξα-γραμμάρια, κόμμα; \
                πέτα-γραμμάρια, κόμμα; \
                τέρα-γραμμάρια, κόμμα; \
                γκίγκα-γραμμάρια, κόμμα; \
                μέγκα-γραμμάρια, κόμμα, \
                κιλά κόμμα; \
                χέκτο-γραμμάρια, κόμμα; \
                δέκα-γραμμάρια, κόμμα; \
                δεκατό-γραμμάρια, κόμμα; \
                εκατοστό-γραμμάρια; κόμμα; \
                μίλι-γραμμάρια, κόμμα; \
                μίκρο-γραμμάρια, κόμμα; \
                νάνο-γραμμάρια, κόμμα; \
                πίκο-γραμμάρια, κόμμα; \
                φέμτο-γραμμάρια, κόμμα; \
                άτο-γραμμάρια, κόμμα; \
                ζέπτο-γραμμάρια, κόμμα; \
                γιόκτο-γραμμάρια, κόμμα; \
                ρόντο-γραμμάρια, κόμμα; \
                κουέκτο-γραμμάρια")?;
                return Ok(());

}

#[test]
fn si_base() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">A</mi><mo>,</mo><mn>2</mn><mi intent=":unit">A</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">cd</mi><mo>,</mo><mn>2</mn><mi intent=":unit">cd</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">K</mi><mo>,</mo><mn>2</mn><mi intent=":unit">K</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">K</mi><mo>,</mo><mn>2</mn><mi intent=":unit">K</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">g</mi><mo>,</mo><mn>2</mn><mi intent=":unit">g</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">m</mi><mo>,</mo><mn>2</mn><mi intent=":unit">m</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">mol</mi><mo>,</mo><mn>2</mn><mi intent=":unit">mol</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">s</mi><mo>,</mo><mn>2</mn><mi intent=":unit">s</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">″</mi><mo>,</mo><mn>2</mn><mi intent=":unit">″</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">&quot;</mi><mo>,</mo><mn>2</mn><mi intent=":unit">&quot;</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">sec</mi><mo>,</mo><mn>2</mn><mi intent=":unit">sec</mi>
    </math>"#;
    test("el", "SimpleSpeak", expr, 
        "1 αμπέρ, κόμμα; 2 αμπέρ, κόμμα; \
                1 καντέλα, κόμμα; 2 καντέλα, κόμμα; \
                1 κέλβιν, κόμμα; 2 κέλβιν, κόμμα; \
                1 κέλβιν, κόμμα; 2 κέλβιν, κόμμα; \
                1 γραμμάριο, κόμμα; 2 γραμμάρια, κόμμα; \
                1 μέτρο, κόμμα; 2 μέτρα, κόμμα; \
                1 μολ, κόμμα; 2 μολ, κόμμα; \
                1 δευτερόλεπτο, κόμμα; 2 δευτερόλεπτα, κόμμα; \
                1 δευτερόλεπτο, κόμμα; 2 δευτερόλεπτα, κόμμα; \
                1 δευτερόλεπτο, κόμμα; 2 δευτερόλεπτα, κόμμα; \
                1 δευτερόλεπτο, κόμμα; 2 δευτερόλεπτα")?;
                return Ok(());

}

#[test]
fn si_base_with_prefixes() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">QA</mi><mo>,</mo><mn>2</mn><mi intent=":unit">RA</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ycd</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Zcd</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">EK</mi><mo>,</mo><mn>2</mn><mi intent=":unit">PK</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">TK</mi><mo>,</mo><mn>2</mn><mi intent=":unit">GK</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Mg</mi><mo>,</mo><mn>2</mn><mi intent=":unit">kg</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">hm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dam</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dmol</mi><mo>,</mo><mn>2</mn><mi intent=":unit">cmol</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ms</mi><mo>,</mo><mn>2</mn><mi intent=":unit">µs</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">nsec</mi><mo>,</mo><mn>2</mn><mi intent=":unit">psec</mi>
    </math>"#;
    test("el", "SimpleSpeak", expr, 
        "1 κουέτα-αμπέρ; κόμμα; 2 ρόνα-αμπέρ; κόμμα; \
                1 γιότα-καντέλα; κόμμα; 2 ζἐτα-καντέλα; κόμμα; \
                1 έξα-κέλβιν; κόμμα; 2 πέτα-κέλβιν; κόμμα; \
                1 τέρα-κέλβιν; κόμμα; 2 γκίγκα-κέλβιν; κόμμα; \
                1, μέγκα-γραμμάριο; κόμμα; 2 κιλά, κόμμα; \
                1 χέκτο-μέτρο; κόμμα; 2 δέκα-μέτρα; κόμμα; \
                1 δεκατό-μολ; κόμμα; 2 εκατοστό-μολ; κόμμα; \
                1, μίλι-δευτερόλεπτο; κόμμα; 2, μίκρο-δευτερόλεπτα; κόμμα; \
                1, νάνο-δευτερόλεπτο; κόμμα; 2, πίκο-δευτερόλεπτα")?;
                return Ok(());

}


#[test]
fn si_derived_1() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">Bq</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Bq</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">C</mi><mo>,</mo><mn>2</mn><mi intent=":unit">C</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">°C</mi><mo>,</mo><mn>2</mn><mi intent=":unit">°C</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">℃</mi><mo>,</mo><mn>2</mn><mi intent=":unit">℃</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">F</mi><mo>,</mo><mn>2</mn><mi intent=":unit">F</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Gy</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Gy</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">H</mi><mo>,</mo><mn>2</mn><mi intent=":unit">H</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Hz</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Hz</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">J</mi><mo>,</mo><mn>2</mn><mi intent=":unit">J</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">kat</mi><mo>,</mo><mn>2</mn><mi intent=":unit">kat</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">lm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">lm</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">lx</mi><mo>,</mo><mn>2</mn><mi intent=":unit">lx</mi>
    </math>"#;
    test("el", "SimpleSpeak", expr, 
        "1 μπεκερέλ, κόμμα; 2 μπεκερέλ, κόμμα; \
                1 κουλόμπ, κόμμα; 2 κουλόμπ, κόμμα; \
                1 βαθμός κελσίου; κόμμα; 2 βαθμοί κελσίου; κόμμα; \
                1 βαθμός κελσίου; κόμμα; 2 βαθμοί κελσίου; κόμμα; \
                1 φαράντ, κόμμα; 2 φαράντ, κόμμα; \
                1 γκρέυ, κόμμα; 2 γκρέυ, κόμμα; \
                1 χένρι, κόμμα; 2 χένρι, κόμμα; \
                1 χέρτς, κόμμα; 2 χέρτς, κόμμα; \
                1 τζάουλ, κόμμα; 2 τζάουλ, κόμμα; \
                1 κάτελ, κόμμα; 2 κάτελ, κόμμα; \
                1 λούμεν, κόμμα; 2 λούμεν, κόμμα; \
                1 λουξ, κόμμα; 2 λουξ")?;
                return Ok(());

}

#[test]
fn si_derived_1_with_prefixes() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">QBq</mi><mo>,</mo><mn>2</mn><mi intent=":unit">RBq</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">YC</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ZC</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">EF</mi><mo>,</mo><mn>2</mn><mi intent=":unit">PF</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">TGy</mi><mo>,</mo><mn>2</mn><mi intent=":unit">GGy</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">MH</mi><mo>,</mo><mn>2</mn><mi intent=":unit">kH</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">daHz</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dHz</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">cJ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">mJ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">µkat</mi><mo>,</mo><mn>2</mn><mi intent=":unit">nkat</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">plm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">flm</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">alx</mi><mo>,</mo><mn>2</mn><mi intent=":unit">zlx</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">m°C</mi><mo>,</mo><mn>2</mn><mi intent=":unit">µ°C</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">p℃</mi><mo>,</mo><mn>2</mn><mi intent=":unit">n℃</mi>
    </math>"#;
    test("el", "SimpleSpeak", expr, 
        "1, κουέτα-μπεκερέλ; κόμμα; 2 ρόνα-μπεκερέλ; κόμμα; \
                1 γιότα-κουλόμπ; κόμμα; 2 ζἐτα-κουλόμπ; κόμμα; \
                1 έξα-φαράντ; κόμμα; 2 πέτα-φαράντ; κόμμα; \
                1 τέρα-γκρέυ; κόμμα; 2 γκίγκα-γκρέυ; κόμμα; \
                1 μέγκα-χένρι; κόμμα; 2 χίλιο-χένρι; κόμμα; \
                1 δέκα-χέρτς; κόμμα; 2 δεκατό-χέρτς; κόμμα; \
                1, εκατοστό-τζάουλ; κόμμα; 2 μίλι-τζάουλ; κόμμα; \
                1 μίκρο-κάτελ; κόμμα; 2 νάνο-κάτελ; κόμμα; \
                1 πίκο-λούμεν; κόμμα; 2 φέμτο-λούμεν; κόμμα; \
                1 άτο-λουξ, κόμμα; 2 ζέπτο-λουξ; κόμμα; \
                1, μίλι-βαθμός κελσίου; κόμμα; 2, μίκρο-βαθμοί κελσίου; κόμμα; \
                1, πίκο-βαθμός κελσίου; κόμμα; 2, νάνο-βαθμοί κελσίου")?;
                return Ok(());

}

#[test]
fn si_derived_2() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">N</mi><mo>,</mo><mn>2</mn><mi intent=":unit">N</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ω</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Ω</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ω</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Ω</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Pa</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Pa</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">S</mi><mo>,</mo><mn>2</mn><mi intent=":unit">S</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Sv</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Sv</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">T</mi><mo>,</mo><mn>2</mn><mi intent=":unit">T</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">V</mi><mo>,</mo><mn>2</mn><mi intent=":unit">V</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">W</mi><mo>,</mo><mn>2</mn><mi intent=":unit">W</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Wb</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Wb</mi>
    </math>"#;
    test("el", "SimpleSpeak", expr, 
        "1 νιούτον, κόμμα; 2 νιούτον, κόμμα; \
                1 ωμ, κόμμα; 2 ωμ, κόμμα; \
                1 ωμ, κόμμα; 2 ωμ, κόμμα; \
                1 πασκάλ, κόμμα; 2 πασκάλ, κόμμα; \
                1 ζίμενς, κόμμα; 2 ζίμενς, κόμμα; \
                1 σίβερτ, κόμμα; 2 σίβερτ, κόμμα; \
                1 τέσλα, κόμμα; 2 τέσλα, κόμμα; \
                1 βόλτ, κόμμα; 2 βόλτ, κόμμα; \
                1 βατ, κόμμα; 2 βατ, κόμμα; \
                1 βέμπερ, κόμμα; 2 βέμπερ")?;
                return Ok(());

}

#[test]
fn si_derived_2_with_prefixes() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">qN</mi><mo>,</mo><mn>2</mn><mi intent=":unit">rN</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">yΩ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">zΩ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">aΩ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">fΩ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">pPa</mi><mo>,</mo><mn>2</mn><mi intent=":unit">nPa</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">µS</mi><mo>,</mo><mn>2</mn><mi intent=":unit">mS</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">cSv</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dSv</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">daT</mi><mo>,</mo><mn>2</mn><mi intent=":unit">hT</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">kV</mi><mo>,</mo><mn>2</mn><mi intent=":unit">MV</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">GW</mi><mo>,</mo><mn>2</mn><mi intent=":unit">TW</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">PWb</mi><mo>,</mo><mn>2</mn><mi intent=":unit">EWb</mi>
    </math>"#;
    test("el", "SimpleSpeak", expr, 
        "1, κουέκτο-νιούτον; κόμμα; 2 ρόντο-νιούτον; κόμμα; \
                1 γιόκτο-ωμ; κόμμα; 2 ζέπτο-ωμ, κόμμα; \
                1 άτο-ωμ, κόμμα; 2 φέμτο-ωμ, κόμμα; \
                1 πίκο-πασκάλ; κόμμα; 2 νάνο-πασκάλ; κόμμα; \
                1 μίκρο-ζίμενς; κόμμα; 2 μίλι-ζίμενς; κόμμα; \
                1, εκατοστό-σίβερτ; κόμμα; 2 δεκατό-σίβερτ; κόμμα; \
                1 δέκα-τέσλα; κόμμα; 2 χέκτο-τέσλα; κόμμα; \
                1 χίλιο-βόλτ; κόμμα; 2 μέγκα-βόλτ; κόμμα; \
                1 γκίγκα-βατ; κόμμα; 2 τέρα-βατ, κόμμα; \
                1 πέτα-βέμπερ; κόμμα; 2 έξα-βέμπερ")?;
                return Ok(());

}


#[test]
fn si_accepted() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">l</mi><mo>,</mo><mn>2</mn><mi intent=":unit">l</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">L</mi><mo>,</mo><mn>2</mn><mi intent=":unit">L</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ℓ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ℓ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">t</mi><mo>,</mo><mn>2</mn><mi intent=":unit">t</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Da</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Da</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Np</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Np</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">u</mi><mo>,</mo><mn>2</mn><mi intent=":unit">u</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">eV</mi><mo>,</mo><mn>2</mn><mi intent=":unit">eV</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">rad</mi><mo>,</mo><mn>2</mn><mi intent=":unit">rad</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">sr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">sr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">a</mi><mo>,</mo><mn>2</mn><mi intent=":unit">a</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">as</mi><mo>,</mo><mn>2</mn><mi intent=":unit">as</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">b</mi><mo>,</mo><mn>2</mn><mi intent=":unit">b</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">B</mi><mo>,</mo><mn>2</mn><mi intent=":unit">B</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Bd</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Bd</mi>
    </math>"#;
    test("el", "SimpleSpeak", expr, 
        "1 λίτρο, κόμμα; 2 λίτρα, κόμμα; \
                1 λίτρο, κόμμα; 2 λίτρα, κόμμα; \
                1 λίτρο, κόμμα; 2 λίτρα, κόμμα; \
                1 τόνος, κόμμα; 2 τόνοι, κόμμα; \
                1 ντάλτον, κόμμα; 2 ντάλτον, κόμμα; \
                1 νέπερ, κόμμα; 2 νέπερ, κόμμα; \
                1, ατομική μονάδα μάζας; κόμμα; 2, ατομικές μονάδες μάζας; κόμμα; \
                1 ηλεκτρονιοβόλτ; κόμμα; 2 ηλεκτρονιοβόλτ; κόμμα; \
                1 ακτίνιο, κόμμα; 2 ακτίνια, κόμμα; \
                1 στερακτίνιο, κόμμα; 2 στερακτίνια, κόμμα; \
                1 άνουμ, κόμμα; 2 άνουμ, κόμμα; \
                1 δευτερόλεπτο τόξου; κόμμα; 2 δευτερόλεπτα τόξου; κόμμα; \
                1 μπιτ, κόμμα; 2 μπιτ, κόμμα; \
                1 μπάιτ, κόμμα; 2 μπάιτ, κόμμα; \
                1 μπώ, κόμμα; 2 μπώ")?;
                return Ok(());

}

#[test]
fn si_accepted_with_prefixes() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">Ql</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Rl</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">YL</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ZL</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Eℓ</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Pℓ</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Tt</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Gt</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">MDa</mi><mo>,</mo><mn>2</mn><mi intent=":unit">kDa</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dNp</mi><mo>,</mo><mn>2</mn><mi intent=":unit">cNp</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">hu</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dau</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">meV</mi><mo>,</mo><mn>2</mn><mi intent=":unit">µeV</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">nrad</mi><mo>,</mo><mn>2</mn><mi intent=":unit">prad</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">fsr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">asr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ga</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Ma</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">zas</mi><mo>,</mo><mn>2</mn><mi intent=":unit">yas</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">kb</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Mb</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">GB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">TB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">TBd</mi><mo>,</mo><mn>2</mn><mi intent=":unit">EBd</mi>
    </math>"#;
    test("el", "SimpleSpeak", expr, 
        "1 κουέτα-λίτρο; κόμμα; 2 ρόνα-λίτρα; κόμμα; \
                1 γιότα-λίτρο; κόμμα; 2 ζἐτα-λίτρα; κόμμα; \
                1 έξα-λίτρο; κόμμα; 2 πέτα-λίτρα; κόμμα; \
                1 τέρα-τόνος; κόμμα; 2 γκίγκα-τόνοι; κόμμα; \
                1 μέγκα-ντάλτον; κόμμα; 2 χίλιο-ντάλτον; κόμμα; \
                1 δεκατό-νέπερ; κόμμα; 2 εκατοστό-νέπερ; κόμμα; \
                1, χέκτο-ατομική μονάδα μάζας; κόμμα; \
                2, δέκα-ατομικές μονάδες μάζας; κόμμα; \
                1, μίλι-ηλεκτρονιοβόλτ; κόμμα; 2, μίκρο-ηλεκτρονιοβόλτ; κόμμα; \
                1 νάνο-ακτίνιο; κόμμα; 2 πίκο-ακτίνια; κόμμα; \
                1, φέμτο-στερακτίνιο; κόμμα; 2, άτο-στερακτίνια; κόμμα; \
                1 γκίγκα-άνουμ; κόμμα; 2 μέγκα-άνουμ; κόμμα; \
                1, ζέπτο-δευτερόλεπτο τόξου; κόμμα; 2, γιόκτο-δευτερόλεπτα τόξου; κόμμα; \
                1 κιλο-μπιτ, κόμμα; 2 μέγκα-μπιτ; κόμμα; \
                1 γκίγκα-μπάιτ; κόμμα; 2 τέρα-μπάιτ; κόμμα; \
                1 τέρα-μπώ, κόμμα; 2 έξα-μπώ")?;
                return Ok(());

}

#[test]
fn without_prefix_time() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">″</mi><mo>,</mo><mn>2</mn><mi intent=":unit">″</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">&quot;</mi><mo>,</mo><mn>2</mn><mi intent=":unit">&quot;</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">′</mi><mo>,</mo><mn>2</mn><mi intent=":unit">′</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">'</mi><mo>,</mo><mn>2</mn><mi intent=":unit">'</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">min</mi><mo>,</mo><mn>2</mn><mi intent=":unit">min</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">h</mi><mo>,</mo><mn>2</mn><mi intent=":unit">h</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">hr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">hr</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">Hr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Hr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">d</mi><mo>,</mo><mn>2</mn><mi intent=":unit">d</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dy</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dy</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">w</mi><mo>,</mo><mn>2</mn><mi intent=":unit">w</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">wk</mi><mo>,</mo><mn>2</mn><mi intent=":unit">wk</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">y</mi><mo>,</mo><mn>2</mn><mi intent=":unit">y</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">yr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">yr</mi>
    </math>"#;
    test("el", "SimpleSpeak", expr, 
        "1 δευτερόλεπτο, κόμμα; 2 δευτερόλεπτα, κόμμα; \
                1 δευτερόλεπτο, κόμμα; 2 δευτερόλεπτα, κόμμα; \
                1 λεπτό, κόμμα; 2 λεπτά, κόμμα; \
                1 λεπτό, κόμμα; 2 λεπτά, κόμμα; \
                1 λεπτό, κόμμα; 2 λεπτά, κόμμα; \
                1 ώρα, κόμμα; 2 ώρες, κόμμα; \
                1 ώρα, κόμμα; 2 ώρες, κόμμα; \
                1 ώρα, κόμμα; 2 ώρες, κόμμα; \
                1 ημέρα, κόμμα; 2 ημέρες, κόμμα; \
                1 ημέρα, κόμμα; 2 ημέρες, κόμμα; \
                1 εβδομάδα, κόμμα; 2 εβδομάδες, κόμμα; \
                1 εβδομάδα, κόμμα; 2 εβδομάδες, κόμμα; \
                1 έτος, κόμμα; 2 έτη, κόμμα; \
                1 έτος, κόμμα; 2 έτη")?;
                return Ok(());

}

#[test]
fn without_prefix_angles() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">°</mi><mo>,</mo><mn>2</mn><mi intent=":unit">°</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">deg</mi><mo>,</mo><mn>2</mn><mi intent=":unit">deg</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">arcmin</mi><mo>,</mo><mn>2</mn><mi intent=":unit">arcmin</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">amin</mi><mo>,</mo><mn>2</mn><mi intent=":unit">amin</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">am</mi><mo>,</mo><mn>2</mn><mi intent=":unit">am</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">MOA</mi><mo>,</mo><mn>2</mn><mi intent=":unit">MOA</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">arcsec</mi><mo>,</mo><mn>2</mn><mi intent=":unit">arcsec</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">asec</mi><mo>,</mo><mn>2</mn><mi intent=":unit">asec</mi>
    </math>"#;
    test("el", "SimpleSpeak", expr, 
        "1 βαθμός, κόμμα; 2 βαθμοί, κόμμα; \
        1 βαθμός, κόμμα; 2 βαθμοί, κόμμα; \
        1 λεπτό τόξου, κόμμα; 2 λεπτά τόξου, κόμμα; \
        1 λεπτό τόξου, κόμμα; 2 λεπτά τόξου, κόμμα; \
        1 λεπτό τόξου, κόμμα; 2 λεπτά τόξου, κόμμα; \
        1 λεπτό τόξου, κόμμα; 2 λεπτά τόξου, κόμμα; \
        1 δευτερόλεπτο τόξου; κόμμα; 2 δευτερόλεπτα τόξου; κόμμα; \
        1 δευτερόλεπτο τόξου; κόμμα; 2 δευτερόλεπτα τόξου")?;
                return Ok(());

}

#[test]
fn without_prefix_distance() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">au</mi><mo>,</mo><mn>2</mn><mi intent=":unit">au</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ltyr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ltyr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">pc</mi><mo>,</mo><mn>2</mn><mi intent=":unit">pc</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Å</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Å</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Å</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Å</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">fm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">fm</mi>
    </math>"#;
    test("el", "SimpleSpeak", expr, 
        "1 αστρονομική μονάδα; κόμμα; 2, αστρονομικές μονάδες; κόμμα; \
        1 έτος φωτός, κόμμα; 2 έτη φωτός, κόμμα; \
        1 παρσέκ, κόμμα; 2 παρσέκ, κόμμα; \
        1 άνγκστρομ, κόμμα; 2 άνγκστρομ, κόμμα; \
        1 άνγκστρομ, κόμμα; 2 άνγκστρομ, κόμμα; \
        1 φέρμι, κόμμα; 2 φέρμι")?;
                return Ok(());

}

#[test]
fn without_prefix_other() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">ha</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ha</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">atm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">atm</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">amu</mi><mo>,</mo><mn>2</mn><mi intent=":unit">amu</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">bar</mi><mo>,</mo><mn>2</mn><mi intent=":unit">bar</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">cal</mi><mo>,</mo><mn>2</mn><mi intent=":unit">cal</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Ci</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Ci</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">grad</mi><mo>,</mo><mn>2</mn><mi intent=":unit">grad</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">M</mi><mo>,</mo><mn>2</mn><mi intent=":unit">M</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">R</mi><mo>,</mo><mn>2</mn><mi intent=":unit">R</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">rpm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">rpm</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">fl dr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">fl dr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">℧</mi><mo>,</mo><mn>2</mn><mi intent=":unit">℧</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dyn</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dyn</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">erg</mi><mo>,</mo><mn>2</mn><mi intent=":unit">erg</mi>
    </math>"#;
    test("el", "SimpleSpeak", expr, 
        "1 εκτάριο, κόμμα; 2 εκτάρια, κόμμα; \
        1 ντεσιμπέλ, κόμμα; 2 ντεσιμπέλ, κόμμα; \
        1 ατμόσφαιρα, κόμμα; 2 ατμόσφαιρες, κόμμα; \
        1, ατομική μονάδα μάζας; κόμμα; 2, ατομικές μονάδες μάζας; κόμμα; \
        1 μπαρ, κόμμα; 2 μπαρ, κόμμα; \
        1 θερμίδα, κόμμα; 2 θερμίδες, κόμμα; \
        1 κιουρί, κόμμα; 2 κιουρί, κόμμα; \
        1 βαθμός, κόμμα; 2 βαθμοί, κόμμα; \
        1 μολάρ, κόμμα; 2 μολάρ, κόμμα; \
        1 ρέντγκεν, κόμμα; 2 ρεντγκεν, κόμμα; \
        1 στροφή ανά λεπτό; κόμμα; 2 στροφές ανά λεπτό; κόμμα; \
        1 δράμι υγρών, κόμμα; 2 δράμια υγρών, κόμμα; \
        1 μο, κόμμα; 2 μο, κόμμα; \
        1 ντυν, κόμμα; 2 ντυν, κόμμα; \
        1 εργ, κόμμα; 2 εργ")?;
                return Ok(());

}

#[test]
fn without_prefix_powers_of_2() -> Result<()> {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">Kib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Kib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Mib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Mib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Gib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Gib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Tib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Tib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Pib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Pib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Eib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Eib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Zib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Zib</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">Yib</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Yib</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">KiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">KiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">MiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">MiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">GiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">GiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">TiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">TiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">PiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">PiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">EiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">EiB</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ZiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ZiB</mi><mo>,</mo> 
        <mn>1</mn><mi intent=":unit">YiB</mi><mo>,</mo><mn>2</mn><mi intent=":unit">YiB</mi>
    </math>"#;
    test("el", "SimpleSpeak", expr, 
        "1 kibi-bit, κόμμα; 2 kibi-bits, κόμμα; \
                1 mebi-bit, κόμμα; 2 mebi-bits, κόμμα; \
                1 gibi-bit, κόμμα; 2 gibi-bits, κόμμα; \
                1 tebi-bit, κόμμα; 2 tebi-bits, κόμμα; \
                1 pebi-bit, κόμμα; 2 pebi-bits, κόμμα; \
                1 exbi-bit, κόμμα; 2 exbi-bits, κόμμα; \
                1 zebi-bit, κόμμα; 2 zebi-bits, κόμμα; \
                1 yobi-bit, κόμμα; 2 yobi-bits, κόμμα; \
                1 kibi-byte, κόμμα; 2 kibi-bytes, κόμμα; \
                1 mebi-byte, κόμμα; 2 mebi-bytes, κόμμα; \
                1 gibi-byte, κόμμα; 2 gibi-bytes, κόμμα; \
                1 tebi-byte, κόμμα; 2 tebi-bytes, κόμμα; \
                1 pebi-byte, κόμμα; 2 pebi-bytes, κόμμα; \
                1 exbi-byte, κόμμα; 2 exbi-bytes, κόμμα; \
                1 zebi-byte, κόμμα; 2 zebi-bytes, κόμμα; \
                1 yobi-byte, κόμμα; 2 yobi-bytes")?;
                return Ok(());

}


#[test]
fn si_other_numbers() -> Result<()> {
    let expr = r#"<math><mn>1.0</mn><mi intent=":unit">l</mi><mo>,</mo>
                            <mn>2.0</mn><mo>&#xA0;</mo><mi intent=":unit">m</mi><mo>,</mo>
                            <mi>x</mi><mo>&#xA0;</mo><mi intent=":unit">ms</mi><mo>,</mo>
                            <mi>y</mi><mi intent=":unit">µs</mi><mo>,</mo>
                            <mi intent=":unit">dag</mi><mo>,</mo>
                            <mn>1235</mn><mi intent=":unit">daN</mi><mo>,</mo>
                            <mn>2.5</mn><mi intent=":unit">&#xB5;sec</mi><mo>,</mo>
                            <mn>32.34</mn><mi intent=":unit">mol</mi></math>"#;
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Terse")], expr,
            "1.0 l κόμμα, 2.0 m κόμμα; x, μίλι-δευτερόλεπτα; κόμμα; y, μίκρο-δευτερόλεπτα; κόμμα; \
            δέκα-γραμμάρια, κόμμα; 1235 δέκα-νιούτον; κόμμα; 2.5, μίκρο-δευτερόλεπτα; κόμμα; 32.34 μολ")?;
    test_prefs("el", "ClearSpeak", vec![("Verbosity", "Medium")], expr,
            "1.0 λίτρο, κόμμα; 2.0 μέτρα, κόμμα; x, μίλι-δευτερόλεπτα; κόμμα; y, μίκρο-δευτερόλεπτα; κόμμα; \
            δέκα-γραμμάρια, κόμμα; 1235 δέκα-νιούτον; κόμμα; 25, μίκρο-δευτερόλεπτα; κόμμα; 32.34 μολ")?;
    test_prefs("el", "SimpleSpeak", vec![("Verbosity", "Verbose")], expr,
            "1.0 λίτρο, κόμμα; 2.0 μέτρα, κόμμα; x, μίλι-δευτερόλεπτα; κόμμα; y, μίκρο-δευτερόλεπτα; κόμμα; \
                    δέκα-γραμμάρια, κόμμα; 1235 δέκα-νιούτον; κόμμα; 2.5, μίκρο-δευτερόλεπτα; κόμμα; 32.34 μολ")?;
                    return Ok(());
                    //theodora. fails  32.34 --> 3234
                    // The issue is at src/prefs (set_separator, decimal_separator)

}


#[test]
fn test_mtext_inference() -> Result<()> {
    let expr = r#"<math><mo>[</mo>
                <mn>1</mn><mtext>t</mtext><mo>,</mo>
                <mn>2</mn><mtext>PA</mtext><mo>,</mo>
                <mn>3</mn><mtext>Pa</mtext><mo>,</mo>
                <mn>4.5</mn><mtext>mT</mtext>
            <mo>]</mo></math>"#;
    test("el", "SimpleSpeak", expr, 
        "ανοίγει αγκύλη; 1 τόνος, κόμμα; 2 πέτα-αμπέρ; κόμμα; \
                3 πασκάλ, κόμμα; 4.5 μίλι-τέσλα; κλείνει αγκύλη")?;
                return Ok(());
                //theodora. fails 4.5 --> converted to 45 
                // // The issue is at src/prefs (set_separator, decimal_separator)

}

    #[test]
    fn infer_unit() -> Result<()> {
        let expr = r#"<math>
            <mn>3</mn><mi mathvariant="normal">m</mi><mo>,</mo>
            <mn>1</mn><mi>km</mi><mo>,</mo>
            <mn>3</mn><mtext>m</mtext><mo>,</mo>
            <mfrac><mn>3</mn><mn>10</mn></mfrac><mi mathvariant="normal">F</mi><mo>,</mo>
            <msub><mi>m</mi><mi>min</mi></msub>
            </math>"#;
        test("el", "SimpleSpeak", expr, 
            "3 μέτρα, κόμμα; 1 χίλιο-μέτρο; κόμμα; 3 μέτρα, κόμμα; 3 δέκατα φαράντ; κόμμα; m δείκτης ελάχιστο τέλος δείκτη")?;
            return Ok(());

    }
