use futures::prelude::*;
use irc::client::prelude::*;
mod plugins;
use plugins::{common::Plugin, remember::RememberPlugin, lta_query::LTAQuery};

#[tokio::main]
async fn main() -> irc::error::Result<()> {
    let config = Config {
        nickname: Some("rustybot15".to_owned()),
        server: Some("irc.libera.chat".to_owned()),
        channels: vec!["##xade_bridge".to_owned()],
        use_tls: Some(true),
        ..Default::default()
    };

    let mut client: Client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;

    let mut plugins: [Box<dyn Plugin>; 2] = [
        Box::new(RememberPlugin::new("/home/xade/memo", &client).await),
        Box::new(LTAQuery::new("ZKybqazFSJSEmg/AnIBuiQ==", &client).await),
    ];

    while let Some(message) = stream.next().await.transpose()? {
        print!("{}", message);

        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                for e in plugins.iter_mut() {
                    e.process(target, msg).await.expect("plugin fail");
                }
            }, 
            _ => (),
        }
    }
    Ok(())
}
