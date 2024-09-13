use log::Level;
use tokio::time;

async fn run() {
    log::info!("Sleeping");
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("Awake!");
}

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = run(); //futures are kind of like lazy promises

    rt.block_on(future);
}
