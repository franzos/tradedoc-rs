# Tradedoc: PDF generation for trade documents

The goal of this module is to easily generate PDF's like invoices, receipts, packing lists, etc.

Supported:

- Invoice
- Receipt (Planned)
- Packing List (Planned)

Features:

- Translation
- Customization

## Generate sample invoice

```bash
./target/debug/generate_sample_invoice
```

## Generate PDF's

### Example

Refer to `src/bin/generate_sample_invoice.rs` for an example.

```rs
let pdf_data = generate_pdf_invoice(
    &order,
    &order_items,
    &warehouse_address,
    properties,
    translation,
)
.map_err(|e| format!("Failed to generate PDF: {:?}", e))?;
```

### Fonts

You can customize the fonts, but because they are not embedded, they have to be available on the target system, or you'll see a fallback. This is particularly important if you want **bold** text.

On Linux, it's easy to find out which fonts are available. To list bold fonts:

```bash
fc-list | grep Bold
```

## Development

Compile and start server:

```bash
guix shell rust rust-cargo rust:tools python gcc-toolchain
cargo build
```