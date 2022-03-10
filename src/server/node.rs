use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Orbit, Rocket};

#[rocket::async_trait]
pub trait NodeRunner: Send + Sync + 'static {
    async fn start_node(
        &self,
        _address: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[rocket::async_trait]
impl Fairing for Box<dyn NodeRunner> {
    fn info(&self) -> Info {
	Info {
	    name: "Node Launcher",
	    kind: Kind::Liftoff,
	}
    }

	
    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
	self.start_node(rocket.config().address.to_string()).await.expect("failed to launce node");
    }
}

#[derive(Default)]
pub struct DummyNode;

#[rocket::async_trait]
impl NodeRunner for DummyNode {}
