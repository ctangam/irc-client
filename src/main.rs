use tokio::net::{TcpListener, TcpSocket, TcpStream};
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;



#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("irc.freenode.net:6667").await.unwrap();
    stream.write_all("NICK web-88".as_bytes()).await.unwrap();
    stream.write_all("USER guest 0 * :Coding Challenges Client".as_bytes()).await.unwrap();

    let mut buf = [0; 8192];
    loop {
        let n = stream.read(&mut buf).await.unwrap();
        if n == 0 {
            break;
        }
        print!("{}", String::from_utf8_lossy(&buf[..n]));
    }
}