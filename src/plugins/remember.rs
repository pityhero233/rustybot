use crate::plugins::common::Plugin;
use irc::client::prelude::*;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use async_trait::async_trait;
use std::error::Error;
use irc::client::Client as IRCClient;
pub struct RememberPlugin<'a>{
    f: File,          // how do I tell compiler this is just "intermediate variable" and do not need init?
    irc_client: &'a IRCClient
}

#[async_trait]
impl<'a> Plugin for RememberPlugin<'a> {
    async fn process(&mut self, _target: &String, msg: &String) -> Result<(), Box<dyn Error>>{
        self.f.write_all(format!("{msg}\n").as_bytes()).await.unwrap();
        self.send(_target, "noted")?;
        Ok(())
    }
    fn get_client(&self) -> &Client {
        return self.irc_client;
    }
}

impl<'a> RememberPlugin<'a> {
    pub async fn new(file_path: &str, cli: &'a IRCClient) -> RememberPlugin<'a> {
        let fi = File::create(file_path).await.unwrap();
        RememberPlugin{f: fi, irc_client: cli}
    }
}