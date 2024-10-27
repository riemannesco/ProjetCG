mod proxy;

fn main() -> std::io::Result<()> {
    let ip = Some("127.0.0.1:25565".to_string());

    let mut proxy = proxy::Proxy::connect(ip).expect("Couldn't connect");

    let _ = proxy.write("Lucas sort en boite tous les week-end");

    loop {}

    Ok(())
}
