use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    German,
    French,
    Spanish,
    Portuguese,
    Thai,
    Italian,
}

impl Language {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "en" | "english" => Some(Language::English),
            "de" | "german" => Some(Language::German),
            "fr" | "french" => Some(Language::French),
            "es" | "spanish" => Some(Language::Spanish),
            "pt" | "portuguese" => Some(Language::Portuguese),
            "th" | "thai" => Some(Language::Thai),
            "it" | "italian" => Some(Language::Italian),
            _ => None,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::German => "de",
            Language::French => "fr",
            Language::Spanish => "es",
            Language::Portuguese => "pt",
            Language::Thai => "th",
            Language::Italian => "it",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::German => "German",
            Language::French => "French",
            Language::Spanish => "Spanish",
            Language::Portuguese => "Portuguese",
            Language::Thai => "Thai",
            Language::Italian => "Italian",
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

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
    pub language: Language,

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
    
    // Document types
    pub packing_list_title: String,
    pub proforma_invoice_title: String,
    
    // Packing list specific
    pub sku_header: String,
    pub packed_header: String,
    pub return_address_label: String,
    pub shipping_method_label: String,
    pub package_info_title: String,
    pub package_weight_label: String,
    pub package_dimensions_label: String,
    pub carrier_label: String,
    pub tracking_number_label: String,
    pub total_items_label: String,
    pub packer_verification_title: String,
    pub packed_by_label: String,
    pub signature_label: String,
    
    // Proforma invoice specific
    pub estimated_total_label: String,
    pub proforma_notice: String,
    pub proforma_footer_notice: String,
}

impl Default for Dictionary {
    fn default() -> Self {
        Self {
            language: Language::English,

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
            
            // Document types
            packing_list_title: "PACKING LIST".to_string(),
            proforma_invoice_title: "PROFORMA INVOICE".to_string(),
            
            // Packing list specific
            sku_header: "SKU".to_string(),
            packed_header: "Packed".to_string(),
            return_address_label: "Return Address".to_string(),
            shipping_method_label: "Shipping Method:".to_string(),
            package_info_title: "PACKAGE INFORMATION".to_string(),
            package_weight_label: "Package Weight:".to_string(),
            package_dimensions_label: "Package Dimensions:".to_string(),
            carrier_label: "Carrier:".to_string(),
            tracking_number_label: "Tracking Number:".to_string(),
            total_items_label: "TOTAL ITEMS:".to_string(),
            packer_verification_title: "PACKER VERIFICATION".to_string(),
            packed_by_label: "Packed by:".to_string(),
            signature_label: "Signature:".to_string(),
            
            // Proforma invoice specific
            estimated_total_label: "Estimated Total".to_string(),
            proforma_notice: "This is not a bill - for estimate purposes only".to_string(),
            proforma_footer_notice: "NOTICE: This proforma invoice is an estimate only and not a request for payment.".to_string(),
        }
    }
}

impl Dictionary {
    pub fn to_de(self) -> Self {
        Self {
            language: Language::German,

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
            
            // Document types
            packing_list_title: "PACKLISTE".to_string(),
            proforma_invoice_title: "PROFORMA RECHNUNG".to_string(),
            
            // Packing list specific
            sku_header: "SKU".to_string(),
            packed_header: "Verpackt".to_string(),
            return_address_label: "Rücksendeadresse".to_string(),
            shipping_method_label: "Versandart:".to_string(),
            package_info_title: "PAKETINFORMATIONEN".to_string(),
            package_weight_label: "Paketgewicht:".to_string(),
            package_dimensions_label: "Paketabmessungen:".to_string(),
            carrier_label: "Spediteur:".to_string(),
            tracking_number_label: "Sendungsverfolgung:".to_string(),
            total_items_label: "ARTIKEL GESAMT:".to_string(),
            packer_verification_title: "VERPACKUNGSBESTÄTIGUNG".to_string(),
            packed_by_label: "Verpackt von:".to_string(),
            signature_label: "Unterschrift:".to_string(),
            
            // Proforma invoice specific
            estimated_total_label: "Geschätzte Summe".to_string(),
            proforma_notice: "Dies ist keine Rechnung - nur zur Schätzung".to_string(),
            proforma_footer_notice: "HINWEIS: Diese Proforma-Rechnung ist nur eine Schätzung und keine Zahlungsaufforderung.".to_string(),
        }
    }

    pub fn to_fr(self) -> Self {
        Self {
            language: Language::French,

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
            
            // Document types
            packing_list_title: "BON DE LIVRAISON".to_string(),
            proforma_invoice_title: "FACTURE PROFORMA".to_string(),
            
            // Packing list specific
            sku_header: "SKU".to_string(),
            packed_header: "Emballé".to_string(),
            return_address_label: "Adresse de retour".to_string(),
            shipping_method_label: "Méthode d'expédition:".to_string(),
            package_info_title: "INFORMATIONS COLIS".to_string(),
            package_weight_label: "Poids du colis:".to_string(),
            package_dimensions_label: "Dimensions du colis:".to_string(),
            carrier_label: "Transporteur:".to_string(),
            tracking_number_label: "Numéro de suivi:".to_string(),
            total_items_label: "TOTAL ARTICLES:".to_string(),
            packer_verification_title: "VÉRIFICATION EMBALLAGE".to_string(),
            packed_by_label: "Emballé par:".to_string(),
            signature_label: "Signature:".to_string(),
            
            // Proforma invoice specific
            estimated_total_label: "Total estimé".to_string(),
            proforma_notice: "Ceci n'est pas une facture - à des fins d'estimation uniquement".to_string(),
            proforma_footer_notice: "AVIS: Cette facture proforma est seulement une estimation et non une demande de paiement.".to_string(),
        }
    }

    pub fn to_es(self) -> Self {
        Self {
            language: Language::Spanish,

            invoice_title: "FACTURA".to_string(),
            from_label: "De:".to_string(),
            ship_to_label: "Enviar a:".to_string(),
            bill_to_label: "Dirección de facturación:".to_string(),
            phone_label: "Teléfono:".to_string(),
            vat_label: "NIF/CIF:".to_string(),
            product_header: "Producto".to_string(),
            quantity_header: "Cantidad".to_string(),
            unit_price_header: "Precio unitario".to_string(),
            discount_header: "Descuento".to_string(),
            tax_header: "Impuestos".to_string(),
            total_header: "Total".to_string(),
            subtotal_before_discount_label: "Subtotal antes descuento:".to_string(),
            discount_label: "Descuento:".to_string(),
            subtotal_label: "Subtotal:".to_string(),
            shipping_label: "Envío:".to_string(),
            tax_label: "Impuestos:".to_string(),
            total_label: "Total:".to_string(),
            notes_label: "Notas:".to_string(),
            invoice_number_prefix: "Factura #".to_string(),
            date_label: "Fecha:".to_string(),
            order_status_label: "Estado del pedido:".to_string(),
            
            // Document types
            packing_list_title: "ALBARÁN DE ENTREGA".to_string(),
            proforma_invoice_title: "FACTURA PROFORMA".to_string(),
            
            // Packing list specific
            sku_header: "SKU".to_string(),
            packed_header: "Empaquetado".to_string(),
            return_address_label: "Dirección de devolución".to_string(),
            shipping_method_label: "Método de envío:".to_string(),
            package_info_title: "INFORMACIÓN DEL PAQUETE".to_string(),
            package_weight_label: "Peso del paquete:".to_string(),
            package_dimensions_label: "Dimensiones del paquete:".to_string(),
            carrier_label: "Transportista:".to_string(),
            tracking_number_label: "Número de seguimiento:".to_string(),
            total_items_label: "TOTAL ARTÍCULOS:".to_string(),
            packer_verification_title: "VERIFICACIÓN DE EMPAQUETADO".to_string(),
            packed_by_label: "Empaquetado por:".to_string(),
            signature_label: "Firma:".to_string(),
            
            // Proforma invoice specific
            estimated_total_label: "Total estimado".to_string(),
            proforma_notice: "Esto no es una factura - solo para fines de estimación".to_string(),
            proforma_footer_notice: "AVISO: Esta factura proforma es solo una estimación y no una solicitud de pago.".to_string(),
        }
    }

    pub fn to_pt(self) -> Self {
        Self {
            language: Language::Portuguese,

            invoice_title: "FATURA".to_string(),
            from_label: "De:".to_string(),
            ship_to_label: "Enviar para:".to_string(),
            bill_to_label: "Endereço de faturação:".to_string(),
            phone_label: "Telefone:".to_string(),
            vat_label: "NIF:".to_string(),
            product_header: "Produto".to_string(),
            quantity_header: "Quantidade".to_string(),
            unit_price_header: "Preço unitário".to_string(),
            discount_header: "Desconto".to_string(),
            tax_header: "Imposto".to_string(),
            total_header: "Total".to_string(),
            subtotal_before_discount_label: "Subtotal antes desconto:".to_string(),
            discount_label: "Desconto:".to_string(),
            subtotal_label: "Subtotal:".to_string(),
            shipping_label: "Envio:".to_string(),
            tax_label: "Imposto:".to_string(),
            total_label: "Total:".to_string(),
            notes_label: "Notas:".to_string(),
            invoice_number_prefix: "Fatura #".to_string(),
            date_label: "Data:".to_string(),
            order_status_label: "Estado do pedido:".to_string(),
            
            // Document types
            packing_list_title: "GUIA DE REMESSA".to_string(),
            proforma_invoice_title: "FATURA PROFORMA".to_string(),
            
            // Packing list specific
            sku_header: "SKU".to_string(),
            packed_header: "Embalado".to_string(),
            return_address_label: "Endereço de retorno".to_string(),
            shipping_method_label: "Método de envio:".to_string(),
            package_info_title: "INFORMAÇÕES DA ENCOMENDA".to_string(),
            package_weight_label: "Peso da encomenda:".to_string(),
            package_dimensions_label: "Dimensões da encomenda:".to_string(),
            carrier_label: "Transportadora:".to_string(),
            tracking_number_label: "Número de rastreamento:".to_string(),
            total_items_label: "TOTAL DE ARTIGOS:".to_string(),
            packer_verification_title: "VERIFICAÇÃO DE EMBALAGEM".to_string(),
            packed_by_label: "Embalado por:".to_string(),
            signature_label: "Assinatura:".to_string(),
            
            // Proforma invoice specific
            estimated_total_label: "Total estimado".to_string(),
            proforma_notice: "Esta não é uma fatura - apenas para fins de estimativa".to_string(),
            proforma_footer_notice: "AVISO: Esta fatura proforma é apenas uma estimativa e não uma solicitação de pagamento.".to_string(),
        }
    }


    pub fn to_th(self) -> Self {
        Self {
            language: Language::Thai,

            invoice_title: "ใบแจ้งหนี้".to_string(),
            from_label: "จาก:".to_string(),
            ship_to_label: "จัดส่งถึง:".to_string(),
            bill_to_label: "ที่อยู่เรียกเก็บเงิน:".to_string(),
            phone_label: "โทรศัพท์:".to_string(),
            vat_label: "เลขภาษี:".to_string(),
            product_header: "สินค้า".to_string(),
            quantity_header: "จำนวน".to_string(),
            unit_price_header: "ราคาต่อหน่วย".to_string(),
            discount_header: "ส่วนลด".to_string(),
            tax_header: "ภาษี".to_string(),
            total_header: "รวม".to_string(),
            subtotal_before_discount_label: "ยอดรวมก่อนหักส่วนลด:".to_string(),
            discount_label: "ส่วนลด:".to_string(),
            subtotal_label: "ยอดรวมย่อย:".to_string(),
            shipping_label: "ค่าจัดส่ง:".to_string(),
            tax_label: "ภาษี:".to_string(),
            total_label: "รวมทั้งสิ้น:".to_string(),
            notes_label: "หมายเหตุ:".to_string(),
            invoice_number_prefix: "ใบแจ้งหนี้ #".to_string(),
            date_label: "วันที่:".to_string(),
            order_status_label: "สถานะคำสั่งซื้อ:".to_string(),
            
            // Document types
            packing_list_title: "ใบแพ็คกิ้ง".to_string(),
            proforma_invoice_title: "ใบแจ้งหนี้เบื้องต้น".to_string(),
            
            // Packing list specific
            sku_header: "SKU".to_string(),
            packed_header: "บรรจุแล้ว".to_string(),
            return_address_label: "ที่อยู่สำหรับส่งคืน".to_string(),
            shipping_method_label: "วิธีการจัดส่ง:".to_string(),
            package_info_title: "ข้อมูลพัสดุ".to_string(),
            package_weight_label: "น้ำหนักพัสดุ:".to_string(),
            package_dimensions_label: "ขนาดพัสดุ:".to_string(),
            carrier_label: "บริษัทขนส่ง:".to_string(),
            tracking_number_label: "หมายเลขติดตาม:".to_string(),
            total_items_label: "จำนวนรายการทั้งหมด:".to_string(),
            packer_verification_title: "การตรวจสอบการแพ็ค".to_string(),
            packed_by_label: "แพ็คโดย:".to_string(),
            signature_label: "ลายเซ็น:".to_string(),
            
            // Proforma invoice specific
            estimated_total_label: "ยอดรวมโดยประมาณ".to_string(),
            proforma_notice: "นี่ไม่ใช่บิล - สำหรับวัตถุประสงค์ในการประมาณการเท่านั้น".to_string(),
            proforma_footer_notice: "ประกาศ: ใบแจ้งหนี้เบื้องต้นนี้เป็นเพียงการประมาณการเท่านั้น ไม่ใช่การร้องขอการชำระเงิน".to_string(),
        }
    }

    pub fn to_it(self) -> Self {
        Self {
            language: Language::Italian,
            
            invoice_title: "FATTURA".to_string(),
            from_label: "Da:".to_string(),
            ship_to_label: "Spedire a:".to_string(),
            bill_to_label: "Fatturare a:".to_string(),
            phone_label: "Telefono:".to_string(),
            vat_label: "Partita IVA:".to_string(),
            product_header: "Prodotto".to_string(),
            quantity_header: "Quantità".to_string(),
            unit_price_header: "Prezzo unitario".to_string(),
            discount_header: "Sconto".to_string(),
            tax_header: "Tasse".to_string(),
            total_header: "Totale".to_string(),
            subtotal_before_discount_label: "Subtotale prima dello sconto:".to_string(),
            discount_label: "Sconto:".to_string(),
            subtotal_label: "Subtotale:".to_string(),
            shipping_label: "Spedizione:".to_string(),
            tax_label: "Tasse:".to_string(),
            total_label: "Totale:".to_string(),
            notes_label: "Note:".to_string(),
            invoice_number_prefix: "Fattura #".to_string(),
            date_label: "Data:".to_string(),
            order_status_label: "Stato dell'ordine:".to_string(),
            
            // Document types
            packing_list_title: "BOLLA DI CONSEGNA".to_string(),
            proforma_invoice_title: "FATTURA PROFORMA".to_string(),
            
            // Packing list specific
            sku_header: "SKU".to_string(),
            packed_header: "Imballato".to_string(),
            return_address_label: "Indirizzo di ritorno".to_string(),
            shipping_method_label: "Metodo di spedizione:".to_string(),
            package_info_title: "INFORMAZIONI PACCO".to_string(),
            package_weight_label: "Peso del pacco:".to_string(),
            package_dimensions_label: "Dimensioni del pacco:".to_string(),
            carrier_label: "Corriere:".to_string(),
            tracking_number_label: "Numero di tracciamento:".to_string(),
            total_items_label: "TOTALE ARTICOLI:".to_string(),
            packer_verification_title: "VERIFICA IMBALLAGGIO".to_string(),
            packed_by_label: "Imballato da:".to_string(),
            signature_label: "Firma:".to_string(),
            
            // Proforma invoice specific
            estimated_total_label: "Totale stimato".to_string(),
            proforma_notice: "Questo non è una fattura - solo a scopo di stima".to_string(),
            proforma_footer_notice: "AVVISO: Questa fattura proforma è solo una stima e non una richiesta di pagamento.".to_string(),
        }
    }

    pub fn for_language(language: Language) -> Self {
        let base = Dictionary::default();
        match language {
            Language::English => base,
            Language::German => base.to_de(),
            Language::French => base.to_fr(),
            Language::Spanish => base.to_es(),
            Language::Portuguese => base.to_pt(),
            Language::Thai => base.to_th(),
            Language::Italian => base.to_it(),
        }
    }
}
