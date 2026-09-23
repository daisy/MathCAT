//! Regression tests for German ClearSpeak interval endpoint wording.
//! Each endpoint combination should use one German connector and no untranslated words.

use crate::common::*;
use anyhow::Result;

#[test]
fn interval_open_open() -> Result<()> {
    let expr = "<math>
        <mrow><mo>(</mo>
            <mrow><mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak(
        "de",
        "ClearSpeak_Paren",
        "Interval",
        expr,
        "das Intervall c bis d, nicht einschließlich c und nicht einschließlich d",
    )
}

#[test]
fn interval_open_closed() -> Result<()> {
    let expr = "<math>
        <mrow><mo>(</mo>
            <mrow><mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>]</mo></mrow>
    </math>";
    test_ClearSpeak(
        "de",
        "ClearSpeak_Paren",
        "Interval",
        expr,
        "das Intervall c bis d, nicht einschließlich c aber einschließlich d",
    )
}

#[test]
fn interval_closed_open() -> Result<()> {
    let expr = "<math>
        <mrow><mo>[</mo>
            <mrow><mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak(
        "de",
        "ClearSpeak_Paren",
        "Interval",
        expr,
        "das Intervall c bis d, einschließlich c aber nicht einschließlich d",
    )
}

#[test]
fn interval_closed_closed() -> Result<()> {
    let expr = "<math>
        <mrow><mo>[</mo>
            <mrow><mi>c</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>]</mo></mrow>
    </math>";
    test_ClearSpeak(
        "de",
        "ClearSpeak_Paren",
        "Interval",
        expr,
        "das Intervall c bis d, einschließlich c und d",
    )
}

#[test]
fn interval_negative_infinity_to_open_endpoint() -> Result<()> {
    let expr = "<math>
        <mrow><mo>(</mo>
            <mrow><mo>-</mo><mi>∞</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak(
        "de",
        "ClearSpeak_Paren",
        "Interval",
        expr,
        "das Intervall minus unendlich bis d, nicht einschließlich d",
    )
}

#[test]
fn interval_negative_infinity_to_closed_endpoint() -> Result<()> {
    let expr = "<math>
        <mrow><mo>(</mo>
            <mrow><mo>-</mo><mi>∞</mi><mo>,</mo><mi>d</mi></mrow>
        <mo>]</mo></mrow>
    </math>";
    test_ClearSpeak(
        "de",
        "ClearSpeak_Paren",
        "Interval",
        expr,
        "das Intervall minus unendlich bis d, einschließlich d",
    )
}

#[test]
fn interval_open_endpoint_to_infinity() -> Result<()> {
    let expr = "<math>
        <mrow><mo>(</mo>
            <mrow><mi>c</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak(
        "de",
        "ClearSpeak_Paren",
        "Interval",
        expr,
        "das Intervall c bis unendlich, nicht einschließlich c",
    )
}

#[test]
fn interval_closed_endpoint_to_infinity() -> Result<()> {
    let expr = "<math>
        <mrow><mo>[</mo>
            <mrow><mi>c</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak(
        "de",
        "ClearSpeak_Paren",
        "Interval",
        expr,
        "das Intervall c bis unendlich, einschließlich c",
    )
}

#[test]
fn interval_between_infinities_has_no_endpoint_qualifier() -> Result<()> {
    let expr = "<math>
        <mrow><mo>(</mo>
            <mrow><mo>-</mo><mi>∞</mi><mo>,</mo><mi>∞</mi></mrow>
        <mo>)</mo></mrow>
    </math>";
    test_ClearSpeak(
        "de",
        "ClearSpeak_Paren",
        "Interval",
        expr,
        "das Intervall minus unendlich bis unendlich",
    )
}
