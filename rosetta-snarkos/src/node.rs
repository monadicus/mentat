use std::{
    io::{BufRead, BufReader, Read},
    path::Path,
    thread,
};

use mentat::{async_trait, server::NodeRunner, tracing};

#[derive(Default)]
pub struct SnarkOSNode;

#[async_trait]
impl NodeRunner for SnarkOSNode {
    async fn start_node(
        &self,
        address: String,
        node_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: make it so snarkos checks for updates and rebuilds automatically.
        let mut child = std::process::Command::new(node_path)
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

        // TODO: move this method to part of NodeRunner trait.
        // Maybe use tokio?
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
