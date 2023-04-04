use irc::client::Client;
use std::error::Error;
use async_trait::async_trait;
#[async_trait]
pub trait Plugin {
    async fn process(&mut self, target: &String, msg: &String) -> Result<(), Box<dyn Error>>;
    fn send(&mut self, target: &str, msg: &str) -> Result<(), Box<dyn Error>> { // why add async gg
        self.get_client().send_privmsg(target, msg)?;
        Ok(())
    }
    fn get_client(&self) -> &Client;
}