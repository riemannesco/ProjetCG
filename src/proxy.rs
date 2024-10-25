use std::io::Write;

pub struct Proxy {
    output_stream: Option<std::net::TcpStream>,
}

impl Proxy {
    pub fn connect(target_ip: String) -> Result<Proxy, Box<dyn std::error::Error>> {
        let stream = std::net::TcpStream::connect(target_ip)?;

        println!("Connected to the Minecraft Server!");

        Ok(Proxy { output_stream: Some(stream) })
    }

    pub fn write(&mut self, s: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.output_stream.as_mut().expect("Couldn't find a server to write").write_all(s.as_bytes())?;
        Ok(())
    }
}
