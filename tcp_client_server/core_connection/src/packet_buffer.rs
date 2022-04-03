/// A fixed size packet buffer. Should be xplatform deterministic.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PacketBuffer {
    data_len: usize,
    bytes: [u8; Self::MAX_SIZE],
}
impl PacketBuffer {
    const LENGTH_SIZE: usize = 4;
    pub const DATA_SIZE: usize = Self::MAX_SIZE - Self::LENGTH_SIZE;
    const MAX_SIZE: usize = 420;

    /// Returns the raw empty buffer.
    pub fn empty_buffer() -> [u8; Self::MAX_SIZE] {
        [0; Self::MAX_SIZE]
    }

    /// Returns whether the packet is empty or not.
    pub fn is_empty(&self) -> bool {
        self.data_len == 0
    }

    /// Returns a empty packet buffer.
    pub fn empty() -> Self {
        Self {
            data_len: 0,
            bytes: Self::empty_buffer(),
        }
    }

    /// Returns the bytes for the packet buffer.
    pub fn data(&self) -> &[u8] {
        &self.bytes[Self::LENGTH_SIZE..(Self::LENGTH_SIZE + self.data_len)]
    }

    /// Creates a packet from a string.
    pub fn from_str<'a>(s: &'a str) -> Option<Self> {
        Self::from_bytes(s.as_bytes())
    }

    /// Returns the serialized bytes
    pub(crate) fn serialize(&self) -> &[u8] {
        &self.bytes
    }

    /// Attempts to deserialize the given buffer as a packet.
    pub(crate) fn deserialize<'a>(data: &'a [u8]) -> Option<Self> {
        if data.len() <= Self::MAX_SIZE {
            let mut bytes = [0; Self::MAX_SIZE];
            for (idx, byte) in data.iter().enumerate() {
                let byte = u8::from_le_bytes([*byte]);
                bytes[idx] = byte;
            }

            let size = [data[0], data[1], data[2], data[3]];
            let data_len = u32::from_le_bytes(size) as usize;

            Some(Self { data_len, bytes })
        } else {
            None
        }
    }

    /// Creates a packet from the given slice.
    pub fn from_bytes<'a>(data: &'a [u8]) -> Option<Self> {
        if data.len() <= Self::DATA_SIZE {
            let data_len = data.len() as u32;
            let size = data_len.to_le_bytes();

            let mut bytes = [0; Self::MAX_SIZE];

            // Insert length of packet at start
            let mut idx = 0;
            for byte in size {
                bytes[idx] = byte;
                idx += 1;
            }

            // Write data
            for byte in data {
                let [byte] = byte.to_le_bytes();
                bytes[idx] = byte;
                idx += 1;
            }

            Some(Self {
                bytes,
                data_len: data.len(),
            })
        } else {
            None
        }
    }
}
