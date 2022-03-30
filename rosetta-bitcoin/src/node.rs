use std::{
    io::{BufRead, BufReader, Read},
    path::PathBuf,
    process::{Command, Stdio},
    thread,
};

use mentat::{
    async_trait,
    conf::Configuration,
    serde::{Deserialize, Serialize},
    server::NodeRunner,
    tracing,
};

#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(crate = "mentat::serde")]
pub struct CustomConfig {
    data_dir: PathBuf,
    user: String,
    pass: String,
}

#[derive(Default)]
pub struct BitcoinNode;

#[async_trait]
impl NodeRunner for BitcoinNode {
    type Custom = CustomConfig;

    async fn start_node(
        &self,
        config: &Configuration<Self::Custom>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut child = Command::new(&config.node_path)
            .args(&[
                // TODO cant bind to address without setting a whitelist
                // &format!("--bind={address}:4132"),
                // &format!("--rpcbind={address}:3032"),
                "-port=4132",
                &format!("-rpcport={}", config.node_rpc_port),
                &format!("-rpcuser={}", config.custom.user),
                &format!("-rpcpassword={}", config.custom.pass),
                "-txindex=1",
                &format!("--datadir={}", config.custom.data_dir.display()),
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
