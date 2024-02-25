#[repr(C)]
#[repr(packed)]
pub struct Handshake {
    pub protocol_length: u8,
    pub protocol: [u8; 19],
    pub reserved_bytes: [u8; 8],
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}

// It's guaranteed to be 68 bytes with repr(C) and repr(packed)
const HANDSHAKE_SIZE: usize = std::mem::size_of::<Handshake>();

impl Handshake {
    pub fn new(info_hash: [u8; 20]) -> Self {
        Self {
            protocol_length: 19,
            protocol: *b"BitTorrent protocol",
            reserved_bytes: [0; 8],
            info_hash,
            peer_id: *b"00112233445566778899",
        }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        let bytes = self as *mut Self as *mut [u8; HANDSHAKE_SIZE];
        // Safety: Self is a POD with repr(c) and repr(packed)
        let bytes: &mut [u8; HANDSHAKE_SIZE] = unsafe { &mut *bytes };
        bytes
    }
}
