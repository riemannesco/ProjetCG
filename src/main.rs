use std::net::TcpStream;

mod proxy;

fn main() -> std::io::Result<()> {

    let mut proxy = proxy::Proxy::connect("192.168.68.71:25566".to_string()).expect("Couldn't connect");

    let _ = proxy.write("Lucas sort en boite tous les week-end");

    loop {}

    Ok(())
}
