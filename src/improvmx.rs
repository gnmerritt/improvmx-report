#![allow(dead_code)]

use chrono::prelude::*;
use chrono::serde::ts_milliseconds_option;
use reqwest;
use reqwest::Error;
use serde_derive::Deserialize;

const API_BASE: &str = "https://api.improvmx.com/v3";

#[derive(Debug, Deserialize)]
pub struct Domain {
    pub active: bool,
    pub domain: String,
    display: String,
    #[serde(with = "ts_milliseconds_option")]
    added: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
struct DomainResponse {
    domains: Vec<Domain>,
    limit: i32,
    page: i32,
    total: i32,
    success: bool,
}

#[derive(Debug, Deserialize)]
struct LogResponse {
    logs: Vec<MessageLogs>,
    success: bool,
}

#[derive(Debug, Deserialize)]
pub struct MessageEvent {
    code: i16,
    #[serde(with = "ts_milliseconds_option")]
    created: Option<DateTime<Utc>>,
    status: String,
    message: String,
    local: String,
    server: String,
}

#[derive(Debug, Deserialize)]
pub struct Contact {
    name: Option<String>,
    email: String,
}

#[derive(Debug, Deserialize)]
pub struct MessageLogs {
    #[serde(with = "ts_milliseconds_option")]
    created: Option<DateTime<Utc>>,
    events: Vec<MessageEvent>,
    forward: Contact,
    recipient: Contact,
    sender: Contact,
    subject: String,
}

#[derive(Debug)]
pub struct UndeliveredMessage {
    pub subject: String,
    pub from: String,
    pub to: String,
    pub forwarded_to: String,
    pub last_status: String,
    pub last_message: String,
}

pub struct ImprovMx {
    api_key: String,
    client: reqwest::blocking::Client,
}

impl ImprovMx {
    pub fn new(api_key: &str) -> Self {
        ImprovMx {
            api_key: api_key.to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    fn get(&self, url: &str) -> Result<reqwest::blocking::Response, Error> {
        let request_builder = self.client.get(url);
        let res = request_builder
            .basic_auth("api", Some(self.api_key.clone()))
            .send()?
            .error_for_status()?;
        Ok(res)
    }

    pub fn domains(&self) -> Result<Vec<Domain>, Error> {
        let url = format!("{}/domains/?is_active", API_BASE);
        let res = self.get(&url)?;
        let parsed: DomainResponse = res.json()?;
        Ok(parsed.domains)
    }

    pub fn undelivered_messages(&self, domain: &Domain) -> Result<Vec<UndeliveredMessage>, Error> {
        let url = format!("{}/domains/{}/logs", API_BASE, domain.domain);
        let res = self.get(&url)?;
        let parsed: LogResponse = res.json()?;
        let undelivered: Vec<_> = parsed
            .logs
            .into_iter()
            .map(|log| match log.events.last() {
                Some(event) if event.status != "DELIVERED" => Some(UndeliveredMessage {
                    subject: log.subject,
                    from: log.sender.email,
                    to: log.recipient.email,
                    forwarded_to: log.forward.email,
                    last_status: event.status.clone(),
                    last_message: event.message.clone(),
                }),
                _ => None,
            })
            .flatten()
            .collect();
        Ok(undelivered)
    }
}
