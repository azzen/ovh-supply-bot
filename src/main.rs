use std::fmt::Display;

use serde::{Deserialize, Serialize};
use teloxide::{requests::Requester, Bot};

#[derive(Debug, Serialize, Deserialize)]
struct Datacenter {
    availability: String,
    datacenter: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerInfo {
    fqn: String,
    memory: String,
    #[serde(rename = "planCode")]
    plan_code: String,
    server: String,
    storage: String,
    datacenters: Vec<Datacenter>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AvailableDatacenter {
    fqn: String,
    datacenter: String,
}

struct AvailableDatacenters(Vec<AvailableDatacenter>);

fn available_datacenters(servers: &[ServerInfo]) -> AvailableDatacenters {
    let dc = servers
        .iter()
        .flat_map(|server| {
            server
                .datacenters
                .iter()
                .filter(|dc| dc.availability != "unavailable")
                .map(|dc| AvailableDatacenter {
                    fqn: server.fqn.clone(),
                    datacenter: dc.datacenter.clone(),
                })
                .collect::<Vec<AvailableDatacenter>>()
        })
        .collect();
    AvailableDatacenters(dc)
}

use std::ops::Deref;

impl Deref for AvailableDatacenters {
    type Target = Vec<AvailableDatacenter>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for AvailableDatacenter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "- {}: {}", self.datacenter.to_uppercase(), self.fqn)
    }
}

impl Display for AvailableDatacenters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return write!(f, "No available datacenter");
        }
        for adc in &self.0 {
            write!(f, "{adc}")?;
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting querying OVH supplies");

    let url = std::env::args()
        .nth(1)
        .expect("Missing URL to check supplies");

    let chat_id = std::env::var("CHAT_ID")
        .expect("Missing chat id")
        .parse::<i64>()
        .expect("Cannot parse  chat id");
    let bot = Bot::from_env();

    let body = reqwest::get(url).await?.text().await?;

    let server_infos: Vec<ServerInfo> = serde_json::from_str(body.as_str())?;
    let available_datacenters = available_datacenters(&server_infos);

    tracing::info!("Product available: \n{available_datacenters}");
    if !available_datacenters.is_empty() {
        if let Err(e) = bot
            .send_message(
                teloxide::types::ChatId(chat_id),
                format!("Servers supplies available: \n{available_datacenters}"),
            )
            .await
        {
            tracing::error!("Failed to send telegram message to chat_id {chat_id}: {e}");
        }
    }

    Ok(())
}
