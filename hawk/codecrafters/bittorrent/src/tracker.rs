use anyhow::{Context, Ok, Result};
use std::net::Ipv4Addr;

use crate::decoder::{decode, Decoded};
use crate::torrent_file::TorrentFile;

#[derive(Debug, PartialEq)]
pub struct TrackerResponse {
    pub complete: i64,
    pub min_interval: i64,
    pub incomplete: i64,
    pub interval: i64,
    pub peer_addr_list: Vec<PeerAddr>,
}

#[derive(Debug, PartialEq)]
pub struct PeerAddr {
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl PeerAddr {
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

pub fn track(torrent_file: &TorrentFile) -> Result<TrackerResponse> {
    let url = get_request_url(&torrent_file).context("get url")?;
    let response_in_bytes = &reqwest::blocking::get(url)
        .context("request the url")?
        .bytes()
        .context("read request as bytes")?[..];
    parse_response(response_in_bytes)
}

fn get_request_url(torrent_file: &TorrentFile) -> Result<String> {
    let mut url = torrent_file.announce.to_owned();
    let url_encoded_info_hash: String = torrent_file
        .info
        .url_encoded_hash_info()
        .context("get url encoded hash info")?;
    url.push_str(&format!("?info_hash={}", url_encoded_info_hash));
    url.push_str("&peer_id=00112233445566778899");
    url.push_str("&port=6881");
    url.push_str("&uploaded=0");
    url.push_str("&downloaded=0");
    url.push_str(&format!("&left={}", torrent_file.info.length));
    url.push_str("&compact=1");
    Ok(url)
}

fn parse_response(response: &[u8]) -> Result<TrackerResponse> {
    let decoded_value = decode(response).context("decode response")?.1;

    let mut complete: Option<i64> = None;
    let mut min_interval: Option<i64> = None;
    let mut incomplete: Option<i64> = None;
    let mut interval: Option<i64> = None;
    let mut peers: Option<Vec<PeerAddr>> = None;

    if let Decoded::Dictionary(dict) = decoded_value {
        if let Decoded::Integer(n) = dict.get("complete").context("should contain complete")? {
            complete = Some(n.to_owned());
        };
        if let Decoded::Integer(n) = dict
            .get("min interval")
            .context("should contain min_interval")?
        {
            min_interval = Some(n.to_owned());
        };
        if let Decoded::Integer(n) = dict
            .get("incomplete")
            .context("should contain incomplete")?
        {
            incomplete = Some(n.to_owned());
        };
        if let Decoded::Integer(n) = dict.get("interval").context("should contain interval")? {
            interval = Some(n.to_owned());
        };
        if let Decoded::String(info) = dict.get("peers").context("should contain peers")? {
            let mut vec: Vec<PeerAddr> = vec![];
            for chunk in info.chunks(6) {
                vec.push(PeerAddr {
                    ip: Ipv4Addr::new(chunk[0], chunk[1], chunk[2], chunk[3]),
                    port: ((chunk[4] as u16) << 8) | chunk[5] as u16,
                })
            }
            peers = Some(vec);
        }
    }

    Ok(TrackerResponse {
        complete: complete.context("get complete")?,
        min_interval: min_interval.context("get min interval")?,
        incomplete: incomplete.context("get incomplete")?,
        interval: interval.context("get interval")?,
        peer_addr_list: peers.context("get peers")?,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::torrent_file::TorrentFileInfo;

    #[test]
    fn create_url_from_torrent_file() {
        assert_eq!(
        get_request_url(&TorrentFile {
            announce: "http://bittorrent-test-tracker.codecrafters.io/announce".to_string(),
            info: TorrentFileInfo {
                name: "sample.txt".to_string(),
                piece_length: 32768,
                pieces:  vec![
                    232, 118, 246, 122, 42, 136, 134, 232, 243, 107, 19, 103, 38, 195, 15, 162,
                    151, 3, 2, 45, 110, 34, 117, 230, 4, 160, 118, 102, 86, 115, 110, 129, 255, 16,
                    181, 82, 4, 173, 141, 53, 240, 13, 147, 122, 2, 19, 223, 25, 130, 188, 141, 9,
                    114, 39, 173, 158, 144, 154, 204, 23,
                ],
                length: 92063,
            },
        })
        .unwrap(),
        "http://bittorrent-test-tracker.codecrafters.io/announce?info_hash=%d6%9f%91%e6%b2%ae%4c%54%24%68%d1%07%3a%71%d4%ea%13%87%9a%7f&peer_id=00112233445566778899&port=6881&uploaded=0&downloaded=0&left=92063&compact=1"
    )
    }

    #[test]
    fn create_track_response() {
        assert_eq!(
            parse_response(&[
                100, 56, 58, 99, 111, 109, 112, 108, 101, 116, 101, 105, 51, 101, 49, 48, 58, 105,
                110, 99, 111, 109, 112, 108, 101, 116, 101, 105, 49, 101, 56, 58, 105, 110, 116,
                101, 114, 118, 97, 108, 105, 54, 48, 101, 49, 50, 58, 109, 105, 110, 32, 105, 110,
                116, 101, 114, 118, 97, 108, 105, 54, 48, 101, 53, 58, 112, 101, 101, 114, 115, 49,
                56, 58, 178, 62, 82, 89, 201, 14, 165, 232, 33, 77, 201, 11, 178, 62, 85, 20, 201,
                33, 101
            ])
            .unwrap(),
            TrackerResponse {
                complete: 3,
                min_interval: 60,
                incomplete: 1,
                interval: 60,
                peer_addr_list: vec![
                    PeerAddr {
                        ip: Ipv4Addr::new(178, 62, 82, 89),
                        port: 51470
                    },
                    PeerAddr {
                        ip: Ipv4Addr::new(165, 232, 33, 77),
                        port: 51467
                    },
                    PeerAddr {
                        ip: Ipv4Addr::new(178, 62, 85, 20),
                        port: 51489
                    }
                ]
            }
        )
    }
}
