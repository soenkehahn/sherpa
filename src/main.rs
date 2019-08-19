use std::net::UdpSocket;

fn main() -> Result<(), Box<std::error::Error>> {
    let mut socket = UdpSocket::bind("0.0.0.0:1234")?;
    let mut buf = [0; 1000];
    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!("amt: {}, src: {}", amt, src);
        println!("{}", String::from_utf8_lossy(&buf));
    }
    Ok(())
}
