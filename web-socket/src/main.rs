use tungstenite::{connect, Message};
use url::Url;

fn main() -> Result<(), tungstenite::Error> {
    let (mut socket, _response) =
        connect(Url::parse("wss://echo.websocket.org")?)?;

    socket.send(Message::Text("Hello, world!".to_string()))?;

    loop {
        let msg = socket.recv()?;

        println!("Received: {}", msg);

        if let Message::Close(_) = msg {
            break;
        }
    }

    Ok(())
}
