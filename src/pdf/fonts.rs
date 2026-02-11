/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use pdf_writer::{Finish, Name, Pdf, Ref, Str};
use pdf_writer::types::{CidFontType, FontFlags, SystemInfo};

const LIBERATION_SANS_REGULAR_TTF: &[u8] =
    include_bytes!("../../assets/fonts/LiberationSans-Regular.ttf");

const LIBERATION_SANS_BOLD_TTF: &[u8] =
    include_bytes!("../../assets/fonts/LiberationSans-Bold.ttf");

const SYSTEM_INFO: SystemInfo = SystemInfo {
    registry: Str(b"Adobe"),
    ordering: Str(b"Identity"),
    supplement: 0,
};

pub struct Fonts {
    pub regular: Ref,
    pub bold: Ref,
}

pub fn embed_fonts(pdf: &mut Pdf) -> Fonts {
    // Object IDs (fixed on purpose)
    let reg_file = Ref::new(20);
    let reg_desc = Ref::new(21);
    let reg_cid  = Ref::new(22);
    let reg_font = Ref::new(23);

    let bold_file = Ref::new(24);
    let bold_desc = Ref::new(25);
    let bold_cid  = Ref::new(26);
    let bold_font = Ref::new(27);

    let reg_tounicode = Ref::new(28);
    let bold_tounicode = Ref::new(29);

    pdf.stream(reg_file, LIBERATION_SANS_REGULAR_TTF);
    pdf.stream(bold_file, LIBERATION_SANS_BOLD_TTF);

    pdf.stream(reg_tounicode, TO_UNICODE_CMAP);
    pdf.stream(bold_tounicode, TO_UNICODE_CMAP);


    pdf.font_descriptor(reg_desc)
        .flags(FontFlags::NON_SYMBOLIC)
        .ascent(800f32)
        .descent(-200f32)
        .cap_height(700f32)
        .italic_angle(0f32)
        .stem_v(80f32)
        .font_file2(reg_file)
        .finish();

    pdf.font_descriptor(bold_desc)
        .flags(FontFlags::NON_SYMBOLIC | FontFlags::FORCE_BOLD)
        .ascent(800f32)
        .descent(-200f32)
        .cap_height(700f32)
        .italic_angle(0f32)
        .stem_v(120f32)
        .font_file2(bold_file)
        .finish();

    pdf.cid_font(reg_cid)
        .subtype(CidFontType::Type2)
        .base_font(Name(b"LiberationSans"))
        .system_info(SYSTEM_INFO)
        .font_descriptor(reg_desc)
        .finish();

    pdf.cid_font(bold_cid)
        .subtype(CidFontType::Type2)
        .base_font(Name(b"LiberationSans-Bold"))
        .system_info(SYSTEM_INFO)
        .font_descriptor(bold_desc)
        .finish();

    pdf.type0_font(reg_font)
        .base_font(Name(b"LiberationSans"))
        .encoding_predefined(Name(b"Identity-H"))
        .descendant_font(reg_cid)
        .to_unicode(reg_tounicode)
        .finish();

    pdf.type0_font(bold_font)
        .base_font(Name(b"LiberationSans-Bold"))
        .encoding_predefined(Name(b"Identity-H"))
        .descendant_font(bold_cid)
        .to_unicode(bold_tounicode)
        .finish();

    Fonts {
        regular: reg_font,
        bold: bold_font,
    }
}

const TO_UNICODE_CMAP: &[u8] = br#"
/CIDInit /ProcSet findresource begin
12 dict begin
begincmap
/CIDSystemInfo
<< /Registry (Adobe)
   /Ordering (UCS)
   /Supplement 0
>> def
/CMapName /LiberationSans-ToUnicode def
/CMapType 2 def

1 begincodespacerange
<0000> <00FF>
endcodespacerange

1 beginbfrange
<0020> <007E> <0020>
endbfrange

endcmap
CMapName currentdict /CMap defineresource pop
end
end
"#;



