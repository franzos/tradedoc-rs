use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use std::fs::File;
use std::io::Write;
use tradedoc::templates::invoice::generate_pdf_invoice;
use tradedoc::types::{Address, Dictionary, DocumentProperties, Order, OrderLineItem};

fn create_sample_address(name: &str) -> Address {
    Address {
        recipient_name: Some(name.to_string()),
        company_name: Some("Sample Company Ltd.".to_string()),
        street: "123 Sample Street".to_string(),
        street2: Some("Floor 4".to_string()),
        city: "Sample City".to_string(),
        state: "Sample State".to_string(),
        country: "Sample Country".to_string(),
        zip: "12345".to_string(),
        phone: Some("+1 234 567 890".to_string()),
        vat_number: Some("VAT123456789".to_string()),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create sample order data
    let order = Order {
        id: "ORD-2023-001".to_string(),
        shipping_address: create_sample_address("John Doe"),
        billing_address: create_sample_address("Jane Doe"),
        currency: "$".to_string(),
        status: "Completed".to_string(),
        shipping_method: "Express".to_string(),
        shipping_total: Decimal::new(1500, 2),
        subtotal_before_discount: Decimal::new(50000, 2),
        discount_total: Decimal::new(5000, 2),
        subtotal: Decimal::new(45000, 2),
        tax_total: Decimal::new(9000, 2),
        total: Decimal::new(55500, 2),
        notes: Some("Thank you for your business!".to_string()),
        created_at: NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S")?,
        updated_at: NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S")?,
    };

    // Create sample order items
    let order_items = vec![
        OrderLineItem {
            id: "ITEM1".to_string(),
            title: "Premium Widget".to_string(),
            sku: Some("WDG-001".to_string()),
            quantity: 2,
            unit_price: Decimal::new(15000, 2),
            unit_tax: Decimal::new(3000, 2),
            unit_discount: Decimal::new(1500, 2),
            subtotal_before_discount: Decimal::new(30000, 2),
            discount_total: Decimal::new(3000, 2),
            subtotal: Decimal::new(27000, 2),
            tax_total: Decimal::new(5400, 2),
            total: Decimal::new(32400, 2),
        },
        OrderLineItem {
            id: "ITEM2".to_string(),
            title: "Basic Gadget".to_string(),
            sku: Some("GDG-001".to_string()),
            quantity: 1,
            unit_price: Decimal::new(20000, 2),
            unit_tax: Decimal::new(4000, 2),
            unit_discount: Decimal::new(2000, 2),
            subtotal_before_discount: Decimal::new(20000, 2),
            discount_total: Decimal::new(2000, 2),
            subtotal: Decimal::new(18000, 2),
            tax_total: Decimal::new(3600, 2),
            total: Decimal::new(21600, 2),
        },
    ];

    // Create warehouse address
    let warehouse_address = Address {
        recipient_name: None,
        company_name: Some("Main Warehouse".to_string()),
        street: "789 Warehouse Ave".to_string(),
        street2: None,
        city: "Storage City".to_string(),
        state: "Storage State".to_string(),
        country: "Storage Country".to_string(),
        zip: "54321".to_string(),
        phone: Some("+1 987 654 321".to_string()),
        vat_number: Some("VAT987654321".to_string()),
    };

    let properties = DocumentProperties {
        font_normal: Some("SourceSans3-Regular".to_string()),
        font_bold: Some("SourceSans3-Bold".to_string()),
        background_color: None,
        font_size_title: Some(20.0),
        font_size_body: Some(10.0),
        font_size_label: Some(10.0),
    };

    let translation = Dictionary::default();

    // Generate PDF
    let pdf_data = generate_pdf_invoice(
        &order,
        &order_items,
        &warehouse_address,
        properties,
        translation,
    )
    .map_err(|e| format!("Failed to generate PDF: {:?}", e))?;

    // Save to file
    let mut file = File::create("sample_invoice.pdf")?;
    file.write_all(&pdf_data)?;

    println!("PDF invoice has been generated as 'sample_invoice.pdf'");
    Ok(())
}
