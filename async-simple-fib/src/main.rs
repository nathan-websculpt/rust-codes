use log::Level;
use tokio::io::AsyncReadExt;

//fibonaci calculation
fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

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

    //spawn task to run in a different thread
    tokio::task::spawn_blocking(move || {
        log::info!("computing fibonaci");
        fib(40);
        log::info!("done computing fibonaci");
    })
    .await
    .unwrap();
}

async fn run() {
    tokio::join!(sleeper(), reader());
}

// you can use this macro instead of creating a new instance of the runtime
// turning main into an async function that returns a future
#[tokio::main]
async fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let start = std::time::Instant::now();
    run().await;
    let end = std::time::Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
