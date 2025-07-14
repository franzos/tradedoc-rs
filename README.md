# Tradedoc: PDF generation for trade documents

The goal of this module is to easily generate PDF's like invoices, receipts, packing lists, etc.

Supported:

- Invoice
- Proforma Invoice
- Packing List
- Receipt (Planned)

Features:

- Translation*
  - English
  - German
  - French
  - Thai
  - Portuguese (Portugal)
  - Spanish
  - Italian
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
let pdf_data = generate_pdf_invoice(&order, &order_items, &warehouse_address, properties, translation)?;

// Proforma Invoice
let pdf_data = generate_pdf_proforma_invoice(&order, &order_items, &warehouse_address, properties, translation)?;

// Packing List
let pdf_data = generate_pdf_packing_list(&order, &order_items, &warehouse_address, properties, translation)?;
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
guix shell -m manifest.scm
cargo build
```

Clippy:

```bash
docker run --rm -v $(pwd):/app -w /app rust:1.82 sh \
-c "rustup component add clippy && cargo clippy \
--all-targets --all-features -- -D warnings"
```