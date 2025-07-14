use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use std::env;
use std::fs::File;
use std::io::Write;
use tradedoc::templates::invoice::generate_pdf_invoice;
use tradedoc::templates::proforma_invoice::generate_pdf_proforma_invoice;
use tradedoc::templates::packing_list::generate_pdf_packing_list;
use tradedoc::types::{Address, Dictionary, DocumentProperties, Order, OrderLineItem, Language};

// Embed the PNG logo in the binary
const GOFRANZ_LOGO: &[u8] = include_bytes!("../../assets/gofranz.png");

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

fn create_sample_data() -> (Order, Vec<OrderLineItem>, Address) {
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
        created_at: NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        updated_at: NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    };

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

    (order, order_items, warehouse_address)
}

fn print_usage() {
    println!("Usage: cargo run --bin example [document-type] [options]");
    println!();
    println!("Document types:");
    println!("  invoice          - Generate a standard invoice");
    println!("  proforma-invoice - Generate a proforma invoice (estimate)");
    println!("  packing-list     - Generate a packing list");
    println!();
    println!("Options:");
    println!("  --language <lang>    - Language (en, de, fr, es, pt, th, it) [default: en]");
    println!("  --font <name>        - Normal font name [default: SourceSans3-Regular]");
    println!("  --font-bold <name>   - Bold font name [default: SourceSans3-Bold]");
    println!();
    println!("Examples:");
    println!("  cargo run --bin example invoice");
    println!("  cargo run --bin example invoice --language de");
    println!("  cargo run --bin example invoice --font Helvetica --font-bold Helvetica-Bold");
    println!("  cargo run --bin example proforma-invoice --language fr --font Arial");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let document_type = &args[1];
    
    // Parse options
    let mut language = Language::English;
    let mut font_normal = "SourceSans3-Regular".to_string();
    let mut font_bold = "SourceSans3-Bold".to_string();
    
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--language" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --language requires a value");
                    print_usage();
                    return Ok(());
                }
                match Language::from_str(&args[i + 1]) {
                    Some(lang) => language = lang,
                    None => {
                        eprintln!("Error: Unsupported language '{}'. Supported: en, de, fr, es, pt, th, it", &args[i + 1]);
                        print_usage();
                        return Ok(());
                    }
                }
                i += 2;
            }
            "--font" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --font requires a value");
                    print_usage();
                    return Ok(());
                }
                font_normal = args[i + 1].clone();
                i += 2;
            }
            "--font-bold" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --font-bold requires a value");
                    print_usage();
                    return Ok(());
                }
                font_bold = args[i + 1].clone();
                i += 2;
            }
            _ => {
                eprintln!("Error: Unknown option '{}'", args[i]);
                print_usage();
                return Ok(());
            }
        }
    }

    let (order, order_items, warehouse_address) = create_sample_data();

    let properties = DocumentProperties {
        font_normal: Some(font_normal),
        font_bold: Some(font_bold),
        background_color: None,
        font_size_title: Some(20.0),
        font_size_body: Some(10.0),
        font_size_label: Some(10.0),
    };

    let translation = Dictionary::for_language(language);

    let base_filename = match document_type.as_str() {
        "invoice" => "sample_invoice",
        "proforma-invoice" => "sample_proforma_invoice",
        "packing-list" => "sample_packing_list",
        _ => {
            eprintln!("Error: Unknown document type '{}'", document_type);
            print_usage();
            return Ok(());
        }
    };

    let filename = if language == Language::English {
        format!("{}.pdf", base_filename)
    } else {
        format!("{}_{}.pdf", base_filename, language.code())
    };

    let pdf_data = match document_type.as_str() {
        "invoice" => generate_pdf_invoice(
            &order,
            &order_items,
            &warehouse_address,
            properties,
            translation,
            Some(GOFRANZ_LOGO), // Use embedded SVG logo
            None,               // Use default fonts
            None,               // Use default fonts
        )?,
        "proforma-invoice" => generate_pdf_proforma_invoice(
            &order,
            &order_items,
            &warehouse_address,
            properties,
            translation,
            Some(GOFRANZ_LOGO), // Use embedded SVG logo
            None,               // Use default fonts
            None,               // Use default fonts
        )?,
        "packing-list" => generate_pdf_packing_list(
            &order,
            &order_items,
            &warehouse_address,
            properties,
            translation,
            Some(GOFRANZ_LOGO), // Use embedded SVG logo
            None,               // Use default fonts
            None,               // Use default fonts
        )?,
        _ => unreachable!(),
    };

    let mut file = File::create(&filename)?;
    file.write_all(&pdf_data)?;

    println!("PDF {} has been generated as '{}'", document_type, filename);
    Ok(())
}