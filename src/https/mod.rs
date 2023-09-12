use crate::credentials::*;
use base64::{engine::general_purpose, Engine as _};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Certificate,
};

#[derive(Debug)]
pub struct LcuClient {
    pub client: reqwest::Client,
    pub host_name: String,
}

impl FromIterator<Credentials> for Vec<LcuClient> {
    fn from_iter<T: IntoIterator<Item = Credentials>>(iter: T) -> Self {
        iter.into_iter()
            .map(|c| {
                c.to_lcu_client(
                    "https://127.0.0.1",
                    Certificate::from_pem(pem::CERTIFICATE).unwrap(),
                )
            })
            .collect()
    }
}

pub trait ToLcuClient {
    fn to_lcu_client(&self, base_url: &str, certificate: Certificate) -> LcuClient;
}

impl ToLcuClient for Credentials {
    fn to_lcu_client(&self, base_url: &str, certificate: Certificate) -> LcuClient {
        let host_name = format!("{}:{}", base_url, self.port);
        let user_pass = HeaderValue::from_str(&format!(
            "Basic {}",
            general_purpose::STANDARD.encode(format!("riot:{}", self.pass).as_bytes())
        ))
        .unwrap();

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", user_pass);
        LcuClient {
            client: reqwest::Client::builder()
                .add_root_certificate(certificate)
                .default_headers(headers)
                .build()
                .unwrap(),
            host_name,
        }
    }
}
