use crate::handshake::Handshake;
use crate::torrent_file::TorrentFile;
use anyhow::{Context, Ok, Result};
use bytes::{BufMut, BytesMut};
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageTag {
    Choke = 0,
    Unchoke = 1,
    Interested = 2,
    NotInterested = 3,
    Have = 4,
    Bitfield = 5,
    Request = 6,
    Piece = 7,
    Cancel = 8,
}

impl From<u8> for MessageTag {
    fn from(value: u8) -> Self {
        match value {
            0 => MessageTag::Choke,
            1 => MessageTag::Unchoke,
            2 => MessageTag::Interested,
            3 => MessageTag::NotInterested,
            4 => MessageTag::Have,
            5 => MessageTag::Bitfield,
            6 => MessageTag::Request,
            7 => MessageTag::Piece,
            8 => MessageTag::Cancel,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub tag: MessageTag,
    pub payload: Vec<u8>,
}

#[repr(C)]
#[repr(packed)]
pub struct Request {
    pub index: [u8; 4],
    pub begin: [u8; 4],
    pub length: [u8; 4],
}

impl Request {
    pub fn new(index: u32, begin: u32, length: u32) -> Self {
        Self {
            index: index.to_be_bytes(),
            begin: begin.to_be_bytes(),
            length: length.to_be_bytes(),
        }
    }

    pub fn index(&self) -> u32 {
        u32::from_be_bytes(self.index)
    }

    pub fn begin(&self) -> u32 {
        u32::from_be_bytes(self.begin)
    }

    pub fn length(&self) -> u32 {
        u32::from_be_bytes(self.length)
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        let bytes = self as *mut Self as *mut [u8; std::mem::size_of::<Self>()];
        // Safety: Self is a POD with repr(c) and repr(packed)
        let bytes: &mut [u8; std::mem::size_of::<Self>()] = unsafe { &mut *bytes };
        bytes
    }
}

#[repr(C)]
// NOTE: needs to be (and is)
// #[repr(packed)]
// but can't be marked as such because of the T: ?Sized part
pub struct Piece<T: ?Sized = [u8]> {
    index: [u8; 4],
    begin: [u8; 4],
    block: T,
}

impl Piece {
    pub fn index(&self) -> u32 {
        u32::from_be_bytes(self.index)
    }

    pub fn begin(&self) -> u32 {
        u32::from_be_bytes(self.begin)
    }

    pub fn block(&self) -> &[u8] {
        &self.block
    }

    const PIECE_LEAD: usize = std::mem::size_of::<Piece<()>>();
    pub fn ref_from_bytes(data: &[u8]) -> Option<&Self> {
        if data.len() < Self::PIECE_LEAD {
            return None;
        }
        let n = data.len();
        // NOTE: The slicing here looks really weird. The reason we do it is because we need the
        // length part of the fat pointer to Piece to hold the length of _just_ the `block` field.
        // And the only way we can change the length of the fat pointer to Piece is by changing the
        // length of the fat pointer to the slice, which we do by slicing it. We can't slice it at
        // the front (as it would invalidate the ptr part of the fat pointer), so we slice it at
        // the back!
        let piece = &data[..n - Self::PIECE_LEAD] as *const [u8] as *const Piece;
        // Safety: Piece is a POD with repr(c) and repr(packed), _and_ the fat pointer data length
        // is the length of the trailing DST field (thanks to the PIECE_LEAD offset).
        Some(unsafe { &*piece })
    }
}

pub struct Peer {
    torrent_file: TorrentFile,
    stream: TcpStream,
}

impl Peer {
    pub fn new(peer_addr: String, torrent_file: TorrentFile) -> Result<Self> {
        let info_hash = torrent_file.info.hash_info().context("hash info")?;

        // Establish a TCP connection with a peer, and perform a handshake
        let mut stream = TcpStream::connect(peer_addr).context("connect to peer")?;
        let mut handshake = Handshake::new(info_hash);
        let handshake_bytes = handshake.as_bytes_mut();
        stream
            .write(handshake_bytes)
            .context("send handshake request")?;
        stream
            .read_exact(handshake_bytes)
            .context("read handshake response")?;
        assert_eq!(handshake.protocol_length, 19);
        assert_eq!(&handshake.protocol, b"BitTorrent protocol");
        assert_eq!(handshake.info_hash, info_hash);

        Ok(Self {
            torrent_file,
            stream,
        })
    }

    pub fn download_a_piece(&mut self, piece_index: u32) -> Result<Vec<u8>> {
        // Exchange multiple peer messages to download the file
        self.wait_message(MessageTag::Bitfield)
            .context("wait bitfield message")?;
        let interested_message = Message {
            tag: MessageTag::Interested,
            payload: Vec::new(),
        };
        self.send_message(interested_message)
            .context("send interested message")?;
        self.wait_message(MessageTag::Unchoke)
            .context("wait unchoke message")?;

        let mut piece_length = self.torrent_file.info.piece_length as u32;
        // Last piece might be smaller than other piece
        if (piece_index + 1) as u64 * piece_length as u64 > self.torrent_file.info.length {
            piece_length = (self.torrent_file.info.length % piece_length as u64) as u32
        }
        let mut all_blocks: Vec<u8> = Vec::with_capacity(piece_length as usize);
        let block_size = 1 << 14;
        let mut block_idx = 0;
        let mut remaining = piece_length;
        while remaining > 0 {
            // Calculate begin and length
            let begin = block_idx * block_size;
            let length: u32;
            if remaining > block_size {
                length = block_size;
                remaining -= block_size;
            } else {
                length = remaining;
                remaining = 0;
            }

            // Prepare message
            let mut request = Request::new(piece_index, begin, length);
            let request_bytes = Vec::from(request.as_bytes_mut());
            let message = Message {
                tag: MessageTag::Request,
                payload: request_bytes,
            };

            // Collect a single block
            self.send_message(message)
                .with_context(|| format!("send #{} request", block_idx))?;
            let res = self
                .wait_message(MessageTag::Piece)
                .with_context(|| format!("wait #{} piece", block_idx))?;
            let piece = Piece::ref_from_bytes(&res.payload[..])
                .expect("always get all Piece response fields from peer");
            assert_eq!(piece.index() as u32, piece_index);
            assert_eq!(piece.begin() as u32, begin);
            assert_eq!(piece.block().len() as u32, length);
            all_blocks.extend(piece.block());

            block_idx += 1;
        }
        assert_eq!(all_blocks.len() as u32, piece_length);

        Ok(all_blocks)
    }

    pub fn send_message(&mut self, message: Message) -> Result<()> {
        let mut buf = BytesMut::with_capacity(4 /* length */ + 1 /* tag */ + message.payload.len());
        buf.put_u32(1 + message.payload.len() as u32);
        buf.put_u8(message.tag as u8);
        buf.put(&message.payload[..]);
        self.stream.write(&buf).context("write message to stream")?;

        Ok(())
    }

    pub fn wait_message(&mut self, message_tag: MessageTag) -> Result<Message> {
        // Read the message length prefix
        let mut length_bytes = [0; 4];
        self.stream
            .read_exact(&mut length_bytes)
            .context("read message length prefix from stream")?;
        let length = u32::from_be_bytes(length_bytes) as usize;

        // Read the message id
        let mut id_bytes = [0; 1];
        self.stream
            .read_exact(&mut id_bytes)
            .context("read message id from stream")?;
        let tag: MessageTag = id_bytes[0].into();
        assert_eq!(tag, message_tag);

        if length == 1 {
            return Ok(Message {
                tag,
                payload: Vec::new(),
            });
        }

        // Read the payload
        let mut payload_bytes = Vec::with_capacity(length - 1);
        // have to resize the vec or its len() would be 0
        payload_bytes.resize_with(length - 1, || 0);
        self.stream
            .read_exact(&mut payload_bytes)
            .context("read message payload from stream")?;

        Ok(Message {
            tag,
            payload: payload_bytes.to_vec(),
        })
    }
}
