use std::io::Write;

pub struct Proxy {
    output_stream: Option<std::net::TcpStream>,
}

impl Proxy {
    pub fn connect(target_ip: Option<String>) -> Result<Proxy, Box<dyn std::error::Error + 'static>> {
        if let Some(target_ip) = target_ip {
            let stream = std::net::TcpStream::connect(target_ip)?;

            println!("Connected to the Minecraft Server!");

            return Ok(Proxy { output_stream: Some(stream) });
        }
        Ok(Proxy { output_stream: Option::None })
    }

    pub fn write(&mut self, s: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.output_stream.as_mut().expect("Couldn't find a server to write").write_all(s.as_bytes())?;
        Ok(())
    }
}
