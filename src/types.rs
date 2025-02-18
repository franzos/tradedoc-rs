use rust_decimal::Decimal;

#[derive(Clone)]
pub struct Address {
    pub recipient_name: Option<String>,
    pub street: String,
    pub street2: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub zip: String,
    pub phone: Option<String>,
    pub vat_number: Option<String>,
    pub company_name: Option<String>,
}

#[derive(Clone)]
pub struct Order {
    pub id: String,
    pub shipping_address: Address,
    pub billing_address: Address,
    pub currency: String,
    pub status: String,
    pub shipping_method: String,
    pub shipping_total: Decimal,
    pub subtotal_before_discount: Decimal,
    pub discount_total: Decimal,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub total: Decimal,
    pub notes: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Clone)]
pub struct OrderLineItem {
    pub id: String,
    pub title: String,
    pub sku: Option<String>,
    pub quantity: i64,
    pub unit_price: Decimal,
    pub unit_tax: Decimal,
    pub unit_discount: Decimal,
    pub subtotal_before_discount: Decimal,
    pub discount_total: Decimal,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub total: Decimal,
}

#[derive(Clone)]
pub struct DocumentProperties {
    // default Helvetica
    pub font_normal: Option<String>,
    // default Helvetica-Bold
    pub font_bold: Option<String>,
    // "0.9 0.9 0.9" (light gray)
    pub background_color: Option<(f32, f32, f32)>,
    pub font_size_title: Option<f32>,
    pub font_size_body: Option<f32>,
    pub font_size_label: Option<f32>,
}

#[derive(Clone)]
pub struct DocumentPropertiesDefault {
    // default Helvetica
    pub font_normal: String,
    // default Helvetica-Bold
    pub font_bold: String,
    // "0.9 0.9 0.9" (light gray)
    pub background_color: (f32, f32, f32),
    pub font_size_title: f32,
    pub font_size_body: f32,
    pub font_size_label: f32,
}

impl DocumentProperties {
    pub fn input_or_default(self) -> DocumentPropertiesDefault {
        DocumentPropertiesDefault {
            font_normal: self.font_normal.unwrap_or("Helvetica".to_string()),
            font_bold: self.font_bold.unwrap_or("Helvetica-Bold".to_string()),
            background_color: self.background_color.unwrap_or((0.9, 0.9, 0.9)),
            font_size_title: self.font_size_title.unwrap_or(20.0),
            font_size_body: self.font_size_body.unwrap_or(10.0),
            font_size_label: self.font_size_label.unwrap_or(10.0),
        }
    }
}

#[derive(Clone)]
pub struct Dictionary {
    // Title
    pub invoice_title: String,

    // Headers
    pub from_label: String,
    pub ship_to_label: String,
    pub bill_to_label: String,

    // Contact info
    pub phone_label: String,
    pub vat_label: String,

    // Table headers
    pub product_header: String,
    pub quantity_header: String,
    pub unit_price_header: String,
    pub discount_header: String,
    pub tax_header: String,
    pub total_header: String,

    // Summary labels
    pub subtotal_before_discount_label: String,
    pub discount_label: String,
    pub subtotal_label: String,
    pub shipping_label: String,
    pub tax_label: String,
    pub total_label: String,
    pub notes_label: String,

    // Document info
    pub invoice_number_prefix: String,
    pub date_label: String,
    pub order_status_label: String,
}

impl Default for Dictionary {
    fn default() -> Self {
        Self {
            invoice_title: "INVOICE".to_string(),
            from_label: "From:".to_string(),
            ship_to_label: "Ship To:".to_string(),
            bill_to_label: "Bill To:".to_string(),
            phone_label: "Phone:".to_string(),
            vat_label: "VAT:".to_string(),
            product_header: "Product".to_string(),
            quantity_header: "Qty".to_string(),
            unit_price_header: "Unit Price".to_string(),
            discount_header: "Discount".to_string(),
            tax_header: "Tax".to_string(),
            total_header: "Total".to_string(),
            subtotal_before_discount_label: "Subtotal Before Discount:".to_string(),
            discount_label: "Discount:".to_string(),
            subtotal_label: "Subtotal:".to_string(),
            shipping_label: "Shipping:".to_string(),
            tax_label: "Tax:".to_string(),
            total_label: "Total:".to_string(),
            notes_label: "Notes:".to_string(),
            invoice_number_prefix: "Invoice #".to_string(),
            date_label: "Date:".to_string(),
            order_status_label: "Order Status:".to_string(),
        }
    }
}

impl Dictionary {
    pub fn to_de(self) -> Self {
        Self {
            invoice_title: "RECHNUNG".to_string(),
            from_label: "Von:".to_string(),
            ship_to_label: "Versand an:".to_string(),
            bill_to_label: "Rechnungsadresse:".to_string(),
            phone_label: "Telefon:".to_string(),
            vat_label: "USt-IdNr.:".to_string(),
            product_header: "Produkt".to_string(),
            quantity_header: "Menge".to_string(),
            unit_price_header: "Preis".to_string(),
            discount_header: "Rabatt".to_string(),
            tax_header: "Steuer".to_string(),
            total_header: "Gesamt".to_string(),
            subtotal_before_discount_label: "Zwischensumme vor Rabatt:".to_string(),
            discount_label: "Rabatt:".to_string(),
            subtotal_label: "Zwischensumme:".to_string(),
            shipping_label: "Versand:".to_string(),
            tax_label: "Steuer:".to_string(),
            total_label: "Gesamt:".to_string(),
            notes_label: "Notizen:".to_string(),
            invoice_number_prefix: "Rechnung #".to_string(),
            date_label: "Datum:".to_string(),
            order_status_label: "Bestellstatus:".to_string(),
        }
    }

    pub fn to_fr(self) -> Self {
        Self {
            invoice_title: "FACTURE".to_string(),
            from_label: "De".to_string(),
            ship_to_label: "Expédition à:".to_string(),
            bill_to_label: "Adresse de facturation:".to_string(),
            phone_label: "Téléphone:".to_string(),
            vat_label: "Numéro de TVA:".to_string(),
            product_header: "Produit".to_string(),
            quantity_header: "Quantité".to_string(),
            unit_price_header: "Prix unitaire".to_string(),
            discount_header: "Remise".to_string(),
            tax_header: "Taxe".to_string(),
            total_header: "Total".to_string(),
            subtotal_before_discount_label: "Sous-total avant remise:".to_string(),
            discount_label: "Remise:".to_string(),
            subtotal_label: "Sous-total:".to_string(),
            shipping_label: "Expédition:".to_string(),
            tax_label: "Taxe:".to_string(),
            total_label: "Total:".to_string(),
            notes_label: "Remarques:".to_string(),
            invoice_number_prefix: "Facture #".to_string(),
            date_label: "Date:".to_string(),
            order_status_label: "Statut de la commande:".to_string(),
        }
    }
}
