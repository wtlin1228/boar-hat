use crate::peer::Peer;
use crate::torrent_file::TorrentFile;
use crate::{torrent_file::parse_torrent_file, tracker::track};
use anyhow::{Context, Error};
use std::collections::HashMap;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::{fs, path::PathBuf};

pub struct Download;

impl Download {
    pub fn download_file(
        torrent_file_path: &PathBuf,
        output_file_path: &PathBuf,
    ) -> anyhow::Result<()> {
        // Read the torrent file to get the tracker URL
        let contents = fs::read(torrent_file_path).context("open file")?;
        let torrent_file = parse_torrent_file(&contents[..]).context("parse file")?;

        // Perform the tracker GET request to get a list of peers
        let track_result = track(&torrent_file).context("track peers")?;
        let peer_addr_list: Vec<String> = track_result
            .peer_addr_list
            .iter()
            .map(|addr| addr.to_string())
            .collect();

        // Get how many pieces need to be downloaded
        let hexed_pieces: Vec<String> = torrent_file.info.hex_pieces().context("hex pieces")?;
        let piece_count = hexed_pieces.len();

        println!(
            "have #{} pieces to download, have #{} peers to download from",
            piece_count,
            peer_addr_list.len()
        );

        // Start downloading n pieces from m peers
        let (tx, rx) = mpsc::channel::<(u32, Result<Vec<u8>, Error>)>();
        let mut peer_idx = 0;
        for piece_index in 0..piece_count {
            let peer_addr = peer_addr_list
                .get(peer_idx % peer_addr_list.len())
                .with_context(|| format!("get peer address for #{} piece", piece_index))?
                .clone();
            peer_idx += 1;
            let torrent_file = torrent_file.clone();
            let tx = tx.clone();
            Self::download_piece(piece_index as u32, peer_addr, torrent_file, tx);
        }

        let mut all_pieces: HashMap<usize, Vec<u8>> = HashMap::new();

        for received in rx {
            let (piece_index, piece_data_result) = received;
            match piece_data_result {
                Ok(piece_data) => {
                    println!("Got #{} piece", piece_index);
                    all_pieces.insert(piece_index as usize, piece_data);

                    // All pieces are downloaded, don't need to receive data anymore
                    if all_pieces.len() == piece_count {
                        break;
                    }
                }
                Err(_) => {
                    // println!("failed to download #{} piece, reschedule...", piece_index);
                    let peer_addr = peer_addr_list
                        .get(peer_idx % peer_addr_list.len())
                        .with_context(|| format!("get peer address for #{} piece", piece_index))?
                        .clone();
                    peer_idx += 1;
                    let torrent_file = torrent_file.clone();
                    let tx = tx.clone();
                    Self::download_piece(piece_index, peer_addr, torrent_file, tx);
                }
            }
        }

        // Aggregate all pieces and output to the target file
        let mut aggregated_data: Vec<u8> = Vec::with_capacity(torrent_file.info.length as usize);
        for i in 0..hexed_pieces.len() {
            let piece = all_pieces.insert(i, vec![]).unwrap();
            aggregated_data.extend(piece);
        }
        fs::write(&output_file_path, aggregated_data)
            .with_context(|| format!("write the aggregated data to file {:?}", output_file_path))?;

        Ok(())
    }

    fn download_piece(
        piece_index: u32,
        peer_addr: String,
        torrent_file: TorrentFile,
        tx: Sender<(u32, Result<Vec<u8>, Error>)>,
    ) -> () {
        thread::spawn(move || {
            // println!(
            //     "trying to download #{} piece from {}",
            //     piece_index, peer_addr
            // );

            // Connect to peer
            let peer = Peer::new(peer_addr.clone(), torrent_file);
            if peer.is_err() {
                tx.send((
                    piece_index,
                    Err(Error::msg(format!("fail to connect to peer {}", peer_addr))),
                ))
                .unwrap();
                return;
            }
            let mut peer = peer.unwrap();

            // Download a piece
            let piece = peer.download_a_piece(piece_index);
            if piece.is_err() {
                tx.send((
                    piece_index,
                    Err(Error::msg(format!(
                        "fail to download #{} piece",
                        piece_index
                    ))),
                ))
                .unwrap();
                return;
            }
            let piece = piece.unwrap();

            // Send downloaded piece back to the main thread
            tx.send((piece_index, Ok(piece))).unwrap();
        });
    }
}
