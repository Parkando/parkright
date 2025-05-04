#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Settlement {
    #[serde(rename = "SettlementId")]
    pub id: i64,
    #[serde(rename = "SettlementDate")]
    pub date: String,
}
