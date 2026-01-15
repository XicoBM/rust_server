
use tokio::net::{TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:4000").await?;
    println!("Servidor rodando em 0.0.0.0:4000");

    loop {
        let (mut socket, _) = listener.accept().await?;
        
        tokio::spawn(async move{
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back
            loop {
                let n = match socket.read(&mut buf).await {
                    // Socket closed
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                let buf = 
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("Failed to write to socket; err = {:?}", e);
                    return;
                } 
            }
        });
    }
}
