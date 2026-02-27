/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use pdf_writer::{Finish, Name, Pdf, Ref, Str, Stream, Filter};
use pdf_writer::types::{CidFontType, FontFlags, SystemInfo, UnicodeCmap};
use ttf_parser::{Face, GlyphId};
use miniz_oxide::deflate::compress_to_vec_zlib;
use crate::{MM_PER_PT};

pub const LIBERATION_SANS_REGULAR_TTF: &[u8] =
    include_bytes!("../../../../assets/fonts/LiberationSansRegular.ttf");

pub const LIBERATION_SANS_BOLD_TTF: &[u8] =
    include_bytes!("../../../../assets/fonts/LiberationSansBold.ttf");

const SYSTEM_INFO: SystemInfo = SystemInfo {
    registry: Str(b"Adobe"),
    ordering: Str(b"Identity"),
    supplement: 0,
};

pub struct EmbeddedFont {
    pub type0_ref: Ref,
    pub face: Face<'static>, // Store this so we can encode/measure later
}

impl EmbeddedFont {
    /// Maps a string to 2-byte Big-Endian Glyph IDs for pdf-writer
    pub fn encode(&self, text: &str) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(text.len() * 2);
        for c in text.chars() {
            let gid = self.face.glyph_index(c).map(|g| g.0).unwrap_or(0);
            bytes.push((gid >> 8) as u8);
            bytes.push((gid & 0xFF) as u8);
        }
        bytes
    }

    /// Measures text width in PDF points for a given font size
    pub fn measure(&self, text: &str, size: f32) -> f32 {
        let mut width_units = 0.0;
        for c in text.chars() {
            let gid = self.face.glyph_index(c).unwrap_or(GlyphId(0));
            width_units += self.face.glyph_hor_advance(gid).unwrap_or(0) as f32;
        }
        // (Width in units * size in pts) / UnitsPerEm
        let width_pt = (width_units * size) / self.face.units_per_em() as f32;

        width_pt * MM_PER_PT
    }
}


//noinspection ALL
pub fn embed_ttf_font(pdf: &mut Pdf, next_id: &mut Ref, custom_font_name: Name, font_data: &'static [u8]) -> EmbeddedFont {

    let face = Face::parse(font_data, 0).expect("Invalid font data");
    // Allocate IDs
    let type0_ref = next_id.bump();
    let cid_ref = next_id.bump();
    let desc_ref = next_id.bump();
    let stream_ref = next_id.bump();
    let cmap_ref = next_id.bump();

    // Extract unicode cmap from .ttf
    let mut cmap = UnicodeCmap::<u16>::new(Name(b"Custom"), SYSTEM_INFO);
    for subtable in face
        .tables()
        .cmap
        .iter()
        .flat_map(|c| c.subtables) {
            if subtable.is_unicode() {
                subtable.codepoints(|cp| {
                    if let (Some(c), Some(gid)) = (char::from_u32(cp), subtable.glyph_index(cp)) {
                        cmap.pair(gid.0, c);
                    }
                });
                break;
            }
    }
    pdf.stream(cmap_ref, &cmap.finish());

    let units_per_em = face.units_per_em();

    // Widths
    // PDF CIDFont widths are: [first_gid [w1 w2 w3]]
    let mut widths = Vec::new();
    for gid in 0..face.number_of_glyphs() {
        let width = face.glyph_hor_advance(GlyphId(gid)).unwrap_or(0);
        let pdf_width = (width as f32 * 1000.0) / units_per_em as f32;
        widths.push(pdf_width.round());
    }


    let compressed = compress_to_vec_zlib(font_data, 6);
    let mut stream: Stream = pdf.stream(stream_ref, &compressed);
    stream.pair(Name(b"Length1"), font_data.len() as i32);
    stream.filter(Filter::FlateDecode);
    stream.finish();

    // Font Descriptor
    // 1. Font Descriptor (Metadata)
    let mut desc = pdf.font_descriptor(desc_ref);
    let bbox = face.global_bounding_box();
    desc.name(custom_font_name);
    desc.bbox(pdf_writer::Rect::new(bbox.x_min as f32, bbox.y_min as f32, bbox.x_max as f32, bbox.y_max as f32));
    desc.ascent(face.ascender() as f32);
    desc.descent(face.descender() as f32);
    desc.cap_height(face.capital_height().unwrap_or(700) as f32);
    desc.italic_angle(face.italic_angle());
    desc.flags(FontFlags::SYMBOLIC); // Adjust based on face.is_monospaced(), etc.
    desc.stem_v(80.0);
    if let Some(cap) = face.capital_height() {
        desc.cap_height(cap as f32);
    } else {
        desc.cap_height(700.0); // Standard fallback
    }
    desc.font_file2(stream_ref);
    desc.finish();

    // CID Font
    let mut cid = pdf.cid_font(cid_ref);
    cid.subtype(CidFontType::Type2);
    cid.base_font(custom_font_name);
    cid.system_info(SYSTEM_INFO);
    cid.font_descriptor(desc_ref);
    // Add the Identity mapping manually here
    cid.pair(Name(b"CIDToGIDMap"), Name(b"Identity"));

    let mut w = cid.widths();
    w.consecutive(0, widths);
    w.finish();
    cid.finish();

    let mut type0 = pdf.type0_font(type0_ref);
    type0.base_font(custom_font_name);
    type0.encoding_predefined(Name(b"Identity-H"));
    type0.descendant_font(cid_ref);
    type0.to_unicode(cmap_ref);
    type0.finish();

    EmbeddedFont {
        type0_ref,
        face,
    }
}