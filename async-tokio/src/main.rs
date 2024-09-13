#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handles = vec![];

    for i in 0..3 {
        let handle = tokio::spawn(async move {
            let mut listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", 3000 + i)).await?;

            loop {
                let (mut socket, _) = listener.accept().await?;

                tokio::spawn(async move {
                    let mut buf = [0; 1024];

                    loop {
                        let n = match socket.read(&mut buf).await {
                            Ok(n) if n == 0 => break,
                            Ok(n) => n,
                            Err(e) => {
                                eprintln!("failed to read from socket; err = {e}");
                                break
                            }
                        };

                        let echo = format!("Node {} echoed: {}", i, String::from_utf8_lossy(&buf[..n]));
                        println!("{}", echo);
                        socket.write_all(echo.as_bytes()).await?;
                    }
                });
            }
        });

        handles.push(handle);
    }

    futures::future::join_all(handles).await?;

    Ok(())
}


// the AsyncRead, AsyncWrite, and AsyncBufRead traits
async fn read_file(path: &str) -> std::io::Result<()> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut contents = String::new();

    file.read_to_string(&mut contents).await?;

    println!("{}", contents);

    Ok(())
}

async fn write_file(path: &str, contents: &str) -> std::io::Result<()> {
    let mut file = tokio::fs::File::create(path).await?;

    file.write_all(contents.as_bytes()).await?;

    Ok(())
}

async fn read_lines(path: &str) -> std::io::Result<()> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut reader = tokio::io::BufReader::new(file);

    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        println!("{}", line);
    }

    Ok(())
}
