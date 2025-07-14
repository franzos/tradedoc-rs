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
    None,                    // Custom normal font
    None,                    // Custom bold font
)?;

// Proforma Invoice
let pdf_data = generate_pdf_proforma_invoice(
    &order,
    &order_items,
    &warehouse_address,
    properties,
    translation,
    Some(logo_bytes),        // Logo data (PNG/SVG)
    None,                    // Custom normal font
    None,                    // Custom bold font
)?;

// Packing List
let pdf_data = generate_pdf_packing_list(
    &order,
    &order_items,
    &warehouse_address,
    properties,
    translation,
    Some(logo_bytes),        // Logo data (PNG/SVG)
    None,                    // Custom normal font
    None,                    // Custom bold font
)?;
```

_This is a bit verbose, but allows support for more document types in the future._

### Fonts

The following fonts are embedded for ease of use:

- `NotoSans-Regular`
- `NotoSans-SemiBold`
- `NotoSansThai-Regular`
- `NotoSansThai-SemiBold`

You can also provide custom fonts by passing font bytes to the `custom_font_normal` and `custom_font_bold` parameters. Pass `None` to use the default embedded fonts.

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
    // ... other parameters
    Some(LOGO),  // Logo data
    // ... remaining parameters
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