use std::{
    io::{BufRead, BufReader, Read},
    thread,
};

use mentat::server::NodeRunner;

#[derive(Default)]
pub struct SnarkOSNode;

#[rocket::async_trait]
impl NodeRunner for SnarkOSNode {
    async fn start_node(&self, address: String) -> Result<(), Box<dyn std::error::Error>> {
        // also need the address.
        let mut child = std::process::Command::new("/app/node-runner")
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
                        rocket::error!("SnarkOS: {line}");
                    } else {
                        rocket::info!("SnarkOS: {line}");
                    }
                }
            });
        }
        spawn_reader(stdout, false);
        spawn_reader(stderr, true);

        Ok(())
    }
}
