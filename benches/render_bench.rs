/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

//! Benchmarks the actual claim this crate makes (see README): generating a
//! full Swiss QR-bill should be fast. Measures the two public render entry
//! points end-to-end - QR code generation, layout, font metrics, and
//! encoding all included - for tests/data/valid_input/normal_slip_valid.toml,
//! a representative bill with a debtor address, reference, and message
//! populated, not the cheapest empty-fields case.
//!
//! Run with `cargo bench`; HTML report lands in target/criterion/report/index.html.

use std::convert::TryFrom;
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use swiss_qrust::pdf::render_bill_to_pdf;
use swiss_qrust::svg::render_bill_to_svg;
use swiss_qrust::{BillData, InputBill, Language};

fn sample_bill() -> BillData {
    let toml = std::fs::read_to_string("tests/data/valid_input/normal_slip_valid.toml").expect("fixture file missing");
    let input: InputBill = toml::from_str(&toml).expect("fixture file is invalid TOML");
    BillData::try_from(input).expect("fixture bill data is invalid")
}

fn bench_pdf(c: &mut Criterion) {
    let bill = sample_bill();
    c.bench_function("render_bill_to_pdf", |b| {
        b.iter(|| render_bill_to_pdf(black_box(&bill), black_box(Language::De)).unwrap());
    });
}

fn bench_svg(c: &mut Criterion) {
    let bill = sample_bill();
    c.bench_function("render_bill_to_svg", |b| {
        b.iter(|| render_bill_to_svg(black_box(&bill), black_box(Language::De)).unwrap());
    });
}

criterion_group!(benches, bench_pdf, bench_svg);
criterion_main!(benches);
