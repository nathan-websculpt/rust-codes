use log::Level;
use tokio::io::AsyncReadExt;

async fn sleeper() {
    log::info!("Sleeping");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    log::info!("Awake!");
}

async fn reader() {
    log::info!("Reading them datas");
    let mut reading = tokio::fs::File::open("tst-csv.xlsx").await.unwrap();
    let mut contents = vec![];
    reading.read_to_end(&mut contents).await.unwrap();
    log::info!("Read {} bytes", contents.len()); // tell us how many bytes we read
}

async fn run() {
    // run syndronously
    // sleeper().await;
    // reader().await;

    // run asynchronously
    tokio::join!(sleeper(), reader());
}

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();
    let future = run(); //futures are kind of like lazy promises

    rt.block_on(future);
}
