//! Regression fixtures from the Greek Math Reader 2026.08.2 exported registry.
//! The source offers these fixtures under MathCAT terms; linguistic review remains separate.
//! NVDA Greek Math (Greek Math Reader) by Bouronikos Christos (cbouronikos@uth.gr)
use crate::common::*;
use anyhow::Result;

#[test]
fn absolute_value() -> Result<()> {
    // Export status: reviewed; assert the name and every argument.
    let expr = r#"<math><mrow intent="absolute-value:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "η απόλυτη τιμή του 2")
}

#[test]
fn acceleration() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="acceleration:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "επιτάχυνση του 2")
}

#[test]
fn adjoint() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="adjoint:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "συζυγής ανάστροφος του 2")
}

#[test]
fn angular_momentum() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="angular-momentum:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "στροφορμή του 2")
}

#[test]
fn anticommutator() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="anticommutator:function($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "αντιμεταθέτης των 2 και 3")
}

#[test]
fn asymptotic_equivalence() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="asymptotic-equivalence:infix($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "2 είναι ασυμπτωτικά ισοδύναμο με 3")
}

#[test]
fn augmented_matrix() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="augmented-matrix:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "επαυξημένος πίνακας του 2")
}

#[test]
fn block_matrix() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="block-matrix:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "πίνακας κατά μπλοκ του 2")
}

#[test]
fn boundary_condition() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="boundary-condition:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "συνοριακή συνθήκη, 2")
}

#[test]
fn bounded_operator() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="bounded-operator:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "φραγμένος τελεστής του 2")
}

#[test]
fn bra() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="bra:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "μπρα του 2")
}

#[test]
fn braket() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="braket:function($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "εσωτερικό γινόμενο 2 με 3")
}

#[test]
fn cardinality() -> Result<()> {
    // Export status: reviewed; assert the name and every argument.
    let expr = r#"<math><mrow intent="cardinality:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "πληθάριθμος του 2")
}

#[test]
fn classical_hamiltonian() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="classical-hamiltonian:function($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "χαμιλτονιανή συνάρτηση των 2 και 3")
}

#[test]
fn closed_interval() -> Result<()> {
    // Export status: reviewed; assert the name and every argument.
    let expr = r#"<math><mrow intent="closed-interval:function($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "κλειστό διάστημα από 2 έως 3")
}

#[test]
fn closed_open_interval() -> Result<()> {
    // Export status: reviewed; assert the name and every argument.
    let expr = r#"<math><mrow intent="closed-open-interval:function($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "διάστημα κλειστό αριστερά ανοιχτό δεξιά από 2 έως 3")
}

#[test]
fn commutator() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="commutator:function($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "μεταθέτης των 2 και 3")
}

#[test]
fn conditional_expectation() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="conditional-expectation:function($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "δεσμευμένη αναμενόμενη τιμή του 2 δεδομένου του 3")
}

#[test]
fn confidence_interval() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="confidence-interval:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "διάστημα εμπιστοσύνης του 2")
}

#[test]
fn contravariant_index() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="contravariant-index:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "ανταλλοίωτος δείκτης του 2")
}

#[test]
fn coordinate() -> Result<()> {
    // Export status: reviewed; assert the name and every argument.
    let expr = r#"<math><mrow intent="coordinate:function($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "σημείο με συντεταγμένες 2 και 3")
}

#[test]
fn covariance() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="covariance:function($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "συνδιακύμανση των 2 και 3")
}

#[test]
fn covariant_index() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="covariant-index:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "συναλλοίωτος δείκτης του 2")
}

#[test]
fn cross_product() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="cross-product:infix($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "2 διανυσματικό γινόμενο 3")
}

#[test]
fn curl() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="curl:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "στροβιλισμός του 2")
}

#[test]
fn diagonal_matrix() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="diagonal-matrix:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "διαγώνιος πίνακας του 2")
}

#[test]
fn differential_form() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="differential-form:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "διαφορική μορφή του 2")
}

#[test]
fn dirac_delta() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="dirac-delta:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "δέλτα του Ντιράκ στο 2")
}

#[test]
fn directional_derivative() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="directional-derivative:function($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "παράγωγος του 2 κατά την κατεύθυνση 3")
}

#[test]
fn divergence() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="divergence:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "απόκλιση του 2")
}

#[test]
fn dot_product() -> Result<()> {
    // Export status: reviewed; assert the name and every argument.
    let expr = r#"<math><mrow intent="dot-product:infix($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "2 εσωτερικό γινόμενο 3")
}

#[test]
fn eigenvalue() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="eigenvalue:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "ιδιοτιμή του 2")
}

#[test]
fn eigenvector() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="eigenvector:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "ιδιοδιάνυσμα του 2")
}

#[test]
fn electric_field() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="electric-field:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "ηλεκτρικό πεδίο του 2")
}

#[test]
fn entropy() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="entropy:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "εντροπία του 2")
}

#[test]
fn estimator() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="estimator:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "εκτιμητής του 2")
}

#[test]
fn evaluation() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="evaluation:function($a0,$a1,$a2)"><mn arg="a0">2</mn><mn arg="a1">3</mn><mn arg="a2">4</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "2, υπολογισμένο από 3 έως 4")
}

#[test]
fn expectation() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="expectation:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "αναμενόμενη τιμή του 2")
}

#[test]
fn exterior_product() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="exterior-product:infix($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "2 εξωτερικό γινόμενο 3")
}

#[test]
fn field_structure() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="field-structure:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "σώμα του 2")
}

#[test]
fn four_momentum() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="four-momentum:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "τετραορμή του 2")
}

#[test]
fn four_vector() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="four-vector:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "τετραδιάνυσμα του 2")
}

#[test]
fn fourier_transform() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="fourier-transform:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "μετασχηματισμός Φουριέ της 2")
}

#[test]
fn gamma_function() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="gamma-function:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "συνάρτηση γάμα του 2")
}

#[test]
fn generalized_function() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="generalized-function:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "γενικευμένη συνάρτηση του 2")
}

#[test]
fn gradient() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="gradient:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "ανάδελτα του 2")
}

#[test]
fn group() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="group:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "ομάδα του 2")
}

#[test]
fn hamiltonian() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="hamiltonian:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "χαμιλτονιανός τελεστής του 2")
}

#[test]
fn hessian() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="hessian:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "εσσιανός πίνακας του 2")
}

#[test]
fn hypothesis_test() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="hypothesis-test:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "έλεγχος υπόθεσης του 2")
}

#[test]
fn ideal() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="ideal:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "ιδεώδες του 2")
}

#[test]
fn identity_matrix() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="identity-matrix:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "μοναδιαίος πίνακας τάξης 2")
}

#[test]
fn independence() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="independence:infix($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "2 ανεξάρτητο από 3")
}

#[test]
fn initial_condition() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="initial-condition:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "αρχική συνθήκη, 2")
}

#[test]
fn interval() -> Result<()> {
    // Export status: reviewed; assert the name and every argument.
    let expr = r#"<math><mrow intent="interval:function($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "διάστημα από 2 έως 3")
}

#[test]
fn jacobian() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="jacobian:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "ιακωβιανός πίνακας του 2")
}

#[test]
fn ket() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="ket:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "κετ του 2")
}

#[test]
fn lagrangian() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="lagrangian:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "λαγκρανζιανή συνάρτηση του 2")
}

#[test]
fn laplace_transform() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="laplace-transform:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "μετασχηματισμός Λαπλάς του 2")
}

#[test]
fn laplacian() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="laplacian:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "λαπλασιανή του 2")
}

#[test]
fn line_integral() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="line-integral:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "επικαμπύλιο ολοκλήρωμα του 2")
}

#[test]
fn magnetic_field() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="magnetic-field:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "μαγνητικό πεδίο του 2")
}

#[test]
fn manifold() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="manifold:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "πολλαπλότητα του 2")
}

#[test]
fn material_derivative() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="material-derivative:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "υλική παράγωγος του 2")
}

#[test]
fn matrix_element() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="matrix-element:function($a0,$a1,$a2)"><mn arg="a0">2</mn><mn arg="a1">3</mn><mn arg="a2">4</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "στοιχείο πίνακα με μπρα 2 3 κετ 4")
}

#[test]
fn measure() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="measure:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "μέτρο του 2")
}

#[test]
fn metric_tensor() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="metric-tensor:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "μετρικός τανυστής του 2")
}

#[test]
fn modular_congruence() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="modular-congruence:function($a0,$a1,$a2)"><mn arg="a0">2</mn><mn arg="a1">3</mn><mn arg="a2">4</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "2 ισότιμο με 3 μόντουλο 4")
}

#[test]
fn momentum() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="momentum:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "ορμή του 2")
}

#[test]
fn morphism() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="morphism:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "μορφισμός του 2")
}

#[test]
fn multiple_integral() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="multiple-integral:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "πολλαπλό ολοκλήρωμα του 2")
}

#[test]
fn norm() -> Result<()> {
    // Export status: reviewed; assert the name and every argument.
    let expr = r#"<math><mrow intent="norm:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "νόρμα του 2")
}

#[test]
fn normal_distribution() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="normal-distribution:function($a0,$a1,$a2)"><mn arg="a0">2</mn><mn arg="a1">3</mn><mn arg="a2">4</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "2 ακολουθεί κανονική κατανομή με μέση τιμή 3 και διακύμανση 4")
}

#[test]
fn open_closed_interval() -> Result<()> {
    // Export status: reviewed; assert the name and every argument.
    let expr = r#"<math><mrow intent="open-closed-interval:function($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "διάστημα ανοιχτό αριστερά κλειστό δεξιά από 2 έως 3")
}

#[test]
fn open_interval() -> Result<()> {
    // Export status: reviewed; assert the name and every argument.
    let expr = r#"<math><mrow intent="open-interval:function($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "ανοιχτό διάστημα από 2 έως 3")
}

#[test]
fn open_set() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="open-set:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "ανοιχτό σύνολο του 2")
}

#[test]
fn ordinary_derivative() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="ordinary-derivative:function($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "παράγωγος του 2 ως προς 3")
}

#[test]
fn partial_derivative() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="partial-derivative:function($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "μερική παράγωγος του 2 ως προς 3")
}

#[test]
fn partition_function() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="partition-function:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "συνάρτηση επιμερισμού του 2")
}

#[test]
fn point_definition() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="point-definition:function($a0,$a1,$a2)"><mn arg="a0">2</mn><mn arg="a1">3</mn><mn arg="a2">4</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "το σημείο 2 έχει συντεταγμένες 3 και 4")
}

#[test]
fn position_vector() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="position-vector:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "διάνυσμα θέσης του 2")
}

#[test]
fn power() -> Result<()> {
    // Export status: reviewed; assert the name and every argument.
    let expr = r#"<math><mrow intent="power:function($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "2 στον κύβο")
}

#[test]
fn probability() -> Result<()> {
    // Export status: reviewed; assert the name and every argument.
    let expr = r#"<math><mrow intent="probability:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "πιθανότητα του 2")
}

#[test]
fn proper_time() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="proper-time:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "ιδιοχρόνος του 2")
}

#[test]
fn quadratic_form() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="quadratic-form:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "τετραγωνική μορφή του 2")
}

#[test]
fn quantum_adjoint() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="quantum-adjoint:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "ερμιτιανός συζυγής του 2")
}

#[test]
fn quantum_operator() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="quantum-operator:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "κβαντικός τελεστής του 2")
}

#[test]
fn quotient_group() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="quotient-group:function($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "ομάδα πηλίκο του 2 ως προς 3")
}

#[test]
fn ring() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="ring:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "δακτύλιος του 2")
}

#[test]
fn semantic_entailment() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="semantic-entailment:infix($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "2 συνεπάγεται σημασιολογικά 3")
}

#[test]
fn standard_deviation() -> Result<()> {
    // Export status: reviewed; assert the name and every argument.
    let expr = r#"<math><mrow intent="standard-deviation:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "τυπική απόκλιση του 2")
}

#[test]
fn stochastic_integral() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="stochastic-integral:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "στοχαστικό ολοκλήρωμα του 2")
}

#[test]
fn stochastic_process() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="stochastic-process:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "στοχαστική διαδικασία του 2")
}

#[test]
fn surface_integral() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="surface-integral:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "επιφανειακό ολοκλήρωμα του 2")
}

#[test]
fn tensor_product() -> Result<()> {
    // Export status: reviewed; assert the name and every argument.
    let expr = r#"<math><mrow intent="tensor-product:infix($a0,$a1)"><mn arg="a0">2</mn><mn arg="a1">3</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "2 τανυστικό γινόμενο 3")
}

#[test]
fn topological_space() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="topological-space:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "τοπολογικός χώρος του 2")
}

#[test]
fn torque() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="torque:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "ροπή του 2")
}

#[test]
fn transpose() -> Result<()> {
    // Export status: reviewed; assert the name and every argument.
    let expr = r#"<math><mrow intent="transpose:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "ανάστροφος του 2")
}

#[test]
fn variance() -> Result<()> {
    // Export status: reviewed; assert the name and every argument.
    let expr = r#"<math><mrow intent="variance:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "διακύμανση του 2")
}

#[test]
fn velocity() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="velocity:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "ταχύτητα του 2")
}

#[test]
fn wavefunction() -> Result<()> {
    // Export status: source-checked-pending-expert-review; assert the name and every argument.
    let expr = r#"<math><mrow intent="wavefunction:function($a0)"><mn arg="a0">2</mn></mrow></math>"#;
    test("el", "GreekMathReader", expr, "κυματοσυνάρτηση του 2")
}
