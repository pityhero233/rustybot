use crate::plugins::common::Plugin;
use irc::client::prelude::*;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use async_trait::async_trait;
use std::error::Error;
use irc::client::Client as IRCClient;
use chrono::Local;

/// Remember what is important.
/// Remember block start with "rmb" end with "ermb". Also support single line prefixed with "rmbl".
/// Time is auto notated.
/// 
pub struct RememberPlugin<'a>{
    f: File,          // how do I tell compiler this is just "intermediate variable" and do not need init?
    irc_client: &'a IRCClient,
    state: bool
}

#[async_trait]
impl<'a> Plugin for RememberPlugin<'a> {
    async fn process(&mut self, _target: &String, msg: &String) -> Result<(), Box<dyn Error>>{
        let r: Vec<&str> = msg.split(' ').collect();
        if msg == "rmb" { 
            self.state = true;
            self.send(_target, "I will now start remembering. ")?;
            self.f.write_all(format!("{}: \n", Local::now()).as_bytes()).await?;
        } else if msg == "ermb" { 
            self.state = false;
            self.f.write_all(format!("\n").as_bytes()).await?;
            self.send(_target, "Okey-dokey!")?;
        } else if self.state == true {
            self.f.write_all(format!("{msg}\n").as_bytes()).await?;
        } else if r[0] == "rmbl" && msg.len() > 5 {
            self.f.write_all(format!("{}\n", &msg[5..]).as_bytes()).await?;
        }
        Ok(())
    }
    fn get_client(&self) -> &Client {
        return self.irc_client;
    }
}

impl<'a> RememberPlugin<'a> {
    pub async fn new(file_path: &str, cli: &'a IRCClient) -> RememberPlugin<'a> {
        let fi = File::create(file_path).await.unwrap();
        RememberPlugin{f: fi, irc_client: cli, state: false}
    }
}