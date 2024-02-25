use anyhow::{Context, Ok, Result};
use serde::Serialize;
use sha1::{Digest, Sha1};

use crate::decoder::{decode, Decoded};

#[derive(PartialEq, Debug, Clone)]
pub struct TorrentFile {
    pub announce: String,
    pub info: TorrentFileInfo,
}

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct TorrentFileInfo {
    pub name: String,
    #[serde(rename = "piece length")]
    pub piece_length: u64,
    #[serde(with = "serde_bytes")]
    pub pieces: Vec<u8>,
    pub length: u64,
}

impl TorrentFileInfo {
    pub fn hash_info(&self) -> Result<[u8; 20]> {
        let bencoded_info_dictionary =
            serde_bencode::to_bytes(&self).context("hash info dictionary")?;
        let mut hasher = Sha1::new();
        hasher.update(bencoded_info_dictionary);
        Ok(hasher.finalize().into())
    }

    pub fn hex_info(&self) -> Result<String> {
        Ok(hex::encode(self.hash_info().context("get hash info")?))
    }

    pub fn url_encoded_hash_info(&self) -> Result<String> {
        Ok(self.hash_info().context("get hash info")?.iter().fold(
            "".to_string(),
            |mut acc, &byte| {
                acc.push_str("%");
                acc.push_str(&hex::encode([byte]));
                acc
            },
        ))
    }

    pub fn hex_pieces(&self) -> Result<Vec<String>> {
        Ok(self
            .pieces
            .chunks(20)
            .map(|chunk| hex::encode(chunk))
            .collect())
    }
}

pub fn parse_torrent_file(contents: &[u8]) -> Result<TorrentFile> {
    let decoded_value = decode(contents).context("decode file contents")?.1;

    let mut announce: Option<String> = None;
    let mut length: Option<u64> = None;
    let mut name: Option<String> = None;
    let mut piece_length: Option<u64> = None;
    let mut pieces: Option<Vec<u8>> = None;
    if let Decoded::Dictionary(dict) = decoded_value {
        if let Decoded::String(s) = dict.get("announce").context("should contain announce")? {
            announce = Some(
                std::str::from_utf8(s)
                    .context("announce isn't in valid UTF-8 format")?
                    .to_string(),
            );
        };
        if let Decoded::Dictionary(info) = dict.get("info").context("should contain info")? {
            if let Decoded::Integer(n) = info.get("length").context("should contain length")? {
                length = Some(n.to_owned() as u64);
            }
            if let Decoded::String(s) = info.get("name").context("should contain name")? {
                name = Some(
                    std::str::from_utf8(s)
                        .context("name isn't in valid UTF-8 format")?
                        .to_string(),
                );
            }
            if let Decoded::Integer(n) = info
                .get("piece length")
                .context("should contain piece length")?
            {
                piece_length = Some(n.to_owned() as u64);
            }
            if let Decoded::String(s) = info.get("pieces").context("should contain pieces")? {
                pieces = Some(s.to_vec());
            }
        }
    }

    Ok(TorrentFile {
        announce: announce.context("get announce from torrent file")?,
        info: TorrentFileInfo {
            length: length.context("get info.length from torrent file")?,
            name: name.context("get info.name from torrent file")?,
            piece_length: piece_length.context("get info.piece_length")?,
            pieces: pieces.context("get info.pieces")?,
        },
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_the_torrent_file() {
        assert_eq!(
            parse_torrent_file(&[
                100, 56, 58, 97, 110, 110, 111, 117, 110, 99, 101, 53, 53, 58, 104, 116, 116, 112,
                58, 47, 47, 98, 105, 116, 116, 111, 114, 114, 101, 110, 116, 45, 116, 101, 115,
                116, 45, 116, 114, 97, 99, 107, 101, 114, 46, 99, 111, 100, 101, 99, 114, 97, 102,
                116, 101, 114, 115, 46, 105, 111, 47, 97, 110, 110, 111, 117, 110, 99, 101, 49, 48,
                58, 99, 114, 101, 97, 116, 101, 100, 32, 98, 121, 49, 51, 58, 109, 107, 116, 111,
                114, 114, 101, 110, 116, 32, 49, 46, 49, 52, 58, 105, 110, 102, 111, 100, 54, 58,
                108, 101, 110, 103, 116, 104, 105, 57, 50, 48, 54, 51, 101, 52, 58, 110, 97, 109,
                101, 49, 48, 58, 115, 97, 109, 112, 108, 101, 46, 116, 120, 116, 49, 50, 58, 112,
                105, 101, 99, 101, 32, 108, 101, 110, 103, 116, 104, 105, 51, 50, 55, 54, 56, 101,
                54, 58, 112, 105, 101, 99, 101, 115, 54, 48, 58, 232, 118, 246, 122, 42, 136, 134,
                232, 243, 107, 19, 103, 38, 195, 15, 162, 151, 3, 2, 45, 110, 34, 117, 230, 4, 160,
                118, 102, 86, 115, 110, 129, 255, 16, 181, 82, 4, 173, 141, 53, 240, 13, 147, 122,
                2, 19, 223, 25, 130, 188, 141, 9, 114, 39, 173, 158, 144, 154, 204, 23, 101, 101
            ])
            .unwrap(),
            TorrentFile {
                announce: "http://bittorrent-test-tracker.codecrafters.io/announce".to_string(),
                info: TorrentFileInfo {
                    name: "sample.txt".to_string(),
                    piece_length: 32768,
                    pieces: vec![
                        232, 118, 246, 122, 42, 136, 134, 232, 243, 107, 19, 103, 38, 195, 15, 162,
                        151, 3, 2, 45, 110, 34, 117, 230, 4, 160, 118, 102, 86, 115, 110, 129, 255,
                        16, 181, 82, 4, 173, 141, 53, 240, 13, 147, 122, 2, 19, 223, 25, 130, 188,
                        141, 9, 114, 39, 173, 158, 144, 154, 204, 23
                    ],
                    length: 92063
                }
            }
        );
    }

    #[test]
    fn hash_the_torrent_file_info() {
        assert_eq!(
            TorrentFileInfo {
                name: "sample.txt".to_string(),
                piece_length: 32768,
                pieces: vec![
                    232, 118, 246, 122, 42, 136, 134, 232, 243, 107, 19, 103, 38, 195, 15, 162,
                    151, 3, 2, 45, 110, 34, 117, 230, 4, 160, 118, 102, 86, 115, 110, 129, 255, 16,
                    181, 82, 4, 173, 141, 53, 240, 13, 147, 122, 2, 19, 223, 25, 130, 188, 141, 9,
                    114, 39, 173, 158, 144, 154, 204, 23
                ],
                length: 92063
            }
            .hash_info()
            .unwrap(),
            [
                214, 159, 145, 230, 178, 174, 76, 84, 36, 104, 209, 7, 58, 113, 212, 234, 19, 135,
                154, 127
            ]
        )
    }

    #[test]
    fn hex_the_torrent_file_info() {
        assert_eq!(
            TorrentFileInfo {
                name: "sample.txt".to_string(),
                piece_length: 32768,
                pieces: vec![
                    232, 118, 246, 122, 42, 136, 134, 232, 243, 107, 19, 103, 38, 195, 15, 162,
                    151, 3, 2, 45, 110, 34, 117, 230, 4, 160, 118, 102, 86, 115, 110, 129, 255, 16,
                    181, 82, 4, 173, 141, 53, 240, 13, 147, 122, 2, 19, 223, 25, 130, 188, 141, 9,
                    114, 39, 173, 158, 144, 154, 204, 23
                ],
                length: 92063
            }
            .hex_info()
            .unwrap(),
            "d69f91e6b2ae4c542468d1073a71d4ea13879a7f"
        )
    }

    #[test]
    fn url_encode_the_torrent_file_info() {
        assert_eq!(
            TorrentFileInfo {
                name: "sample.txt".to_string(),
                piece_length: 32768,
                pieces: vec![
                    232, 118, 246, 122, 42, 136, 134, 232, 243, 107, 19, 103, 38, 195, 15, 162,
                    151, 3, 2, 45, 110, 34, 117, 230, 4, 160, 118, 102, 86, 115, 110, 129, 255, 16,
                    181, 82, 4, 173, 141, 53, 240, 13, 147, 122, 2, 19, 223, 25, 130, 188, 141, 9,
                    114, 39, 173, 158, 144, 154, 204, 23
                ],
                length: 92063
            }
            .url_encoded_hash_info()
            .unwrap(),
            "%d6%9f%91%e6%b2%ae%4c%54%24%68%d1%07%3a%71%d4%ea%13%87%9a%7f"
        )
    }

    #[test]
    fn hax_the_torrent_file_pieces() {
        assert_eq!(
            TorrentFileInfo {
                name: "sample.txt".to_string(),
                piece_length: 32768,
                pieces: vec![
                    232, 118, 246, 122, 42, 136, 134, 232, 243, 107, 19, 103, 38, 195, 15, 162,
                    151, 3, 2, 45, 110, 34, 117, 230, 4, 160, 118, 102, 86, 115, 110, 129, 255, 16,
                    181, 82, 4, 173, 141, 53, 240, 13, 147, 122, 2, 19, 223, 25, 130, 188, 141, 9,
                    114, 39, 173, 158, 144, 154, 204, 23
                ],
                length: 92063
            }
            .hex_pieces()
            .unwrap(),
            vec![
                "e876f67a2a8886e8f36b136726c30fa29703022d",
                "6e2275e604a0766656736e81ff10b55204ad8d35",
                "f00d937a0213df1982bc8d097227ad9e909acc17",
            ]
        )
    }
}
