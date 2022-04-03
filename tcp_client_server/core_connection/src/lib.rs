pub mod dev_connection;
mod packet_buffer;
pub use packet_buffer::*;

pub fn server_address() -> String {
    "127.0.0.1:8090".into()
}
