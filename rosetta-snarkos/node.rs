use mentat::server::NodeRunner;

#[derive(Default)]
pub struct SnarkOSNode;

#[async_trait::async_trait]
impl NodeRunner for SnarkOSNode {
    async fn start_node(
        self,
        address: String,
        mut cmd: std::process::Command,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // download parameters
        let parameters_url =
            "https://s3-us-west-1.amazonaws.com/aleo.parameters/posw.proving.b2d14c7";
        let response = reqwest::get(parameters_url).await?;

        let mut dest = {
            // for now only works in docker.
            let mut full_path = std::path::PathBuf::from("/root/.aleo/resources");
            std::fs::create_dir_all(&full_path)?;
            full_path.push("");
            std::fs::File::create("posw.proving.b2d14c7")?
        };

        let content = response.text().await?;
        std::io::copy(&mut content.as_bytes(), &mut dest)?;

        // also need the address.
        let _child = cmd
            .args(&[
                "--node",
                &format!("{address}:4132"),
                "--rpc",
                &format!("{address}:3032"),
                "--trial",
                "--verbosity",
                "2",
            ])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;

        // do some logging here with stdout & stderr
        Ok(())
    }
}
