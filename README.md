# Tradedoc: PDF generation for trade documents

The goal of this module is to easily generate PDF's like invoices, receipts, packing lists, etc.

Supported:

- Invoice
- Proforma Invoice
- Packing List

Features:

- Translation*
  - English (`Language::English`)
  - German (`Language::German`)
  - French (`Language::French`)
  - Spanish (`Language::Spanish`)
  - Portuguese (`Language::Portuguese`)
  - Thai (`Language::Thai`)
  - Italian (`Language::Italian`)
  - Add or overwrite via Dictionary
- Customization

*_Machine translation: If something's off, please provide a PR, or overwrite it with your own dictionary._

## Generate examples

```bash
cargo run --bin example invoice
cargo run --bin example proforma-invoice
cargo run --bin example packing-list
```

With language support:

```bash
cargo run --bin example invoice --language de
cargo run --bin example proforma-invoice --language fr
cargo run --bin example packing-list --language en
```

## Usage

### Generate PDF's

Refer to `src/bin/example.rs` for usage examples.

```rs
// Invoice
let pdf_data = generate_pdf_invoice(
    &order,
    &order_items,
    &warehouse_address,
    properties,
    translation,
    Some(logo_bytes),        // Logo data (PNG/SVG)
)?;

// Proforma Invoice
let pdf_data = generate_pdf_proforma_invoice(
    &order,
    &order_items,
    &warehouse_address,
    properties,
    translation,
    Some(logo_bytes),        // Logo data (PNG/SVG)
)?;

// Packing List
let pdf_data = generate_pdf_packing_list(
    &order,
    &order_items,
    &warehouse_address,
    properties,
    translation,
    Some(logo_bytes),        // Logo data (PNG/SVG)
)?;
```

_Font customization is handled via the `DocumentProperties` struct._

### Data Structures

#### 1. Order

```rs
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use tradedoc::types::{Order, Address};

let order = Order {
    id: "ORD-2023-001".to_string(),
    shipping_address: shipping_address,  // Address struct
    billing_address: billing_address,    // Address struct
    currency: "€".to_string(),
    status: "Completed".to_string(),
    shipping_method: "Express".to_string(),
    shipping_total: Decimal::new(1500, 2),            // €15.00
    subtotal_before_discount: Decimal::new(50000, 2), // €500.00
    discount_total: Decimal::new(5000, 2),            // €50.00
    subtotal: Decimal::new(45000, 2),                 // €450.00
    tax_total: Decimal::new(9000, 2),                 // €90.00
    total: Decimal::new(55500, 2),                    // €555.00
    notes: Some("Thank you for your business!".to_string()),
    created_at: NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    updated_at: NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
};
```

#### 2. Order Items

```rs
use tradedoc::types::OrderLineItem;

let order_items = vec![
    OrderLineItem {
        id: "ITEM1".to_string(),
        title: "Premium Widget".to_string(),
        sku: Some("WDG-001".to_string()),
        quantity: 2,
        unit_price: Decimal::new(15000, 2),               // €150.00
        unit_tax: Decimal::new(3000, 2),                  // €30.00
        unit_discount: Decimal::new(1500, 2),             // €15.00
        subtotal_before_discount: Decimal::new(30000, 2), // €300.00
        discount_total: Decimal::new(3000, 2),            // €30.00
        subtotal: Decimal::new(27000, 2),                 // €270.00
        tax_total: Decimal::new(5400, 2),                 // €54.00
        total: Decimal::new(32400, 2),                    // €324.00
    },
    OrderLineItem {
        id: "ITEM2".to_string(),
        title: "Basic Gadget".to_string(),
        sku: Some("GDG-001".to_string()),
        quantity: 1,
        unit_price: Decimal::new(20000, 2),               // €200.00
        unit_tax: Decimal::new(4000, 2),                  // €40.00
        unit_discount: Decimal::new(2000, 2),             // €20.00
        subtotal_before_discount: Decimal::new(20000, 2), // €200.00
        discount_total: Decimal::new(2000, 2),            // €20.00
        subtotal: Decimal::new(18000, 2),                 // €180.00
        tax_total: Decimal::new(3600, 2),                 // €36.00
        total: Decimal::new(21600, 2),                    // €216.00
    },
];
```

#### 3. Warehouse Address

```rs
use tradedoc::types::Address;

let warehouse_address = Address {
    recipient_name: None,
    company_name: Some("ACME GmbH".to_string()),
    street: "Musterstraße 123".to_string(),
    street2: Some("4. Etage".to_string()),
    city: "Frankfurt am Main".to_string(),
    state: "Hesse".to_string(),
    country: "Germany".to_string(),
    zip: "60311".to_string(),
    phone: Some("+49 69 123 456 789".to_string()),
    vat_number: Some("DE123456789".to_string()),
};

// Customer addresses (similar structure)
let shipping_address = Address {
    recipient_name: Some("Max Mustermann".to_string()),
    company_name: Some("Musterfirma GmbH".to_string()),
    street: "Kundenstraße 456".to_string(),
    street2: None,
    city: "Frankfurt am Main".to_string(),
    state: "Hesse".to_string(),
    country: "Germany".to_string(),
    zip: "60329".to_string(),
    phone: Some("+49 69 987 654 321".to_string()),
    vat_number: Some("DE987654321".to_string()),
};
```

#### 4. Document Properties

```rs
use tradedoc::types::DocumentProperties;

// Use defaults
let properties = DocumentProperties {
    font_normal_path: None,               // Will use embedded NotoSans
    font_bold_path: None,                 // Will use embedded NotoSans-Bold
    background_color: None,               // Will use light gray (0.9, 0.9, 0.9)
    font_size_title: None,                // Will use 20.0
    font_size_body: None,                 // Will use 10.0
    font_size_label: None,                // Will use 10.0
};
```

#### 5. Translation / Dictionary

```rs
use tradedoc::types::{Dictionary, Language};

// Use default English translation
let translation = Dictionary::default();

// Use predefined language
let german_translation = Dictionary::for_language(Language::German);

// Custom translation (override specific labels)
let custom_translation = Dictionary {
    language: Language::English,
    invoice_title: "SALES INVOICE".to_string(),
    from_label: "Vendor:".to_string(),
    ship_to_label: "Delivery Address:".to_string(),
    bill_to_label: "Billing Address:".to_string(),
    phone_label: "Phone:".to_string(),
    vat_label: "Tax ID:".to_string(),
    product_header: "Item".to_string(),
    quantity_header: "Qty".to_string(),
    unit_price_header: "Unit Price".to_string(),
    discount_header: "Discount".to_string(),
    tax_header: "Tax".to_string(),
    total_header: "Total".to_string(),
    // ... other fields use default values
    ..Dictionary::default()
};
```

### Fonts

The following fonts are embedded for ease of use:

- `NotoSans-Regular`
- `NotoSans-SemiBold`
- `NotoSansThai-Regular`
- `NotoSansThai-SemiBold`

**Custom fonts**: You can provide paths to custom TTF font files via `font_normal_path` and `font_bold_path`. If `None`, the system uses embedded NotoSans fonts which provide excellent multi-language support including Thai characters.

```rs
let properties = DocumentProperties {
    font_normal_path: Some("assets/fonts/MyFont-Regular.ttf".to_string()),
    font_bold_path: Some("assets/fonts/MyFont-Bold.ttf".to_string()),
    // ... other properties
};
```

### Logo Support

All document types support optional logo placement in the top-right corner:
- **Supported formats**: PNG, SVG
- **Usage**: Pass logo bytes via the `logo_data` parameter
- **Position**: Top-right corner of the document
- **Size**: Automatically scaled to fit (80mm × 24mm)
- **Embedding**: Use `include_bytes!()` to embed logo in binary

Example:
```rs
const LOGO: &[u8] = include_bytes!("assets/logo.png");

let pdf_data = generate_pdf_invoice(
    &order,
    &order_items,
    &warehouse_address,
    properties,
    translation,
    Some(LOGO),  // Logo data
)?;
```

## Development

Compile and start server:

```bash
guix shell -m manifest.scm
cargo build
```

Clippy:

```bash
docker run --rm -v $(pwd):/app -w /app rust:1.82 sh \
-c "rustup component add clippy && cargo clippy \
--all-targets --all-features -- -D warnings"
```