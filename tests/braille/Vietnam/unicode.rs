// Vietnam braille Unicode full-range coverage tests.
use crate::common::*;
use anyhow::Result;

#[test]
fn unicode_full_italic_capitals() -> Result<()> {
    let expr = r#"<math><mi>𝐴𝐵𝐶𝐷𝐸𝐹𝐺𝐻𝐼𝐽𝐾𝐿𝑀𝑁𝑂𝑃𝑄𝑅𝑆𝑇𝑈𝑉𝑊𝑋𝑌𝑍</mi></math>"#;
    test_braille("Vietnam", expr, "⠸⠨⠁⠸⠨⠃⠸⠨⠉⠸⠨⠙⠸⠨⠑⠸⠨⠋⠸⠨⠛⠸⠨⠓⠸⠨⠊⠸⠨⠚⠸⠨⠅⠸⠨⠇⠸⠨⠍⠸⠨⠝⠸⠨⠕⠸⠨⠏⠸⠨⠟⠸⠨⠗⠸⠨⠎⠸⠨⠞⠸⠨⠥⠸⠨⠧⠸⠨⠺⠸⠨⠭⠸⠨⠽⠸⠵")?;
    return Ok(())
}

#[test]
fn unicode_full_bold_capitals() -> Result<()> {
    let expr = r#"<math><mi>𝐀𝐁𝐂𝐃𝐄𝐅𝐆𝐇𝐈𝐉𝐊𝐋𝐌𝐍𝐎𝐏𝐐𝐑𝐒𝐓𝐔𝐕𝐖𝐗𝐘𝐙</mi></math>"#;
    test_braille("Vietnam", expr, "⠘⠸⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵")?;
    return Ok(())
}

#[test]
fn unicode_full_bold_lowercase() -> Result<()> {
    let expr = r#"<math><mi>𝐚𝐛𝐜𝐝𝐞𝐟𝐠𝐡𝐢𝐣𝐤𝐥𝐦𝐧𝐨𝐩𝐪𝐫𝐬𝐭𝐮𝐯𝐰𝐱𝐲𝐳</mi></math>"#;
    test_braille("Vietnam", expr, "⠘⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵")?;
    return Ok(())
}

#[test]
fn unicode_full_italic_lowercase() -> Result<()> {
    let expr = r#"<math><mi>𝑎𝑏𝑐𝑑𝑒𝑓𝑔𝑕𝑖𝑗𝑘𝑙𝑚𝑛𝑜𝑝𝑞𝑟𝑠𝑡𝑢𝑣𝑤𝑥𝑦𝑧</mi></math>"#;
    test_braille("Vietnam", expr, "⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵")?;
    return Ok(())
}

#[test]
fn unicode_full_bold_italic_capitals() -> Result<()> {
    let expr = r#"<math><mi>𝑨𝑩𝑪𝑫𝑬𝑭𝑮𝑯𝑰𝑱𝑲𝑳𝑴𝑵𝑶𝑷𝑸𝑹𝑺𝑻𝑼𝑽𝑾𝑿𝒀𝒁</mi></math>"#;
    test_braille("Vietnam", expr, "⠘⠨⠸⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵")?;
    return Ok(())
}

#[test]
fn unicode_full_bold_italic_lowercase() -> Result<()> {
    let expr = r#"<math><mi>𝒂𝒃𝒄𝒅𝒆𝒇𝒈𝒉𝒊𝒋𝒌𝒍𝒎𝒏𝒐𝒑𝒒𝒓𝒔𝒕𝒖𝒗𝒘𝒙𝒚𝒛</mi></math>"#;
    test_braille("Vietnam", expr, "⠘⠨⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵")?;
    return Ok(())
}

#[test]
fn unicode_full_script_capitals() -> Result<()> {
    let expr = r#"<math><mi>𝒜𝒝𝒞𝒟𝒠𝒡𝒢𝒣𝒤𝒥𝒦𝒧𝒨𝒩𝒪𝒫𝒬𝒭𝒮𝒯𝒰𝒱𝒲𝒳𝒴𝒵</mi></math>"#;
    test_braille("Vietnam", expr, "⠈⠆⠨⠁⠆⠨⠃⠆⠨⠉⠆⠨⠙⠆⠨⠑⠆⠨⠋⠆⠨⠛⠆⠨⠓⠆⠨⠊⠆⠨⠚⠆⠨⠅⠆⠨⠇⠆⠨⠍⠆⠨⠝⠆⠨⠕⠆⠨⠏⠆⠨⠟⠆⠨⠗⠆⠨⠎⠆⠨⠞⠆⠨⠥⠆⠨⠧⠆⠨⠺⠆⠨⠭⠆⠨⠽⠆⠨⠵")?;
    return Ok(())
}

#[test]
fn unicode_full_script_lowercase() -> Result<()> {
    let expr = r#"<math><mi>𝒶𝒷𝒸𝒹𝒺𝒻𝒼𝒽𝒾𝒿𝓀𝓁𝓂𝓃𝓄𝓅𝓆𝓇𝓈𝓉𝓊𝓋𝓌𝓍𝓎𝓏</mi></math>"#;
    test_braille("Vietnam", expr, "⠈⠆⠁⠆⠃⠆⠉⠆⠙⠆⠑⠆⠋⠆⠛⠆⠓⠆⠊⠆⠚⠆⠅⠆⠇⠆⠍⠆⠝⠆⠕⠆⠏⠆⠟⠆⠗⠆⠎⠆⠞⠆⠥⠆⠧⠆⠺⠆⠭⠆⠽⠆⠵")?;
    return Ok(())
}

#[test]
fn unicode_full_blackletter_capitals() -> Result<()> {
    let expr = r#"<math><mi>𝔄𝔅𝔆𝔇𝔈𝔉𝔊𝔋𝔌𝔍𝔎𝔏𝔐𝔑𝔒𝔓𝔔𝔕𝔖𝔗𝔘𝔙𝔚𝔛𝔜𝔝</mi></math>"#;
    test_braille("Vietnam", expr, "⠈⠸⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵")?;
    return Ok(())
}

#[test]
fn unicode_full_blackletter_lowercase() -> Result<()> {
    let expr = r#"<math><mi>𝔞𝔟𝔠𝔡𝔢𝔣𝔤𝔥𝔦𝔧𝔨𝔩𝔪𝔫𝔬𝔭𝔮𝔯𝔰𝔱𝔲𝔳𝔴𝔵𝔶𝔷</mi></math>"#;
    test_braille("Vietnam", expr, "⠈⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵")?;
    return Ok(())
}

#[test]
fn unicode_full_blackboard_specials() -> Result<()> {
    let expr = r#"<math><mi>ℂℍℕ𝕆</mi></math>"#;
    test_braille("Vietnam", expr, "⠈⠸⠉⠓⠝⠕")?;
    return Ok(())
}

#[test]
fn unicode_full_bold_digits() -> Result<()> {
    let expr = r#"<math><mi>𝟎𝟏𝟐𝟑𝟒𝟓𝟔𝟕𝟖𝟗</mi></math>"#;
    test_braille("Vietnam", expr, "⠘⠆⠼⠼⠚⠘⠆⠼⠼⠁⠘⠆⠼⠼⠃⠘⠆⠼⠼⠉⠘⠆⠼⠼⠙⠘⠆⠼⠼⠑⠘⠆⠼⠼⠋⠘⠆⠼⠼⠛⠘⠆⠼⠼⠓⠘⠆⠼⠼⠊")?;
    return Ok(())
}

#[test]
fn unicode_full_blackboard_partial_lowercase() -> Result<()> {
    let expr = r#"<math><mi>𝕒𝕓𝕔𝕕𝕖𝕗𝕘𝕙𝕚𝕛𝕜𝕝𝕞𝕟𝕠𝕡𝕢𝕣𝕤𝕥𝕦𝕧𝕨𝕩𝕪𝕫</mi></math>"#;
    test_braille("Vietnam", expr, "⠈⠁⠃⠉⠙⠑⠋⠛⠓⠊⠚⠅⠇⠍⠝⠕⠏⠟⠗⠎⠞⠥⠧⠺⠭⠽⠵")?;
    return Ok(())
}

#[test]
fn unicode_full_special_constants() -> Result<()> {
    let expr = r#"<math><mi>ⅆⅇⅈ</mi></math>"#;
    test_braille("Vietnam", expr, "⠙⠑⠊")?;
    return Ok(())
}
