use crate::types::{Address, Dictionary, DocumentPropertiesDefault, Language};
use printpdf::{
    graphics::{Line, LinePoint},
    Color, FontId, Mm, Op, PaintMode, ParsedFont, PdfDocument, Point, Polygon, PolygonRing, Pt,
    RawImage, Rgb, TextItem, WindingOrder, XObjectTransform,
};
use resvg::{tiny_skia, usvg};
use rust_decimal::Decimal;
use unicode_script::{Script, UnicodeScript};
use usvg::TreeParsing;

use super::errors::PdfError;

pub struct FontBundle {
    pub normal: FontId,
    pub bold: FontId,
    pub normal_fallback: Option<FontId>,
    pub bold_fallback: Option<FontId>,
}

#[derive(Debug)]
struct TextSegment {
    text: String,
    script: Script,
}

fn segment_text_by_script(text: &str) -> Vec<TextSegment> {
    let mut segments: Vec<TextSegment> = Vec::new();
    let mut current_segment = String::new();
    let mut last_script = Script::Common;

    for ch in text.chars() {
        let current_script = ch.script();

        if current_segment.is_empty() {
            current_segment.push(ch);
            last_script = current_script;
        } else if current_script == last_script || current_script == Script::Common {
            current_segment.push(ch);
        } else {
            segments.push(TextSegment {
                text: current_segment,
                script: last_script,
            });
            current_segment = ch.to_string();
            last_script = current_script;
        }
    }

    if !current_segment.is_empty() {
        segments.push(TextSegment {
            text: current_segment,
            script: last_script,
        });
    }

    segments
}

fn svg_to_rgba_bytes(svg_data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, PdfError> {
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_data(svg_data, &opt)
        .map_err(|e| PdfError::PrintPdfError(format!("SVG parsing error: {}", e)))?;

    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| PdfError::PrintPdfError("Failed to create pixmap".to_string()))?;

    resvg::render(
        &tree,
        tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );

    Ok(pixmap.data().to_vec())
}

pub fn draw_logo(
    doc: &mut PdfDocument,
    x: i32,
    y: i32,
    image_data: Option<&[u8]>,
    width_mm: f32,
    height_mm: f32,
) -> Result<Vec<Op>, PdfError> {
    match image_data {
        Some(data) => {
            // Check if it's SVG or PNG/other image format
            if data.starts_with(b"<?xml") || data.starts_with(b"<svg") {
                // SVG processing
                let pixel_width = (width_mm * 8.0) as u32; // ~200 DPI
                let pixel_height = (height_mm * 8.0) as u32;

                let rgba_data = svg_to_rgba_bytes(data, pixel_width, pixel_height)?;

                // Create image from RGBA data
                let dynamic_image = image::DynamicImage::ImageRgba8(
                    image::RgbaImage::from_raw(pixel_width, pixel_height, rgba_data).ok_or_else(
                        || {
                            PdfError::PrintPdfError(
                                "Failed to create image from RGBA data".to_string(),
                            )
                        },
                    )?,
                );

                // Convert to raw bytes for RawImage
                let mut png_bytes = Vec::new();
                dynamic_image
                    .write_to(
                        &mut std::io::Cursor::new(&mut png_bytes),
                        image::ImageFormat::Png,
                    )
                    .map_err(|e| {
                        PdfError::PrintPdfError(format!("Failed to encode image to PNG: {}", e))
                    })?;

                // Decode as RawImage and add to document
                let mut warnings = Vec::new();
                let raw_image =
                    RawImage::decode_from_bytes(&png_bytes, &mut warnings).map_err(|e| {
                        PdfError::PrintPdfError(format!("Failed to decode RawImage: {}", e))
                    })?;

                let image_xobject_id = doc.add_image(&raw_image);

                // Create transform for positioning and scaling
                let transform = XObjectTransform {
                    translate_x: Some(Mm(x as f32 * 0.352778).into()),
                    translate_y: Some(Mm(y as f32 * 0.352778).into()),
                    scale_x: Some(1.0), // Scale factor
                    scale_y: Some(1.0), // Scale factor
                    ..Default::default()
                };

                Ok(vec![Op::UseXobject {
                    id: image_xobject_id,
                    transform,
                }])
            } else {
                // Direct PNG/JPEG/other image format processing
                let mut warnings = Vec::new();
                let raw_image = RawImage::decode_from_bytes(data, &mut warnings).map_err(|e| {
                    PdfError::PrintPdfError(format!("Failed to decode RawImage: {}", e))
                })?;

                let image_xobject_id = doc.add_image(&raw_image);

                // Create transform for positioning and scaling
                let transform = XObjectTransform {
                    translate_x: Some(Mm(x as f32 * 0.352778).into()),
                    translate_y: Some(Mm(y as f32 * 0.352778).into()),
                    scale_x: Some(1.0), // Scale factor
                    scale_y: Some(1.0), // Scale factor
                    ..Default::default()
                };

                Ok(vec![Op::UseXobject {
                    id: image_xobject_id,
                    transform,
                }])
            }
        }
        None => Ok(vec![]),
    }
}

pub fn format_decimal(amount: Decimal, currency: &str) -> String {
    format!("{} {:.2}", currency, amount)
}

pub fn draw_text(x: i32, y: i32, text: &str, font_size: f32, fonts: &FontBundle) -> Vec<Op> {
    let segments = segment_text_by_script(text);
    let mut ops = vec![
        Op::StartTextSection,
        Op::SetTextCursor {
            pos: Point::new(Mm(x as f32 * 0.352778), Mm(y as f32 * 0.352778)),
        },
    ];

    for segment in segments {
        let (font_id, text_to_write) = match segment.script {
            Script::Thai => (fonts.normal.clone(), segment.text),
            Script::Latin | Script::Common | _ => {
                if fonts.normal_fallback.is_some() {
                    (
                        fonts.normal_fallback.as_ref().unwrap().clone(),
                        segment.text,
                    )
                } else {
                    (fonts.normal.clone(), segment.text)
                }
            }
        };

        ops.push(Op::SetFontSize {
            font: font_id.clone(),
            size: Pt(font_size),
        });
        ops.push(Op::WriteText {
            font: font_id,
            items: vec![TextItem::Text(text_to_write)],
        });
    }

    ops.push(Op::EndTextSection);
    ops
}

pub fn draw_bold_text(x: i32, y: i32, text: &str, font_size: f32, fonts: &FontBundle) -> Vec<Op> {
    let segments = segment_text_by_script(text);
    let mut ops = vec![
        Op::StartTextSection,
        Op::SetTextCursor {
            pos: Point::new(Mm(x as f32 * 0.352778), Mm(y as f32 * 0.352778)),
        },
    ];

    for segment in segments {
        let (font_id, text_to_write) = match segment.script {
            Script::Thai => (fonts.bold.clone(), segment.text),
            Script::Latin | Script::Common | _ => {
                if fonts.bold_fallback.is_some() {
                    (fonts.bold_fallback.as_ref().unwrap().clone(), segment.text)
                } else {
                    (fonts.bold.clone(), segment.text)
                }
            }
        };

        ops.push(Op::SetFontSize {
            font: font_id.clone(),
            size: Pt(font_size),
        });
        ops.push(Op::WriteText {
            font: font_id,
            items: vec![TextItem::Text(text_to_write)],
        });
    }

    ops.push(Op::EndTextSection);
    ops
}

pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len - 3])
    }
}

pub fn load_fonts(
    doc: &mut PdfDocument,
    language: Option<Language>,
    custom_font_normal_path: Option<&str>,
    custom_font_bold_path: Option<&str>,
) -> Result<FontBundle, PdfError> {
    match language {
        Some(Language::Thai) => {
            // For Thai language, use NotoSans fonts which have good Unicode coverage
            // including both Thai and English characters
            const NOTO_SANS_REGULAR: &[u8] = include_bytes!("../../fonts/NotoSans-Regular.ttf");
            const NOTO_SANS_BOLD: &[u8] = include_bytes!("../../fonts/NotoSans-SemiBold.ttf");
            // Also load Thai-specific fonts for optimal Thai rendering
            const NOTO_SANS_THAI_REGULAR: &[u8] =
                include_bytes!("../../fonts/NotoSansThai-Regular.ttf");
            const NOTO_SANS_THAI_BOLD: &[u8] =
                include_bytes!("../../fonts/NotoSansThai-SemiBold.ttf");

            // Use Thai fonts as primary for proper Thai character rendering
            let font_thai_normal =
                ParsedFont::from_bytes(NOTO_SANS_THAI_REGULAR, 0, &mut Vec::new()).ok_or(
                    PdfError::PrintPdfError("Failed to load Thai normal font".to_string()),
                )?;
            let font_thai_bold = ParsedFont::from_bytes(NOTO_SANS_THAI_BOLD, 0, &mut Vec::new())
                .ok_or(PdfError::PrintPdfError(
                    "Failed to load Thai bold font".to_string(),
                ))?;
            // Regular NotoSans as fallback for English/Latin characters
            let font_normal = ParsedFont::from_bytes(NOTO_SANS_REGULAR, 0, &mut Vec::new()).ok_or(
                PdfError::PrintPdfError("Failed to load normal font".to_string()),
            )?;
            let font_bold = ParsedFont::from_bytes(NOTO_SANS_BOLD, 0, &mut Vec::new()).ok_or(
                PdfError::PrintPdfError("Failed to load bold font".to_string()),
            )?;

            let font_thai_normal_id = doc.add_font(&font_thai_normal);
            let font_thai_bold_id = doc.add_font(&font_thai_bold);
            let font_normal_id = doc.add_font(&font_normal);
            let font_bold_id = doc.add_font(&font_bold);

            Ok(FontBundle {
                normal: font_thai_normal_id,
                bold: font_thai_bold_id,
                normal_fallback: Some(font_normal_id),
                bold_fallback: Some(font_bold_id),
            })
        }
        _ => {
            // Default fonts for all other languages (en, de, fr, es, pt, it)
            // Use custom fonts if provided, otherwise use built-in NotoSans
            let normal_font_data = match custom_font_normal_path {
                Some(path) => std::fs::read(path).map_err(|e| {
                    PdfError::PrintPdfError(format!("Failed to read font file {}: {}", path, e))
                })?,
                None => include_bytes!("../../fonts/NotoSans-Regular.ttf").to_vec(),
            };
            let bold_font_data = match custom_font_bold_path {
                Some(path) => std::fs::read(path).map_err(|e| {
                    PdfError::PrintPdfError(format!("Failed to read font file {}: {}", path, e))
                })?,
                None => include_bytes!("../../fonts/NotoSans-SemiBold.ttf").to_vec(),
            };

            let font_normal = ParsedFont::from_bytes(&normal_font_data, 0, &mut Vec::new()).ok_or(
                PdfError::PrintPdfError("Failed to load normal font".to_string()),
            )?;
            let font_bold = ParsedFont::from_bytes(&bold_font_data, 0, &mut Vec::new()).ok_or(
                PdfError::PrintPdfError("Failed to load bold font".to_string()),
            )?;

            let font_normal_id = doc.add_font(&font_normal);
            let font_bold_id = doc.add_font(&font_bold);

            Ok(FontBundle {
                normal: font_normal_id,
                bold: font_bold_id,
                normal_fallback: None,
                bold_fallback: None,
            })
        }
    }
}

pub fn draw_address(
    pdf_properties: &DocumentPropertiesDefault,
    translation: &Dictionary,
    ops: &mut Vec<Op>,
    x: i32,
    y: i32,
    title: &str,
    address: &Address,
    fonts: &FontBundle,
) -> i32 {
    let mut current_y = y;

    ops.extend(draw_bold_text(
        x,
        current_y,
        title,
        pdf_properties.font_size_label,
        fonts,
    ));
    current_y -= 12;

    ops.extend(draw_text(
        x,
        current_y,
        &address.recipient_name.clone().unwrap_or_default(),
        pdf_properties.font_size_body,
        fonts,
    ));
    current_y -= 12;

    if let Some(company) = &address.company_name {
        if !company.is_empty() {
            ops.extend(draw_text(
                x,
                current_y,
                company,
                pdf_properties.font_size_body,
                fonts,
            ));
            current_y -= 12;
        }
    }

    ops.extend(draw_text(
        x,
        current_y,
        &address.street,
        pdf_properties.font_size_body,
        fonts,
    ));
    current_y -= 12;

    if let Some(street2) = &address.street2 {
        if !street2.is_empty() {
            ops.extend(draw_text(
                x,
                current_y,
                street2,
                pdf_properties.font_size_body,
                fonts,
            ));
            current_y -= 12;
        }
    }

    ops.extend(draw_text(
        x,
        current_y,
        &format!("{}, {} {}", address.city, address.state, address.zip),
        pdf_properties.font_size_body,
        fonts,
    ));
    current_y -= 12;

    ops.extend(draw_text(
        x,
        current_y,
        &address.country,
        pdf_properties.font_size_body,
        fonts,
    ));
    current_y -= 12;

    let has_contact_info = (address.phone.is_some() && !address.phone.as_ref().unwrap().is_empty())
        || (address.vat_number.is_some() && !address.vat_number.as_ref().unwrap().is_empty());
    if has_contact_info {
        current_y -= 8;
    }

    if let Some(phone) = &address.phone {
        if !phone.is_empty() {
            ops.extend(draw_bold_text(
                x,
                current_y,
                &translation.phone_label,
                pdf_properties.font_size_label,
                fonts,
            ));
            ops.extend(draw_text(
                x + 50,
                current_y,
                phone,
                pdf_properties.font_size_body,
                fonts,
            ));
            current_y -= 12;
        }
    }

    if let Some(vat) = &address.vat_number {
        if !vat.is_empty() {
            ops.extend(draw_bold_text(
                x,
                current_y,
                &translation.vat_label,
                pdf_properties.font_size_label,
                fonts,
            ));
            ops.extend(draw_text(
                x + 50,
                current_y,
                vat,
                pdf_properties.font_size_body,
                fonts,
            ));
            current_y -= 12;
        }
    }

    current_y
}

pub fn draw_addresses(
    pdf_properties: &DocumentPropertiesDefault,
    translation: &Dictionary,
    shipping_address: &Address,
    billing_address: &Address,
    shipping_label: &str,
    billing_label: &str,
    fonts: &FontBundle,
) -> (Vec<Op>, i32) {
    let mut ops = vec![];

    let ship_y = draw_address(
        pdf_properties,
        translation,
        &mut ops,
        50,
        610,
        shipping_label,
        shipping_address,
        fonts,
    );
    let bill_y = draw_address(
        pdf_properties,
        translation,
        &mut ops,
        300,
        610,
        billing_label,
        billing_address,
        fonts,
    );

    let final_y = ship_y.min(bill_y);
    let line_y = final_y - 20;
    ops.push(Op::DrawLine {
        line: Line {
            points: vec![
                LinePoint {
                    p: Point::new(Mm(50.0 * 0.352778), Mm(line_y as f32 * 0.352778)),
                    bezier: false,
                },
                LinePoint {
                    p: Point::new(Mm(545.0 * 0.352778), Mm(line_y as f32 * 0.352778)),
                    bezier: false,
                },
            ],
            is_closed: false,
        },
    });

    (ops, line_y)
}

pub fn draw_table_header_background(
    pdf_properties: &DocumentPropertiesDefault,
    current_y: i32,
) -> Vec<Op> {
    vec![
        Op::SetFillColor {
            col: Color::Rgb(Rgb {
                r: pdf_properties.background_color.0,
                g: pdf_properties.background_color.1,
                b: pdf_properties.background_color.2,
                icc_profile: None,
            }),
        },
        Op::DrawPolygon {
            polygon: Polygon {
                rings: vec![PolygonRing {
                    points: vec![
                        LinePoint {
                            p: Point::new(Mm(50.0 * 0.352778), Mm(current_y as f32 * 0.352778)),
                            bezier: false,
                        },
                        LinePoint {
                            p: Point::new(Mm(545.0 * 0.352778), Mm(current_y as f32 * 0.352778)),
                            bezier: false,
                        },
                        LinePoint {
                            p: Point::new(
                                Mm(545.0 * 0.352778),
                                Mm((current_y + 20) as f32 * 0.352778),
                            ),
                            bezier: false,
                        },
                        LinePoint {
                            p: Point::new(
                                Mm(50.0 * 0.352778),
                                Mm((current_y + 20) as f32 * 0.352778),
                            ),
                            bezier: false,
                        },
                    ],
                }],
                mode: PaintMode::Fill,
                winding_order: WindingOrder::NonZero,
            },
        },
        Op::SetFillColor {
            col: Color::Rgb(Rgb {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                icc_profile: None,
            }),
        },
    ]
}

pub fn draw_horizontal_line(y: i32) -> Op {
    Op::DrawLine {
        line: Line {
            points: vec![
                LinePoint {
                    p: Point::new(Mm(50.0 * 0.352778), Mm(y as f32 * 0.352778)),
                    bezier: false,
                },
                LinePoint {
                    p: Point::new(Mm(545.0 * 0.352778), Mm(y as f32 * 0.352778)),
                    bezier: false,
                },
            ],
            is_closed: false,
        },
    }
}
