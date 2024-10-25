use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.68.71:25565")?;

    println!("Connected to the server!");

    Ok(())
}
