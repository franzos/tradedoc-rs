use crate::types::{
    Address, Dictionary, DocumentProperties, DocumentPropertiesDefault, Order, OrderLineItem,
};
use printpdf::{
    Color, Mm, Op, PdfDocument, PdfPage, PdfSaveOptions, Point, Rgb,
    graphics::{LinePoint}, PaintMode, Polygon, PolygonRing, WindingOrder,
};
use chrono::Datelike;

use super::errors::PdfError;
use super::pdf_utils::{
    FontBundle, draw_text, draw_bold_text, truncate_string,
    load_fonts, draw_address, draw_addresses, draw_table_header_background, draw_horizontal_line, draw_logo
};


fn draw_header(
    doc: &mut PdfDocument,
    pdf_properties: &DocumentPropertiesDefault,
    translation: &Dictionary,
    order: &Order,
    warehouse_address: &Address,
    fonts: &FontBundle,
    logo_data: Option<&[u8]>,
) -> Result<Vec<Op>, PdfError> {
    let mut ops = vec![];

    // Add logo in top right if provided
    if let Some(logo) = logo_data {
        ops.extend(draw_logo(doc, 460, 780, Some(logo), 80.0, 24.0)?);
    }

    ops.extend(draw_bold_text(
        50,
        790,
        &translation.packing_list_title,
        pdf_properties.font_size_title,
        fonts,
    ));
    draw_address(
        pdf_properties,
        translation,
        &mut ops,
        50,
        750,
        &translation.from_label,
        warehouse_address,
        fonts,
    );

    ops.extend(draw_text(
        350,
        770,
        &format!("PACK-{}", order.id),
        pdf_properties.font_size_body,
        fonts,
    ));
    ops.extend(draw_text(
        350,
        720,
        &format!(
            "{} {}",
            translation.date_label,
            format!("{:04}-{:02}-{:02}", order.created_at.year(), order.created_at.month(), order.created_at.day())
        ),
        pdf_properties.font_size_body,
        fonts,
    ));
    ops.extend(draw_text(
        350,
        700,
        &format!("{} {}", translation.shipping_method_label, order.shipping_method),
        pdf_properties.font_size_body,
        fonts,
    ));
    ops.extend(draw_text(
        350,
        680,
        &format!("{} {}", translation.order_status_label, order.status),
        pdf_properties.font_size_body,
        fonts,
    ));

    ops.push(draw_horizontal_line(630));

    Ok(ops)
}



fn draw_items_at(
    pdf_properties: &DocumentPropertiesDefault,
    translation: &Dictionary,
    items: &[OrderLineItem],
    start_y: i32,
    fonts: &FontBundle,
) -> Vec<Op> {
    let mut ops = vec![];
    let mut current_y = start_y;

    const PRODUCT_DESC_X: i32 = 50;
    const PRODUCT_DESC_WIDTH: i32 = 250;
    const SKU_X: i32 = PRODUCT_DESC_X + PRODUCT_DESC_WIDTH;
    const SKU_WIDTH: i32 = 100;
    const QUANTITY_X: i32 = SKU_X + SKU_WIDTH;
    const WEIGHT_X: i32 = QUANTITY_X + 80;

    // Draw table header background
    ops.extend(draw_table_header_background(pdf_properties, current_y));

    ops.extend(draw_bold_text(
        PRODUCT_DESC_X,
        current_y + 5,
        &translation.product_header,
        pdf_properties.font_size_label,
        fonts,
    ));
    ops.extend(draw_bold_text(
        SKU_X,
        current_y + 5,
        &translation.sku_header,
        pdf_properties.font_size_label,
        fonts,
    ));
    ops.extend(draw_bold_text(
        QUANTITY_X,
        current_y + 5,
        &translation.quantity_header,
        pdf_properties.font_size_label,
        fonts,
    ));
    ops.extend(draw_bold_text(
        WEIGHT_X,
        current_y + 5,
        &translation.packed_header,
        pdf_properties.font_size_label,
        fonts,
    ));

    current_y -= 25;

    // Draw items
    for item in items {
        ops.extend(draw_text(
            PRODUCT_DESC_X,
            current_y,
            &truncate_string(&item.title, 35),
            pdf_properties.font_size_body,
            fonts,
        ));

        let sku_text = item.sku.as_deref().unwrap_or("N/A");
        ops.extend(draw_text(
            SKU_X,
            current_y,
            &truncate_string(sku_text, 12),
            pdf_properties.font_size_body,
            fonts,
        ));

        let quantity_text = item.quantity.to_string();
        ops.extend(draw_text(
            QUANTITY_X + 35 - quantity_text.len() as i32 * 6,
            current_y,
            &quantity_text,
            pdf_properties.font_size_body,
            fonts,
        ));

        // Add checkbox for "packed" status
        ops.push(Op::SetOutlineColor {
            col: Color::Rgb(Rgb { r: 0.0, g: 0.0, b: 0.0, icc_profile: None }),
        });
        ops.push(Op::DrawPolygon {
            polygon: Polygon {
                rings: vec![PolygonRing {
                    points: vec![
                        LinePoint { p: Point::new(Mm(WEIGHT_X as f32 * 0.352778), Mm(current_y as f32 * 0.352778)), bezier: false },
                        LinePoint { p: Point::new(Mm((WEIGHT_X + 10) as f32 * 0.352778), Mm(current_y as f32 * 0.352778)), bezier: false },
                        LinePoint { p: Point::new(Mm((WEIGHT_X + 10) as f32 * 0.352778), Mm((current_y + 10) as f32 * 0.352778)), bezier: false },
                        LinePoint { p: Point::new(Mm(WEIGHT_X as f32 * 0.352778), Mm((current_y + 10) as f32 * 0.352778)), bezier: false },
                    ]
                }],
                mode: PaintMode::Stroke,
                winding_order: WindingOrder::NonZero,
            }
        });

        current_y -= 20;
    }

    // Package info section
    current_y -= 30;
    ops.extend(draw_bold_text(
        50,
        current_y,
        &translation.package_info_title,
        pdf_properties.font_size_label,
        fonts,
    ));
    current_y -= 20;

    let package_fields = vec![
        format!("{} ___________", translation.package_weight_label),
        format!("{} L:_____ W:_____ H:_____", translation.package_dimensions_label),
        format!("{} ___________", translation.carrier_label),
        format!("{} ___________", translation.tracking_number_label),
    ];

    for field in package_fields {
        ops.extend(draw_text(
            50,
            current_y,
            &field,
            pdf_properties.font_size_body,
            fonts,
        ));
        current_y -= 18;
    }

    // Total items summary
    current_y -= 20;
    let total_items: i64 = items.iter().map(|item| item.quantity).sum();
    ops.extend(draw_bold_text(
        50,
        current_y,
        &format!("{} {}", translation.total_items_label, total_items),
        pdf_properties.font_size_label,
        fonts,
    ));

    // Packer signature section
    current_y -= 40;
    ops.extend(draw_bold_text(
        50,
        current_y,
        &translation.packer_verification_title,
        pdf_properties.font_size_label,
        fonts,
    ));
    current_y -= 20;
    ops.extend(draw_text(
        50,
        current_y,
        &format!("{} ___________________ Date: _________ Time: _________", translation.packed_by_label),
        pdf_properties.font_size_body,
        fonts,
    ));
    current_y -= 20;
    ops.extend(draw_text(
        50,
        current_y,
        &format!("{} ___________________________________", translation.signature_label),
        pdf_properties.font_size_body,
        fonts,
    ));

    ops
}

pub fn generate_pdf_packing_list(
    order: &Order,
    order_items: &[OrderLineItem],
    warehouse_address: &Address,
    properties: DocumentProperties,
    translation: Dictionary,
    logo_data: Option<&[u8]>,
) -> Result<Vec<u8>, PdfError> {
    let pdf_properties = properties.input_or_default();
    let mut doc = PdfDocument::new("Packing List");
    let fonts = load_fonts(&mut doc, Some(translation.language), pdf_properties.font_normal_path.as_deref(), pdf_properties.font_bold_path.as_deref())?;

    let mut operations = Vec::new();
    operations.extend(draw_header(
        &mut doc,
        &pdf_properties,
        &translation,
        order,
        warehouse_address,
        &fonts,
        logo_data,
    )?);

    let (address_ops, line_y) = draw_addresses(
        &pdf_properties,
        &translation,
        &order.shipping_address,
        &order.billing_address,
        &translation.ship_to_label,
        &translation.return_address_label,
        &fonts,
    );
    operations.extend(address_ops);

    let items_y = line_y - 40;
    operations.extend(draw_items_at(
        &pdf_properties,
        &translation,
        order_items,
        items_y,
        &fonts,
    ));

    let page = PdfPage::new(Mm(210.0), Mm(297.0), operations);
    let bytes = doc
        .with_pages(vec![page])
        .save(&PdfSaveOptions::default(), &mut Vec::new());

    Ok(bytes)
}