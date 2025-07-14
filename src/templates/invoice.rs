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
    FontBundle, format_decimal, draw_text, draw_bold_text, truncate_string,
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
        ops.extend(draw_logo(doc, 50, 790, Some(logo), 80.0, 24.0)?);
    }

    ops.extend(draw_bold_text(
        420,
        790,
        &translation.invoice_title,
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
        740,
        &format!("{}{}", translation.invoice_number_prefix, order.id),
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
    order: &Order,
    items: &[OrderLineItem],
    start_y: i32,
    fonts: &FontBundle,
) -> Vec<Op> {
    let mut ops = vec![];
    let mut current_y = start_y;

    const PRODUCT_DESC_X: i32 = 50;
    const PRODUCT_DESC_WIDTH: i32 = 180;
    const QUANTITY_X: i32 = PRODUCT_DESC_X + PRODUCT_DESC_WIDTH;
    const UNIT_PRICE_X: i32 = QUANTITY_X + 50;
    const DISCOUNT_X: i32 = UNIT_PRICE_X + 70;
    const TAX_X: i32 = DISCOUNT_X + 70;
    const TOTAL_X: i32 = TAX_X + 70;

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
        QUANTITY_X,
        current_y + 5,
        &translation.quantity_header,
        pdf_properties.font_size_label,
        fonts,
    ));
    ops.extend(draw_bold_text(
        UNIT_PRICE_X - 0,
        current_y + 5,
        &translation.unit_price_header,
        pdf_properties.font_size_label,
        fonts,
    ));
    ops.extend(draw_bold_text(
        DISCOUNT_X - 0,
        current_y + 5,
        &translation.discount_label,
        pdf_properties.font_size_label,
        fonts,
    ));
    ops.extend(draw_bold_text(
        TAX_X - 0,
        current_y + 5,
        &translation.tax_label,
        pdf_properties.font_size_label,
        fonts,
    ));
    ops.extend(draw_bold_text(
        TOTAL_X - 0,
        current_y + 5,
        &translation.total_label,
        pdf_properties.font_size_label,
        fonts,
    ));

    current_y -= 25;

    // Draw items
    for item in items {
        // Format product title and SKU
        let desc = match &item.sku {
            Some(sku) if !sku.is_empty() => format!("{} [{}]", item.title, sku),
            _ => item.title.clone(),
        };
        ops.extend(draw_text(
            PRODUCT_DESC_X,
            current_y,
            &truncate_string(&desc, 25),
            pdf_properties.font_size_body,
            fonts,
        ));

        let quantity_text = item.quantity.to_string();
        let unit_price_text = format_decimal(item.unit_price, &order.currency);
        let discount_text = format_decimal(item.discount_total, &order.currency);
        let tax_text = format_decimal(item.tax_total, &order.currency);
        let total_text = format_decimal(item.total, &order.currency);

        ops.extend(draw_text(
            QUANTITY_X + 35 - quantity_text.len() as i32 * 6,
            current_y,
            &quantity_text,
            pdf_properties.font_size_body,
            fonts,
        ));
        ops.extend(draw_text(
            UNIT_PRICE_X + 45 - unit_price_text.len() as i32 * 6,
            current_y,
            &unit_price_text,
            pdf_properties.font_size_body,
            fonts,
        ));
        ops.extend(draw_text(
            DISCOUNT_X + 45 - discount_text.len() as i32 * 6,
            current_y,
            &discount_text,
            pdf_properties.font_size_body,
            fonts,
        ));
        ops.extend(draw_text(
            TAX_X + 45 - tax_text.len() as i32 * 6,
            current_y,
            &tax_text,
            pdf_properties.font_size_body,
            fonts,
        ));
        ops.extend(draw_text(
            TOTAL_X + 45 - total_text.len() as i32 * 6,
            current_y,
            &total_text,
            pdf_properties.font_size_body,
            fonts,
        ));

        current_y -= 20;
    }

    current_y -= 20;
    let totals = vec![
        (
            &translation.subtotal_before_discount_label,
            order.subtotal_before_discount,
        ),
        (&translation.discount_label, order.discount_total),
        (&translation.subtotal_label, order.subtotal),
        (&translation.shipping_label, order.shipping_total),
        (&translation.tax_label, order.tax_total),
        (&translation.total_label, order.total),
    ];

    for (label, amount) in totals {
        current_y -= 20;
        ops.push(Op::SetFillColor {
            col: Color::Rgb(Rgb { r: 0.95, g: 0.95, b: 0.95, icc_profile: None }),
        });
        ops.push(Op::DrawPolygon {
            polygon: Polygon {
                rings: vec![PolygonRing {
                    points: vec![
                        LinePoint { p: Point::new(Mm((TAX_X - 70) as f32 * 0.352778), Mm(current_y as f32 * 0.352778)), bezier: false },
                        LinePoint { p: Point::new(Mm((TAX_X - 70 + 215) as f32 * 0.352778), Mm(current_y as f32 * 0.352778)), bezier: false },
                        LinePoint { p: Point::new(Mm((TAX_X - 70 + 215) as f32 * 0.352778), Mm((current_y + 15) as f32 * 0.352778)), bezier: false },
                        LinePoint { p: Point::new(Mm((TAX_X - 70) as f32 * 0.352778), Mm((current_y + 15) as f32 * 0.352778)), bezier: false },
                    ]
                }],
                mode: PaintMode::Fill,
                winding_order: WindingOrder::NonZero,
            }
        });
        
        // Reset text color to black for subsequent text
        ops.push(Op::SetFillColor {
            col: Color::Rgb(Rgb { r: 0.0, g: 0.0, b: 0.0, icc_profile: None }),
        });
        ops.extend(draw_bold_text(
            TAX_X - 65,
            current_y + 2,
            label,
            pdf_properties.font_size_body,
            fonts,
        ));

        let amount_text = format_decimal(amount, &order.currency);
        ops.extend(draw_text(
            TOTAL_X + 45 - amount_text.len() as i32 * 6,
            current_y + 2,
            &amount_text,
            pdf_properties.font_size_body,
            fonts,
        ));
    }

    if let Some(notes) = &order.notes {
        current_y -= 40;
        ops.extend(draw_text(
            PRODUCT_DESC_X,
            current_y,
            &translation.notes_label,
            pdf_properties.font_size_body,
            fonts,
        ));
        current_y -= 15;
        ops.extend(draw_text(
            PRODUCT_DESC_X,
            current_y,
            notes,
            pdf_properties.font_size_body,
            fonts,
        ));
    }

    ops
}

pub fn generate_pdf_invoice(
    order: &Order,
    order_items: &[OrderLineItem],
    warehouse_address: &Address,
    properties: DocumentProperties,
    translation: Dictionary,
    logo_data: Option<&[u8]>,
    custom_font_normal: Option<&[u8]>,
    custom_font_bold: Option<&[u8]>,
) -> Result<Vec<u8>, PdfError> {
    let pdf_properties = properties.input_or_default();
    let mut doc = PdfDocument::new("Invoice");
    let fonts = load_fonts(&mut doc, Some(translation.language), custom_font_normal, custom_font_bold)?;

    // Create content with all operations
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

    // Get the y position after drawing addresses
    let (address_ops, line_y) = draw_addresses(
        &pdf_properties,
        &translation,
        &order.shipping_address,
        &order.billing_address,
        &translation.ship_to_label,
        &translation.bill_to_label,
        &fonts,
    );
    operations.extend(address_ops);

    // Start items section 40 points below the line
    let items_y = line_y - 40;
    operations.extend(draw_items_at(
        &pdf_properties,
        &translation,
        order,
        order_items,
        items_y,
        &fonts,
    ));

    // Create the page with all operations
    let page = PdfPage::new(Mm(210.0), Mm(297.0), operations);
    
    // Generate the PDF
    let bytes = doc
        .with_pages(vec![page])
        .save(&PdfSaveOptions::default(), &mut Vec::new());

    Ok(bytes)
}
