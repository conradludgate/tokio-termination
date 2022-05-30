use std::{future::Future, time::Duration};

use tokio::time::sleep;
use tokio_termination::Tokio;

fn main() -> Tokio<impl Future<Output = ()>> {
    async {
        println!("Hello, ");
        sleep(Duration::from_secs(1)).await;
        println!("Tokio!");
    }
    .into()
}
