use anyhow::{Context, Ok, Result};
use bittorrent_starter_rust::decoder::decode_bencoded_value;
use bittorrent_starter_rust::download::Download;
use bittorrent_starter_rust::handshake::Handshake;
use bittorrent_starter_rust::peer::Peer;
use bittorrent_starter_rust::torrent_file::parse_torrent_file;
use bittorrent_starter_rust::tracker::track;
use clap::{Parser, Subcommand};
use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}
#[derive(Debug, Subcommand)]
enum Command {
    Decode {
        encoded_value: String,
    },
    Info {
        file_path: PathBuf,
    },
    Peers {
        file_path: PathBuf,
    },
    Handshake {
        file_path: PathBuf,
        peer: SocketAddr,
    },
    #[command(name = "download_piece")]
    DownloadPiece {
        #[arg(short)]
        output_file_path: PathBuf,
        file_path: PathBuf,
        piece_index: u32,
    },
    Download {
        #[arg(short)]
        output_file_path: PathBuf,
        file_path: PathBuf,
    },
}

fn main() -> Result<()> {
    match Args::parse().command {
        Command::Decode { encoded_value } => {
            let decoded_value =
                decode_bencoded_value(encoded_value.as_bytes()).context("decode value")?;
            println!("{}", decoded_value.to_string());
        }
        Command::Info { file_path } => {
            let contents = fs::read(file_path).context("open file")?;
            let torrent_file = parse_torrent_file(&contents[..]).context("parse file")?;
            println!("Tracker URL: {}", torrent_file.announce);
            println!("Length: {}", torrent_file.info.length);
            println!(
                "Info Hash: {}",
                torrent_file.info.hex_info().context("hash info")?
            );
            println!("Piece Length: {}", torrent_file.info.piece_length);
            println!("Piece Hashes");
            for s in torrent_file.info.hex_pieces().context("hex pieces")? {
                println!("{}", s);
            }
        }
        Command::Peers { file_path } => {
            let contents = fs::read(file_path).context("open file")?;
            let torrent_file = parse_torrent_file(&contents[..]).context("parse file")?;
            let track_result = track(&torrent_file).context("track peers")?;
            for peer_addr in track_result.peer_addr_list {
                println!("{}", peer_addr.to_string());
            }
        }
        Command::Handshake { file_path, peer } => {
            let contents = fs::read(file_path).context("open file")?;
            let torrent_file = parse_torrent_file(&contents[..]).context("parse file")?;
            let info_hash = torrent_file.info.hash_info().context("hash info")?;
            let mut stream = TcpStream::connect(peer).context("connect to peer")?;
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
            println!("Peer ID: {}", hex::encode(&handshake.peer_id));
        }
        Command::DownloadPiece {
            output_file_path,
            file_path,
            piece_index,
        } => {
            // Read the torrent file to get the tracker URL
            let contents = fs::read(file_path).context("open file")?;
            let torrent_file = parse_torrent_file(&contents[..]).context("parse file")?;

            // Perform the tracker GET request to get a list of peers
            let track_result = track(&torrent_file).context("track peers")?;
            let first_peer_addr = track_result
                .peer_addr_list
                .first()
                .context("get first peer")?
                .to_string();

            // Download the piece from the first peer
            let mut peer = Peer::new(first_peer_addr, torrent_file).context("create peer")?;
            let piece = peer
                .download_a_piece(piece_index)
                .context("download a piece")?;

            // Write downloaded piece to output file
            fs::write(&output_file_path, piece).with_context(|| {
                format!("write the downloaded piece to file {:?}", output_file_path)
            })?;

            println!(
                "Piece {} downloaded to {}.",
                piece_index,
                output_file_path.display()
            );
        }
        Command::Download {
            output_file_path,
            file_path,
        } => {
            Download::download_file(&file_path, &output_file_path)
                .with_context(|| format!("download {:?} to {:?}", file_path, output_file_path))?;
        }
    }
    Ok(())
}
