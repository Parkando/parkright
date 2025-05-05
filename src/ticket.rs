time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ticket {
    pub attendant_number: String,
    pub cost_center: Option<String>,
    pub customer: Option<String>,
    pub issued_amount: f64,
    pub payout: f64,

    #[serde(rename = "OCRNumber")]
    pub ocr_number: String,
    pub payment_ref: String,
    pub place_code: Option<String>,
    pub place_name: String,

    // @TODO: this should be a DateTime
    #[serde(with = "time::serde::rfc3339")]
    pub print_time: time::OffsetDateTime,
    pub settlement_id: i64,
}
