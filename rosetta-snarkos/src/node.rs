use std::{
    io::{BufRead, BufReader, Read},
    thread,
};

use mentat::{async_trait, server::NodeRunner, tracing};

#[derive(Default)]
pub struct SnarkOSNode;

#[async_trait]
impl NodeRunner for SnarkOSNode {
    async fn start_node(&self, address: String) -> Result<(), Box<dyn std::error::Error>> {
        // also need the address.
        let snarkos = std::env::var("NODE").unwrap_or_else(|_| "/app/node-runner".to_string());

        let mut child = std::process::Command::new(snarkos)
            .args(&[
                "--node",
                &format!("{address}:4132"),
                "--rpc",
                &format!("{address}:3032"),
                "--trial",
                "--verbosity",
                "2",
            ])
            .stderr(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        fn spawn_reader<T: 'static + Read + Send>(out: T, err: bool) {
            let mut reader = BufReader::new(out).lines();
            thread::spawn(move || {
                while let Some(Ok(line)) = reader.next() {
                    if err {
                        tracing::error!("SnarkOS: {line}");
                    } else {
                        tracing::info!("SnarkOS: {line}");
                    }
                }
            });
        }
        spawn_reader(stdout, false);
        spawn_reader(stderr, true);

        Ok(())
    }
}
