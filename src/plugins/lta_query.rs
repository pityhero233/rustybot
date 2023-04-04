use lta::{LTAClient, BusRequests};
use crate::plugins::common::Plugin;
use irc::client::Client as IRCClient;
use std::error::Error;
use async_trait::async_trait;
use chrono::{Utc, FixedOffset};
/// LTAQuery plugin returns bus arrival information on query.
/// syntax: bus [station_id] [bus_id]
pub struct LTAQuery<'a> {
    lta_client: LTAClient,
    irc_client: &'a IRCClient
}

#[async_trait]
impl<'a> Plugin for LTAQuery<'a> {
    async fn process(&mut self, _target: &String, msg: &String) -> Result<(), Box<dyn Error>>{
        let splitted: Vec<&str> = msg.split(' ').collect();
        if splitted.len() == 0 || (splitted.len() == 1 && splitted[0] != "bus") {

        } else if splitted.len() != 3 {
            self.send(_target, "invalid amount of arguments")?;
        } else {
            let stop_code: u32 = splitted[1].parse::<u32>()?;
            let result = lta::Bus::get_arrival(&self.lta_client, stop_code, splitted[2]).await?;

            if result.services.len() == 0 {
                self.send(_target, "No bus is currently running!")?;
            } else {
                for (idx, _bus) in result.services.iter().enumerate() {
                    self.send(_target, &format!("Bus No. {}", _bus.service_no))?;
                    for (idx2, e) in _bus.next_bus.iter().enumerate() {
                        if let Some(r) = e {
                            let arrival_time = r.est_arrival;
                            let curr_time = Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()); // TODO proper
                            println!("{arrival_time}, {curr_time}");
                            let d = (arrival_time - curr_time).num_minutes();
                            self.send(_target, &format!("{0} => {1}th bus comes in {d} minutes.", idx + 1, idx2 + 1))?;
                        }
                    }
                }
            }
            self.send(_target, msg)?;
        }
        Ok(())
    }

    fn get_client(&self) -> &IRCClient {
        &self.irc_client
    }
}

impl<'a> LTAQuery<'a> {
    pub async fn new(api: &str, irc: &'a IRCClient) -> LTAQuery<'a> { // time?
        let cli = lta::Client::with_api_key(api).expect("api key verify error");
        LTAQuery {lta_client: cli, irc_client: &irc}
    }
}
