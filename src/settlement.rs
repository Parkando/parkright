#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Settlement {
    #[serde(rename = "SettlementId")]
    pub id: u64,
    #[serde(rename = "SettlementDate")]
    pub date: String,
}
