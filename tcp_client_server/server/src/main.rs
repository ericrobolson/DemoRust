use core_connection::{dev_connection::Server, PacketBuffer};

fn main() {
    let mut conn = Server::new(60);
    loop {
        while let Some((client, msg)) = conn.poll() {
            let string = String::from_utf8(msg.data().to_vec()).unwrap();
            println!("Got {} from {:?}", string, client);
            if let Some(packet) = PacketBuffer::from_str("echo") {
                conn.send(client, packet);
            }
        }
    }
}
