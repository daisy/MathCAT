/// Тесты для правил, общих для различных стилей речи:
/// * модифицированная переменная
use crate::common::*;

// Основная структура тестов:
// 1. Проход по всем приставкам СИ
// 2. Проход по каждой группе единиц СИ
//    a) как в единственном, так и во множественном числе без приставок
//    b) как в единственном, так и во множественном числе с одной приставкой
// 3. Проход по каждой группе единиц, не принимающих приставки СИ
// Они разбиты на части, чтобы легче было видеть ошибки, когда они есть.

#[test]
fn prefix_sweep() {
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
    test("ru", "SimpleSpeak", expr,
        "кветта-граммы, запятая, \
         ронна-граммы, запятая, \
         иотта-граммы, запятая, \
         зетта-граммы, запятая, \
         экса-граммы, запятая, \
         пета-граммы, запятая, \
         тера-граммы, запятая, \
         гига-граммы, запятая, \
         мега-граммы, запятая, \
         кило-граммы, запятая, \
         гекто-граммы, запятая, \
         дека-граммы, запятая, \
         деци-граммы, запятая, \
         санти-граммы, запятая, \
         милли-граммы, запятая, \
         микро-граммы, запятая, \
         нано-граммы, запятая, \
         пико-граммы, запятая, \
         фемто-граммы, запятая, \
         атто-граммы, запятая, \
         зепто-граммы, запятая, \
         иокто-граммы, запятая, \
         ронто-граммы, запятая, \
         квекто-граммы");
}

#[test]
fn si_base() {
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
    test("ru", "SimpleSpeak", expr,
        "1 ампер, запятая, 2 ампера, запятая, \
         1 кандела, запятая; 2 канделы, запятая, \
         1 кельвин, запятая, 2 кельвина, запятая, \
         1 кельвин, запятая, 2 кельвина, запятая, \
         1 грамм, запятая, 2 грамма, запятая, \
         1 метр, запятая, 2 метра, запятая, \
         1 моль, запятая, 2 моля, запятая, \
         1 секунда, запятая, 2 секунды, запятая, \
         1 секунда, запятая, 2 секунды, запятая, \
         1 секунда, запятая, 2 секунды, запятая, \
         1 секунда, запятая, 2 секунды");
}

#[test]
fn si_base_with_prefixes() {
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
    test("ru", "SimpleSpeak", expr,
        "1 кветта-ампер, запятая; 2 ронна-ампера, запятая; \
         1 иотта-кандела, запятая; 2 зетта-канделы, запятая; \
         1 экса-кельвин, запятая; 2 пета-кельвина, запятая; \
         1 тера-кельвин, запятая; 2 гига-кельвина, запятая; \
         1 мега-грамм, запятая; 2 кило-грамма, запятая; \
         1 гекто-метр, запятая; 2 дека-метра, запятая; \
         1 деци-моль, запятая; 2 санти-моля, запятая; \
         1 милли-секунда, запятая; 2 микро-секунды, запятая; \
         1 нано-секунда, запятая; 2 пико-секунды");
}

#[test]
fn si_derived_1() {
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
    test("ru", "SimpleSpeak", expr,
        "1 беккерель, запятая; 2 беккереля, запятая, \
         1 кулон, запятая; 2 кулона, запятая; \
         1 градус Цельсия, запятая; 2 градуса Цельсия, запятая; \
         1 градус Цельсия, запятая; 2 градуса Цельсия, запятая, \
         1 фарад, запятая, 2 фарада, запятая, \
         1 грей, запятая, 2 грея, запятая, \
         1 генри, запятая, 2 генри, запятая, \
         1 герц, запятая, 2 герца, запятая, \
         1 джоуль, запятая, 2 джоуля, запятая, \
         1 катал, запятая, 2 катала, запятая, \
         1 люмен, запятая, 2 люмена, запятая, \
         1 люкс, запятая, 2 люкс");
}

#[test]
fn si_derived_1_with_prefixes() {
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
    test("ru", "SimpleSpeak", expr,
        "1 кветта-беккерель, запятая; 2 ронна-беккереля; запятая; \
         1 иотта-кулон, запятая; 2 зетта-кулона, запятая; \
         1 экса-фарад, запятая; 2 пета-фарада, запятая; \
         1 тера-грей, запятая; 2 гига-грея, запятая; \
         1 мега-генри, запятая; 2 кило-генри, запятая; \
         1 дека-герц, запятая; 2 деци-герца, запятая; \
         1 санти-джоуль, запятая; 2 милли-джоуля, запятая; \
         1 микро-катал, запятая; 2 нано-катала, запятая; \
         1 пико-люмен, запятая; 2 фемто-люмена, запятая; \
         1 атто-люкс, запятая; 2 зепто-люкс, запятая; \
         1 милли-градус Цельсия; запятая; 2 микро-градуса Цельсия; запятая; \
         1 пико-градус Цельсия; запятая; 2 нано-градуса Цельсия");
}

#[test]
fn si_derived_2() {
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
    test("ru", "SimpleSpeak", expr,
        "1 ньютон, запятая, 2 ньютона, запятая, \
         1 ом, запятая, 2 ома, запятая, \
         1 ом, запятая, 2 ома, запятая, \
         1 паскаль, запятая, 2 паскаля, запятая, \
         1 сименс, запятая, 2 сименс, запятая, \
         1 зиверт, запятая; 2 зиверта, запятая, \
         1 тесла, запятая, 2 теслы, запятая, \
         1 вольт, запятая, 2 вольта, запятая, \
         1 ватт, запятая, 2 ватта, запятая, \
         1 вебер, запятая, 2 вебера");
}

#[test]
fn si_derived_2_with_prefixes() {
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
    test("ru", "SimpleSpeak", expr,
        "1 квекто-ньютон, запятая; 2 ронто-ньютона, запятая; \
         1 иокто-ом, запятая; 2 зепто-ома, запятая; \
         1 атто-ом, запятая; 2 фемто-ома, запятая; \
         1 пико-паскаль, запятая; 2 нано-паскаля, запятая; \
         1 микро-сименс, запятая; 2 милли-сименс, запятая; \
         1 санти-зиверт, запятая; 2 деци-зиверта, запятая; \
         1 дека-тесла, запятая; 2 гекто-теслы, запятая; \
         1 кило-вольт, запятая; 2 мега-вольта, запятая; \
         1 гига-ватт, запятая; 2 тера-ватта, запятая; \
         1 пета-вебер, запятая; 2 экса-вебера");
}

#[test]
fn si_accepted() {
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
    test("ru", "SimpleSpeak", expr,
        "1 литр, запятая, 2 литра, запятая, \
         1 литр, запятая, 2 литра, запятая, \
         1 литр, запятая, 2 литра, запятая, \
         1 тонна, запятая; 2 тонны, запятая, \
         1 дальтон, запятая, 2 дальтона, запятая, \
         1 непер, запятая, 2 непера, запятая; \
         1 атомная единица массы, запятая; 2 атомные единицы массы, запятая; \
         1 электронвольт, запятая; 2 электронвольта, запятая, \
         1 радиан, запятая, 2 радиана, запятая, \
         1 стерадиан, запятая; 2 стерадиана, запятая, \
         1 год, запятая, 2 года, запятая, \
         1 угловая секунда, запятая; 2 угловые секунды, запятая, \
         1 бит, запятая, 2 бита, запятая, \
         1 байт, запятая, 2 байта, запятая, \
         1 бод, запятая, 2 бода");
}

#[test]
fn si_accepted_with_prefixes() {
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
    test("ru", "SimpleSpeak", expr,
        "1 кветта-литр, запятая; 2 ронна-литра, запятая; \
         1 иотта-литр, запятая; 2 зетта-литра, запятая; \
         1 экса-литр, запятая; 2 пета-литра, запятая; \
         1 тера-тонна, запятая; 2 гига-тонны; запятая; \
         1 мега-дальтон, запятая; 2 кило-дальтона, запятая; \
         1 деци-непер, запятая; 2 санти-непера, запятая; \
         1 гекто-атомная единица массы; запятая; 2 дека-атомные единицы массы; запятая; \
         1 милли-электронвольт, запятая; 2 микро-электронвольта; запятая; \
         1 нано-радиан, запятая; 2 пико-радиана, запятая; \
         1 фемто-стерадиан, запятая; 2 атто-стерадиана; запятая; \
         1 гига-год, запятая; 2 мега-года, запятая; \
         1 зепто-угловая секунда, запятая; 2 иокто-угловые секунды; запятая; \
         1 кило-бит, запятая; 2 мега-бита, запятая; \
         1 гига-байт, запятая; 2 тера-байта, запятая; \
         1 тера-бод, запятая; 2 экса-бода");
}

#[test]
fn without_prefix_time() {
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
    test("ru", "SimpleSpeak", expr,
        "1 секунда, запятая, 2 секунды, запятая, \
         1 секунда, запятая, 2 секунды, запятая, \
         1 минута, запятая, 2 минуты, запятая, \
         1 минута, запятая, 2 минуты, запятая, \
         1 минута, запятая, 2 минуты, запятая, \
         1 час, запятая, 2 часа, запятая, \
         1 час, запятая, 2 часа, запятая, \
         1 час, запятая, 2 часа, запятая, \
         1 день, запятая, 2 дня, запятая, \
         1 день, запятая, 2 дня, запятая, \
         1 неделя, запятая, 2 недели, запятая, \
         1 неделя, запятая, 2 недели, запятая, \
         1 год, запятая, 2 года, запятая, \
         1 год, запятая, 2 года");
}

#[test]
fn without_prefix_angles() {
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
    test("ru", "SimpleSpeak", expr,
        "1 градус, запятая, 2 градуса, запятая, \
         1 градус, запятая, 2 градуса, запятая, \
         1 угловая минута, запятая; 2 угловые минуты, запятая, \
         1 угловая минута, запятая; 2 угловые минуты, запятая, \
         1 угловая минута, запятая; 2 угловые минуты, запятая, \
         1 угловая минута, запятая; 2 угловые минуты, запятая, \
         1 угловая секунда, запятая; 2 угловые секунды, запятая, \
         1 угловая секунда, запятая; 2 угловые секунды");
}

#[test]
fn without_prefix_distance() {
    let expr = r#"<math>
        <mn>1</mn><mi intent=":unit">au</mi><mo>,</mo><mn>2</mn><mi intent=":unit">au</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">ltyr</mi><mo>,</mo><mn>2</mn><mi intent=":unit">ltyr</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">pc</mi><mo>,</mo><mn>2</mn><mi intent=":unit">pc</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Å</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Å</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">Å</mi><mo>,</mo><mn>2</mn><mi intent=":unit">Å</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">fm</mi><mo>,</mo><mn>2</mn><mi intent=":unit">fm</mi>
    </math>"#;
    test("ru", "SimpleSpeak", expr,
        "1 астрономическая единица, запятая; 2 астрономические единицы, запятая, \
         1 световой год, запятая; 2 световых года, запятая, \
         1 парсек, запятая, 2 парсека, запятая, \
         1 ангстрем, запятая; 2 ангстрема, запятая, \
         1 ангстрем, запятая; 2 ангстрема, запятая, \
         1 ферми, запятая, 2 ферми");
}

#[test]
fn without_prefix_other() {
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
        <mn>1</mn><mi intent=":unit">℧</mi><mo>,</mo><mn>2</mn><mi intent=":unit">℧</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">dyn</mi><mo>,</mo><mn>2</mn><mi intent=":unit">dyn</mi><mo>,</mo>
        <mn>1</mn><mi intent=":unit">erg</mi><mo>,</mo><mn>2</mn><mi intent=":unit">erg</mi>
    </math>"#;
    test("ru", "SimpleSpeak", expr,
        "1 гектар, запятая; 2 гектара, запятая, \
         1 децибел, запятая; 2 децибела, запятая, \
         1 атмосфера, запятая; 2 атмосферы, запятая; \
         1 атомная единица массы, запятая; 2 атомные единицы массы, запятая, \
         1 бар, запятая, 2 бара, запятая, \
         1 калория, запятая; 2 калории, запятая, \
         1 кюри, запятая, 2 кюри, запятая, \
         1 град, запятая; 2 града, запятая, \
         1 моляр, запятая, 2 моляра, запятая, \
         1 рентген, запятая; 2 рентгена, запятая; \
         1 оборот в минуту, запятая; 2 оборота в минуту, запятая, \
         1 эм-аш-о, запятая, 2 эм-аш-о, запятая, \
         1 дина, запятая, 2 дины, запятая, \
         1 эрг, запятая, 2 эрга");
}

#[test]
fn without_prefix_powers_of_2() {
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
    test("ru", "SimpleSpeak", expr,
        "1 кибибит, запятая; 2 кибибита, запятая, \
         1 мебибит, запятая; 2 мебибита, запятая, \
         1 гибибит, запятая; 2 гибибита, запятая, \
         1 тебибит, запятая; 2 тебибита, запятая, \
         1 пебибит, запятая; 2 пебибита, запятая, \
         1 эксбибит, запятая; 2 эксбибита, запятая, \
         1 зебибит, запятая; 2 зебибита, запятая, \
         1 йобибит, запятая; 2 йобибита, запятая, \
         1 кибибайт, запятая; 2 кибибайта, запятая, \
         1 мебибайт, запятая; 2 мебибайта, запятая, \
         1 гибибайт, запятая; 2 гибибайта, запятая, \
         1 тебибайт, запятая; 2 тебибайта, запятая, \
         1 пебибайт, запятая; 2 пебибайта, запятая, \
         1 эксбибайт, запятая; 2 эксбибайта, запятая, \
         1 зебибайт, запятая; 2 зебибайта, запятая, \
         1 йобибайт, запятая; 2 йобибайта");
}

#[test]
fn si_other_numbers() {
    let expr = r#"<math><mn>1.0</mn><mi intent=":unit">l</mi><mo>,</mo>
                            <mn>2.0</mn><mo>&#xA0;</mo><mi intent=":unit">m</mi><mo>,</mo>
                            <mi>x</mi><mo>&#xA0;</mo><mi intent=":unit">ms</mi><mo>,</mo>
                            <mi>y</mi><mi intent=":unit">µs</mi><mo>,</mo>
                            <mi intent=":unit">dag</mi><mo>,</mo>
                            <mn>1235</mn><mi intent=":unit">daN</mi><mo>,</mo>
                            <mn>2.5</mn><mi intent=":unit">&#xB5;sec</mi><mo>,</mo>
                            <mn>32.34</mn><mi intent=":unit">mol</mi></math>"#;
    test("ru", "SimpleSpeak", expr,
        "1.0 литра, запятая; 2.0 метра, запятая; икс миллисекунд, запятая; игрек микросекунд, запятая, \
         дека-граммы, запятая; 1235 дека-ньютонов; запятая; 2.5 микросекунды; запятая; 32.34 моля");
}

#[test]
fn test_mtext_inference() {
    let expr = r#"<math><mo>[</mo>
                <mn>1</mn><mtext>t</mtext><mo>,</mo>
                <mn>2</mn><mtext>PA</mtext><mo>,</mo>
                <mn>3</mn><mtext>Pa</mtext><mo>,</mo>
                <mn>4.5</mn><mtext>mT</mtext>
            <mo>]</mo></math>"#;
    test("ru", "SimpleSpeak", expr,
        "открывающая квадратная скобка; 1 тонна, запятая; 2 пета-ампера, запятая, \
         3 паскаля, запятая; 4.5 милли-теслы; закрывающая квадратная скобка");
}
