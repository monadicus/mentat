use std::{
    io::{BufRead, BufReader, Read},
    thread,
};

use mentat::server::NodeRunner;

#[derive(Default)]
pub struct SnarkOSNode;

#[async_trait::async_trait]
impl NodeRunner for SnarkOSNode {
    async fn start_node(
        &self,
        address: String,
        mut cmd: std::process::Command,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // also need the address.
        let mut child = cmd
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

        fn spawn_reader<T: 'static + Read + Send>(prefix: &'static str, out: T) {
            let mut reader = BufReader::new(out).lines();
            thread::spawn(move || {
                while let Ok(line) = reader.next().unwrap() {
                    println!("SnarkOS{}: {}", prefix, line);
                }
            });
        }
        spawn_reader("", stdout);
        spawn_reader(" ERROR", stderr);

        Ok(())
    }
}
