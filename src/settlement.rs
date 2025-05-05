time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Settlement {
    #[serde(rename = "SettlementId")]
    pub id: i64,
    #[serde(rename = "SettlementDate", with = "date_format")]
    pub date: time::Date,
}
