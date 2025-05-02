use std::sync::Arc;

use base64::{Engine, prelude::BASE64_STANDARD};
use reqwest::header::{HeaderMap, HeaderValue};

mod error;
mod settlement;
mod ticket;

pub use {error::Error, settlement::Settlement, ticket::Ticket};

const PROD_URL: &str = "https://parkright.se/";

#[allow(dead_code)]
const TEST_URL: &str = "https://test.parkright.se/";

#[derive(Clone)]
pub struct Client {
    c: reqwest::Client,
    base_url: Arc<String>,
}

impl Client {
    pub fn from_env() -> Self {
        let username = std::env::var("PARKRIGHT_USERNAME").expect("PARKRIGHT_USERNAME");
        let password = std::env::var("PARKRIGHT_PASSWORD").expect("PARKRIGHT_PASSWORD");

        println!("{username}:{password}");

        Self::new(username, password)
    }

    pub fn new(username: impl AsRef<str>, password: impl AsRef<str>) -> Self {
        let auth_key =
            BASE64_STANDARD.encode(format!("{}:{}", username.as_ref(), password.as_ref()));
        let auth_header_value = format!("Basic {auth_key}");

        println!("{auth_header_value}");

        let mut headers = HeaderMap::new();

        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&auth_header_value).expect("Basc auth header value"),
        );
        headers.insert(
            reqwest::header::ACCEPT,
            HeaderValue::from_static("application/json"),
        );

        let c = reqwest::ClientBuilder::new()
            .user_agent(format!("Parkando Parkright v{}", env!("CARGO_PKG_VERSION")))
            .default_headers(headers)
            .build()
            .expect("building");

        Self {
            c,
            base_url: Arc::new(String::from(PROD_URL)),
        }
    }

    async fn fetch_and_parse<T>(&self, url: &str) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        let res = self.c.get(url).send().await?;

        let status = res.status();
        if !status.is_success() {
            return Err(Error::NonOkReply {
                status: status.as_u16(),
            });
        }

        let bs = res.bytes().await?;

        match serde_json::from_slice::<T>(&bs) {
            Ok(res) => Ok(res),

            Err(err) => Err(Error::Deserializing { err }),
        }
    }

    pub async fn fetch_settlements(&self) -> Result<Vec<Settlement>, Error> {
        #[derive(serde::Deserialize)]
        struct Wrapper {
            #[serde(rename = "AvailableSettlements")]
            settlements: Vec<Settlement>,
        }

        let url = format!(
            "{}/api/v1/ReviewSettlementCasesOffStreet/availablesettlements?startDate=1900-01-01&endDate=2026-01-01",
            self.base_url.as_str()
        );

        let res = self.fetch_and_parse::<Wrapper>(&url).await?;
        Ok(res.settlements)
    }

    pub async fn fetch_tickets(&self, settlement_ids: &[u64]) -> Result<Vec<Ticket>, Error> {
        if settlement_ids.is_empty() {
            return Ok(vec![]);
        }

        let param_ids = settlement_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let url = format!(
            "{}/api/v1/ReviewSettlementCasesOffStreet/report?settlementIds={}",
            self.base_url.as_str(),
            param_ids
        );

        let res = self.fetch_and_parse::<Vec<Ticket>>(&url).await?;

        Ok(res)
    }
}
