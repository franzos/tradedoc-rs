use crate::types::{
    Address, Dictionary, DocumentProperties, DocumentPropertiesDefault, Order, OrderLineItem,
};
use lopdf::content::{Content, Operation};
use lopdf::dictionary;
use lopdf::{Document, Object, Stream};
use rust_decimal::Decimal;

use super::errors::PdfError;

fn format_decimal(amount: Decimal, currency: &str) -> String {
    format!("{} {:.2}", currency, amount)
}

fn draw_text(x: i32, y: i32, text: &str, font_size: f32) -> Vec<Operation> {
    vec![
        Operation::new("BT", vec![]),
        Operation::new("Tf", vec!["F1".into(), Object::Real(font_size)]),
        Operation::new("Td", vec![x.into(), y.into()]),
        Operation::new("Tj", vec![Object::string_literal(text)]),
        Operation::new("ET", vec![]),
    ]
}

fn draw_bold_text(x: i32, y: i32, text: &str, font_size: f32) -> Vec<Operation> {
    vec![
        Operation::new("BT", vec![]),
        Operation::new("/F2", vec![]),
        Operation::new(&font_size.to_string(), vec![]),
        Operation::new("Tf", vec![]),
        Operation::new("Td", vec![x.into(), y.into()]),
        Operation::new("Tj", vec![Object::string_literal(text)]),
        Operation::new("ET", vec![]),
    ]
}

fn draw_address(
    pdf_properties: &DocumentPropertiesDefault,
    translation: &Dictionary,
    ops: &mut Vec<Operation>,
    x: i32,
    y: i32,
    title: &str,
    address: &Address,
) -> i32 {
    let mut current_y = y;

    ops.extend(draw_bold_text(
        x,
        current_y,
        title,
        pdf_properties.font_size_label,
    ));
    current_y -= 12;

    ops.extend(draw_text(
        x,
        current_y,
        &address.recipient_name.clone().unwrap_or_default(),
        pdf_properties.font_size_body,
    ));
    current_y -= 12;

    if let Some(company) = &address.company_name {
        if !company.is_empty() {
            ops.extend(draw_text(
                x,
                current_y,
                company,
                pdf_properties.font_size_body,
            ));
            current_y -= 12;
        }
    }

    ops.extend(draw_text(
        x,
        current_y,
        &address.street,
        pdf_properties.font_size_body,
    ));
    current_y -= 12;

    if let Some(street2) = &address.street2 {
        if !street2.is_empty() {
            ops.extend(draw_text(
                x,
                current_y,
                street2,
                pdf_properties.font_size_body,
            ));
            current_y -= 12;
        }
    }

    ops.extend(draw_text(
        x,
        current_y,
        &format!("{}, {} {}", address.city, address.state, address.zip),
        pdf_properties.font_size_body,
    ));
    current_y -= 12;

    ops.extend(draw_text(
        x,
        current_y,
        &address.country,
        pdf_properties.font_size_body,
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
            ));
            ops.extend(draw_text(
                x + 40,
                current_y,
                phone,
                pdf_properties.font_size_body,
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
            ));
            ops.extend(draw_text(
                x + 35,
                current_y,
                vat,
                pdf_properties.font_size_body,
            ));
            current_y -= 12;
        }
    }

    current_y
}

fn draw_header(
    pdf_properties: &DocumentPropertiesDefault,
    translation: &Dictionary,
    order: &Order,
    warehouse_address: &Address,
) -> Vec<Operation> {
    let mut ops = vec![];

    ops.extend(draw_bold_text(
        50,
        820,
        &translation.proforma_invoice_title,
        pdf_properties.font_size_title,
    ));
    draw_address(
        pdf_properties,
        translation,
        &mut ops,
        50,
        780,
        &translation.from_label,
        warehouse_address,
    );

    ops.extend(draw_text(
        350,
        800,
        &format!("PROFORMA-{}", order.id),
        pdf_properties.font_size_body,
    ));
    ops.extend(draw_text(
        350,
        780,
        &format!(
            "{} {}",
            translation.date_label,
            order.created_at.format("%Y-%m-%d")
        ),
        pdf_properties.font_size_body,
    ));
    ops.extend(draw_text(
        350,
        760,
        &format!("{} {}", translation.order_status_label, order.status),
        pdf_properties.font_size_body,
    ));

    // Add "PROFORMA INVOICE" notice
    ops.extend(draw_bold_text(
        350,
        740,
        &translation.proforma_notice,
        pdf_properties.font_size_body,
    ));

    ops.extend(vec![
        Operation::new("m", vec![50.into(), 660.into()]),
        Operation::new("l", vec![545.into(), 660.into()]),
        Operation::new("S", vec![]),
    ]);

    ops
}

fn draw_addresses(
    pdf_properties: &DocumentPropertiesDefault,
    translation: &Dictionary,
    shipping_address: &Address,
    billing_address: &Address,
) -> (Vec<Operation>, i32) {
    let mut ops = vec![];

    let ship_y = draw_address(
        pdf_properties,
        translation,
        &mut ops,
        50,
        640,
        &translation.ship_to_label,
        shipping_address,
    );
    let bill_y = draw_address(
        pdf_properties,
        translation,
        &mut ops,
        300,
        640,
        &translation.bill_to_label,
        billing_address,
    );

    let final_y = ship_y.min(bill_y);
    let line_y = final_y - 20;
    ops.extend(vec![
        Operation::new("m", vec![50.into(), line_y.into()]),
        Operation::new("l", vec![545.into(), line_y.into()]),
        Operation::new("S", vec![]),
    ]);

    (ops, line_y)
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len - 3])
    }
}

fn draw_items_at(
    pdf_properties: &DocumentPropertiesDefault,
    translation: &Dictionary,
    order: &Order,
    items: &[OrderLineItem],
    start_y: i32,
) -> Vec<Operation> {
    let mut ops = vec![];
    let mut current_y = start_y;

    const PRODUCT_DESC_X: i32 = 50;
    const PRODUCT_DESC_WIDTH: i32 = 180;
    const QUANTITY_X: i32 = PRODUCT_DESC_X + PRODUCT_DESC_WIDTH;
    const UNIT_PRICE_X: i32 = QUANTITY_X + 50;
    const DISCOUNT_X: i32 = UNIT_PRICE_X + 70;
    const TAX_X: i32 = DISCOUNT_X + 70;
    const TOTAL_X: i32 = TAX_X + 70;

    // Draw table header
    ops.extend(vec![
        Operation::new("q", vec![]),
        Operation::new(
            &format!(
                "{} {} {} rg",
                pdf_properties.background_color.0,
                pdf_properties.background_color.1,
                pdf_properties.background_color.2
            ),
            vec![],
        ),
        Operation::new(
            "re",
            vec![50.into(), current_y.into(), 495.into(), 20.into()],
        ),
        Operation::new("f", vec![]),
        Operation::new("Q", vec![]),
    ]);

    ops.extend(draw_bold_text(
        PRODUCT_DESC_X,
        current_y + 5,
        &translation.product_header,
        pdf_properties.font_size_label,
    ));
    ops.extend(draw_bold_text(
        QUANTITY_X,
        current_y + 5,
        &translation.quantity_header,
        pdf_properties.font_size_label,
    ));
    ops.extend(draw_bold_text(
        UNIT_PRICE_X - 20,
        current_y + 5,
        &translation.unit_price_header,
        pdf_properties.font_size_label,
    ));
    ops.extend(draw_bold_text(
        DISCOUNT_X - 20,
        current_y + 5,
        &translation.discount_label,
        pdf_properties.font_size_label,
    ));
    ops.extend(draw_bold_text(
        TAX_X - 20,
        current_y + 5,
        &translation.tax_label,
        pdf_properties.font_size_label,
    ));
    ops.extend(draw_bold_text(
        TOTAL_X - 20,
        current_y + 5,
        &translation.estimated_total_label,
        pdf_properties.font_size_label,
    ));

    current_y -= 25;

    // Draw items
    for item in items {
        let desc = match &item.sku {
            Some(sku) if !sku.is_empty() => format!("{} [{}]", item.title, sku),
            _ => item.title.clone(),
        };
        ops.extend(draw_text(
            PRODUCT_DESC_X,
            current_y,
            &truncate_string(&desc, 25),
            pdf_properties.font_size_body,
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
        ));
        ops.extend(draw_text(
            UNIT_PRICE_X + 45 - unit_price_text.len() as i32 * 6,
            current_y,
            &unit_price_text,
            pdf_properties.font_size_body,
        ));
        ops.extend(draw_text(
            DISCOUNT_X + 45 - discount_text.len() as i32 * 6,
            current_y,
            &discount_text,
            pdf_properties.font_size_body,
        ));
        ops.extend(draw_text(
            TAX_X + 45 - tax_text.len() as i32 * 6,
            current_y,
            &tax_text,
            pdf_properties.font_size_body,
        ));
        ops.extend(draw_text(
            TOTAL_X + 45 - total_text.len() as i32 * 6,
            current_y,
            &total_text,
            pdf_properties.font_size_body,
        ));

        current_y -= 20;
    }

    current_y -= 20;
    let estimated_total_label = &translation.estimated_total_label;
    let totals = vec![
        (
            &translation.subtotal_before_discount_label,
            order.subtotal_before_discount,
        ),
        (&translation.discount_label, order.discount_total),
        (&translation.subtotal_label, order.subtotal),
        (&translation.shipping_label, order.shipping_total),
        (&translation.tax_label, order.tax_total),
        (&estimated_total_label, order.total),
    ];

    for (label, amount) in totals {
        current_y -= 20;
        ops.extend(vec![
            Operation::new("q", vec![]),
            Operation::new("0.95 0.95 0.95 rg", vec![]),
            Operation::new(
                "re",
                vec![(TAX_X - 70).into(), current_y.into(), 215.into(), 15.into()],
            ),
            Operation::new("f", vec![]),
            Operation::new("Q", vec![]),
        ]);
        ops.extend(draw_bold_text(
            TAX_X - 65,
            current_y + 2,
            label,
            pdf_properties.font_size_body,
        ));

        let amount_text = format_decimal(amount, &order.currency);
        ops.extend(draw_text(
            TOTAL_X + 45 - amount_text.len() as i32 * 6,
            current_y + 2,
            &amount_text,
            pdf_properties.font_size_body,
        ));
    }

    // Add proforma notice at the bottom
    current_y -= 40;
    ops.extend(draw_bold_text(
        50,
        current_y,
        &translation.proforma_footer_notice,
        pdf_properties.font_size_body,
    ));

    if let Some(notes) = &order.notes {
        current_y -= 25;
        ops.extend(draw_text(
            50,
            current_y,
            &translation.notes_label,
            pdf_properties.font_size_body,
        ));
        current_y -= 15;
        ops.extend(draw_text(
            50,
            current_y,
            notes,
            pdf_properties.font_size_body,
        ));
    }

    ops
}

pub fn generate_pdf_proforma_invoice(
    order: &Order,
    order_items: &[OrderLineItem],
    warehouse_address: &Address,
    properties: DocumentProperties,
    translation: Dictionary,
) -> Result<Vec<u8>, PdfError> {
    let mut doc = Document::with_version("1.5");
    let pdf_properties = properties.input_or_default();
    let pages_id = doc.new_object_id();

    let font_normal_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => pdf_properties.font_normal.clone(),
        "Encoding" => "WinAnsiEncoding",
    });

    let font_bold_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => pdf_properties.font_bold.clone(),
        "Encoding" => "WinAnsiEncoding",
    });

    let resources_id = doc.add_object(dictionary! {
        "Font" => dictionary! {
            "F1" => font_normal_id,
            "F2" => font_bold_id,
        },
    });

    let mut operations = Vec::new();
    operations.extend(draw_header(
        &pdf_properties,
        &translation,
        order,
        warehouse_address,
    ));

    let (address_ops, line_y) = draw_addresses(
        &pdf_properties,
        &translation,
        &order.shipping_address,
        &order.billing_address,
    );
    operations.extend(address_ops);

    let items_y = line_y - 40;
    operations.extend(draw_items_at(
        &pdf_properties,
        &translation,
        order,
        order_items,
        items_y,
    ));

    let content = Content { operations };
    let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode()?));

    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    });

    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![page_id.into()],
        "Count" => 1,
        "Resources" => resources_id,
        "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
    };

    doc.objects.insert(pages_id, Object::Dictionary(pages));

    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });

    doc.trailer.set("Root", catalog_id);
    doc.compress();

    let mut buffer = Vec::new();
    doc.save_to(&mut buffer)?;

    Ok(buffer)
}