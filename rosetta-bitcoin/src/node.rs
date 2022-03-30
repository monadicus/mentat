use std::{
    io::{BufRead, BufReader, Read},
    path::Path,
    process::{Command, Stdio},
    thread,
};

use mentat::{async_trait, server::NodeRunner, tracing};

#[derive(Default)]
pub struct BitcoinNode;

#[async_trait]
impl NodeRunner for BitcoinNode {
    async fn start_node(
        &self,
        address: String,
        _node_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO 0rphon un-hack this
        // let bitcoin = std::env::var("NODE").unwrap_or_else(|_|
        // "/app/node-runner".to_string());
        let bitcoin = "D:\\Program Files\\Bitcoin\\daemon\\bitcoind.exe";

        let mut child = Command::new(bitcoin)
            .args(&[
                // TODO cant bind to address without setting a whitelist
                // &format!("--bind={address}:4132"),
                // &format!("--rpcbind={address}:3032"),
                &format!("-port=4132"),
                &format!("-rpcport=3032"),
                &format!("-rpcuser=USER"),
                &format!("-rpcpassword=PASS"),
                &format!("-txindex=1"),
                &format!("--datadir=D:\\files\\btc"),
            ])
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        // TODO: move this method to part of NodeRunner trait.
        // Maybe use tokio?
        fn spawn_reader<T: 'static + Read + Send>(out: T, err: bool) {
            let mut reader = BufReader::new(out).lines();
            thread::spawn(move || {
                while let Some(Ok(line)) = reader.next() {
                    if err {
                        tracing::error!("Bitcoin: {line}");
                    } else {
                        tracing::info!("Bitcoin: {line}");
                    }
                }
            });
        }
        spawn_reader(stdout, false);
        spawn_reader(stderr, true);

        Ok(())
    }
}
