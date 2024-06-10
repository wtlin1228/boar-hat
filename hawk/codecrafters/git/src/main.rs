use anyhow::Context;
use clap::{Parser, Subcommand};
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init,

    CatFile {
        /// Pretty-print the contents of <object> based on its type.
        #[arg(short)]
        pretty_print: bool,

        /// The name of the object to show. For a more complete list of ways to spell object names, see the
        /// "SPECIFYING REVISIONS" section in gitrevisions(7).
        object: String,
    },

    HashObject {
        /// Actually write the object into the object database.
        #[arg(short)]
        write: bool,

        file: String,
    },

    LsTree {
        /// list only filenames
        #[arg(long)]
        name_only: bool,

        tree_ish: String,
    },

    WriteTree,

    CommitTree {
        tree: String,

        /// Each -p indicates the id of a parent commit object.
        #[arg(short)]
        parent: Option<String>,

        /// A paragraph in the commit log message. This can be given more than once and each <message> becomes its own paragraph
        #[arg(short)]
        message: String,
    },

    Clone {
        /// The (possibly remote) repository to clone from. See the GIT URLS section below for more information on specifying repositories.
        repository: String,

        /// The name of a new directory to clone into. The "humanish" part of the source repository is used if no directory is explicitly given (repo
        /// for /path/to/repo.git and foo for host.xz:foo/.git). Cloning into an existing directory is only allowed if the directory is empty.
        directory: String,
    },
}

fn main() -> anyhow::Result<()> {
    match Args::parse().command {
        Command::Init => {
            fs::create_dir(".git")?;
            fs::create_dir(".git/objects")?;
            fs::create_dir(".git/refs")?;
            fs::write(".git/HEAD", "ref: refs/heads/main\n")?;
            println!("Initialized git directory");
        }
        Command::CatFile {
            pretty_print,
            object,
        } => {
            anyhow::ensure!(pretty_print, "only support -p");

            let mut object = Object::read(&object, None)?;
            match object.kind {
                Kind::Blob => {
                    let stdout = io::stdout();
                    let mut handle = stdout.lock();
                    std::io::copy(&mut object.reader, &mut handle)
                        .context("write blob to stdout")?;
                }
                _ => todo!(),
            }
        }
        Command::HashObject { write, file } => {
            anyhow::ensure!(write, "only support -w");

            let hash = Object::write(Kind::Blob, &fs::read(file)?, None)?;
            println!("{}", hex::encode(hash));
        }
        Command::LsTree {
            name_only,
            tree_ish,
        } => {
            let mut object = Object::read(&tree_ish, None)?;
            match object.kind {
                Kind::Tree => {
                    let stdout = io::stdout();
                    let mut handle = stdout.lock();
                    let mut buf = Vec::new();
                    let mut hashbuf = [0; 20];
                    loop {
                        buf.clear();
                        let n = object
                            .reader
                            .read_until(0, &mut buf)
                            .context("read next tree entry")?;
                        if n == 0 {
                            break;
                        }
                        object
                            .reader
                            .read_exact(&mut hashbuf)
                            .context("read tree entry hash")?;

                        let without_ending_null = buf.split_last().context("split last")?.1;
                        let mut iter = without_ending_null.splitn(2, |&b| b == b' ');
                        let _mode = iter.next().context("get tree entry mode")?;
                        let name = iter.next().context("get tree entry name")?;

                        match name_only {
                            true => {
                                handle.write_all(name).context("write name to console")?;
                                write!(handle, "\n").context("write new line to console")?;
                            }
                            false => todo!(),
                        }
                    }
                }
                _ => todo!(),
            }
        }
        Command::WriteTree => {
            let path = Path::new(".").to_path_buf();
            let hash = write_tree(&path)?;
            if let Some(hash) = hash {
                let hash_hex = hex::encode(hash);
                println!("{}", hash_hex);
            }
        }
        Command::CommitTree {
            tree,
            parent,
            message,
        } => {
            let mut content: Vec<u8> = Vec::new();
            content.extend(format!("tree {}\n", tree).as_bytes());
            if let Some(parent) = parent {
                content.extend(format!("parent {}\n", parent).as_bytes());
            }
            content.extend(b"author wtlin1228 <wtlin1228@gmail.com> 1717228746 +0800\n");
            content.extend(b"committer wtlin1228 <wtlin1228@gmail.com> 1717228746 +0800\n");
            content.push(b'\n');
            content.extend(message.as_bytes());
            content.push(b'\n');

            let hash = Object::write(Kind::Commit, &content, None)?;
            println!("{}", hex::encode(hash));
        }
        Command::Clone {
            repository,
            directory,
        } => {
            let path = PathBuf::from(&directory).join(".git");
            fs::create_dir_all(&path)?;
            fs::create_dir(path.join("objects"))?;
            fs::create_dir(path.join("refs"))?;

            // reference:
            // - https://git-scm.com/docs/http-protocol/2.16.6
            // - https://github.com/git/git/blob/795ea8776befc95ea2becd8020c7a284677b4161/Documentation/gitprotocol-pack.txt
            // - https://github.com/git/git/blob/795ea8776befc95ea2becd8020c7a284677b4161/Documentation/gitprotocol-pack.txt
            // - https://github.com/git/git/blob/795ea8776befc95ea2becd8020c7a284677b4161/Documentation/gitprotocol-pack.txt
            // - https://medium.com/@concertdaw/sneaky-git-number-encoding-ddcc5db5329f

            let reference_discovery_url =
                format!("{}/info/refs?service=git-upload-pack", repository);
            let mut advertised_refs = reqwest::blocking::get(&reference_discovery_url)
                .context(format!("GET {}", reference_discovery_url))?;
            // If HEAD is a valid ref, HEAD MUST appear as the first advertised
            // ref.  If HEAD is not a valid ref, HEAD MUST NOT appear in the
            // advertisement list at all, but other refs may still appear.
            read_pkt_line(&mut advertised_refs)?; // 001e# service=git-upload-pack
            read_pkt_line(&mut advertised_refs)?; // 0000
            let first_ref = read_pkt_line(&mut advertised_refs).context("read first-ref")?;
            let head_object_id =
                String::from_utf8(first_ref[..40].to_vec()).context("read head object id")?;

            let upload_request = format!("0032want {}\n00000009done\n", head_object_id);
            let mut server_response = reqwest::blocking::Client::new()
                .post(format!("{}/git-upload-pack", repository))
                .header("Content-Type", "application/x-git-upload-pack-request")
                .body(upload_request)
                .send()
                .context(format!("POST {}/git-upload-pack\n", repository))?;

            let mut nak = [0; "0008NAK\n".len()];
            server_response
                .read_exact(&mut nak)
                .context("read `0008NAK\\n`")?;
            let mut signature = [0; "PACK".len()];
            server_response
                .read_exact(&mut signature)
                .context("read signature `PACK`")?;
            let mut version_number = [0; 4];
            server_response
                .read_exact(&mut version_number)
                .context("read version-number")?;
            let mut num_objects = [0; 4];
            server_response
                .read_exact(&mut num_objects)
                .context("read number of objects contained in the pack")?;
            let num_objects = u32::from_be_bytes(num_objects);

            let mut pack = Vec::new();
            server_response.read_to_end(&mut pack)?;
            let checksum = pack.split_off(pack.len() - 20);
            anyhow::ensure!(hex::encode(checksum).len() == 40);

            let mut object_entries = &pack[..];
            for _ in 0..num_objects {
                let mut current_byte: &u8;
                (current_byte, object_entries) = object_entries
                    .split_first()
                    .context("get object's first byte")?;
                let mut msb = current_byte & 0b1000_0000;
                let object_type = (current_byte & 0b0111_0000) >> 4;
                let mut object_size = (current_byte & 0b0000_1111) as usize;
                let mut shift = 4;
                while msb > 0 {
                    (current_byte, object_entries) = object_entries
                        .split_first()
                        .context("read one more byte since the last MSB is 1")?;
                    msb = current_byte & 0b1000_0000;
                    object_size += ((current_byte & 0b0111_1111) as usize) << shift;
                    shift += 7;
                }
                // - OBJ_COMMIT (1)
                // - OBJ_TREE (2)
                // - OBJ_BLOB (3)
                // - OBJ_TAG (4)
                // - OBJ_OFS_DELTA (6)
                // - OBJ_REF_DELTA (7)
                match object_type {
                    // OBJ_COMMIT (1) | OBJ_TREE (2) | OBJ_BLOB (3)
                    1 | 2 | 3 => {
                        let mut z = ZlibDecoder::new(object_entries);
                        let mut data = vec![0u8; object_size];
                        z.read_exact(&mut data).context("read object's content")?;
                        Object::write(
                            Kind::from_object_type(object_type)?,
                            &data,
                            Some(&directory),
                        )?;
                        if object_size == 0 {
                            // zlib compresses empty bytes into 8 bytes, not 0 bytes
                            object_entries = &object_entries[8..];
                        } else {
                            object_entries = &object_entries[z.total_in() as usize..];
                        }
                    }
                    // OBJ_TAG (4)
                    4 => anyhow::bail!("invalid object type: OBJ_TAG (4)"),
                    // OBJ_OFS_DELTA (6)
                    6 => anyhow::bail!("invalid object type: OBJ_OFS_DELTA (6)"),
                    // OBJ_REF_DELTA (7)
                    7 => {
                        // the object meta information (which we already parsed earlier) is followed
                        // by the 20-byte name of the base object
                        object_entries = &object_entries[20..];
                        let mut z = ZlibDecoder::new(object_entries);
                        let mut data = vec![0u8; object_size];
                        z.read_exact(&mut data).context("read object's content")?;
                        object_entries = &object_entries[z.total_in() as usize..];
                    }
                    _ => anyhow::bail!("invalid object type: {}", object_type),
                }
            }

            fs::write(path.join("HEAD"), format!("ref: {}\n", head_object_id))?;
            println!("Initialized git directory");
            Object::create_file(&head_object_id, &directory, &PathBuf::from(&directory))?;
        }
    }
    Ok(())
}

fn read_pkt_line(reader: &mut impl Read) -> anyhow::Result<Vec<u8>> {
    let mut pkt_length = [0; 4];
    reader
        .read_exact(&mut pkt_length)
        .context("read pkt-length")?;
    let pkt_length = u32::from_str_radix(&String::from_utf8(pkt_length.to_vec())?, 16)?;
    if pkt_length == 0 {
        return Ok(Vec::new());
    }
    let mut line = vec![0u8; pkt_length as usize - 4];
    reader
        .read_exact(&mut line)
        .context(format!("read line (length = {})", line.len()))?;
    Ok(line)
}

fn encode(bytes: &[u8]) -> io::Result<Vec<u8>> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(bytes).unwrap();
    e.finish()
}

enum Kind {
    Blob,
    Tree,
    Commit,
}

impl Kind {
    fn from(s: &str) -> anyhow::Result<Self> {
        match s {
            "blob" => Ok(Self::Blob),
            "tree" => Ok(Self::Tree),
            "commit" => Ok(Self::Commit),
            _ => anyhow::bail!("invalid kind: {}", s),
        }
    }

    fn from_object_type(n: u8) -> anyhow::Result<Self> {
        match n {
            1 => Ok(Self::Commit),
            2 => Ok(Self::Tree),
            3 => Ok(Self::Blob),
            _ => anyhow::bail!("invalid kind: {}", n),
        }
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Blob => write!(f, "blob"),
            Kind::Tree => write!(f, "tree"),
            Kind::Commit => write!(f, "commit"),
        }
    }
}

struct Object<R> {
    kind: Kind,
    #[allow(dead_code)]
    expected_size: u64,
    reader: R,
}

impl Object<()> {
    fn read(hash: &str, directory: Option<&str>) -> anyhow::Result<Object<impl BufRead>> {
        let (folder, filename) = hash.split_at(2);
        let file_path = match directory {
            Some(dir) => format!("{}/.git/objects/{}/{}", dir, folder, filename),
            None => format!(".git/objects/{}/{}", folder, filename),
        };
        let f = fs::File::open(&file_path).context(format!("open file: {}", file_path))?;
        let z = ZlibDecoder::new(f);
        let mut reader = BufReader::new(z);
        let mut buf = Vec::new();
        reader
            .read_until(0, &mut buf)
            .context("read header from .git/objects")?;
        let without_ending_null = buf.split_last().context("split last")?.1;
        let header = String::from_utf8(without_ending_null.to_vec())
            .context("object header isn't valid UTF-8")?;
        let (kind, size) = header.split_once(' ').context(format!(
            "object header isn't `<kink> <size>\0`: {:?}",
            header
        ))?;
        let size = size
            .parse::<u64>()
            .context(format!("object header has invalid size: {}", size))?;
        Ok(Object {
            kind: Kind::from(kind)?,
            expected_size: size,
            reader: reader.take(size),
        })
    }

    fn write(kind: Kind, content: &[u8], directory: Option<&str>) -> anyhow::Result<Vec<u8>> {
        let mut git_object_formatted_content = format!("{} {}\0", kind, content.len()).into_bytes();
        git_object_formatted_content.extend(content);

        // do hash
        let mut hasher = Sha1::new();
        hasher.update(&git_object_formatted_content[..]);
        let hash = hasher.finalize();
        let hash_hex = hex::encode(hash);

        // write git object
        let target_dir = match directory {
            Some(dir) => format!("{}/.git/objects/{}", dir, &hash_hex[..2]),
            None => format!(".git/objects/{}", &hash_hex[..2]),
        };
        fs::create_dir_all(target_dir.as_str())?;
        fs::write(
            format!("{}/{}", target_dir, &hash_hex[2..]),
            encode(&git_object_formatted_content[..])?,
        )?;

        Ok(hash.to_vec())
    }

    fn create_file(hash_hex: &str, directory: &str, path: &PathBuf) -> anyhow::Result<()> {
        let mut object = Object::read(hash_hex, Some(directory))?;
        match object.kind {
            Kind::Blob => {
                let mut f = fs::File::create(path)?;
                std::io::copy(&mut object.reader, &mut f).context("write blob to file")?;
            }
            Kind::Tree => {
                let mut buf = Vec::new();
                let mut hashbuf = [0; 20];
                loop {
                    buf.clear();
                    let n = object
                        .reader
                        .read_until(0, &mut buf)
                        .context("read next tree entry")?;
                    if n == 0 {
                        break;
                    }
                    object
                        .reader
                        .read_exact(&mut hashbuf)
                        .context("read tree entry hash")?;

                    let without_ending_null = buf.split_last().context("split last")?.1;
                    let mut iter = without_ending_null.splitn(2, |&b| b == b' ');
                    let mode = iter.next().context("get tree entry mode")?;
                    let name = iter.next().context("get tree entry name")?;
                    let path = path.join(String::from_utf8(name.to_vec())?);
                    if mode == b"40000" {
                        fs::create_dir(&path)?;
                    }
                    Object::create_file(&hex::encode(hashbuf), directory, &path)?;
                }
            }
            Kind::Commit => {
                let mut tree_label = [0; "tree ".len()];
                object.reader.read_exact(&mut tree_label)?;
                let mut hash_hex = [0; 40];
                object.reader.read_exact(&mut hash_hex)?;
                Object::create_file(&String::from_utf8(hash_hex.to_vec())?, directory, path)?;
            }
        }
        Ok(())
    }
}

fn write_tree(path: &PathBuf) -> anyhow::Result<Option<Vec<u8>>> {
    anyhow::ensure!(path.is_dir(), "write to tree in path: {:?}", path);

    let dir = path.read_dir().unwrap();
    let mut entries = dir.fold(Vec::new(), |mut acc, x| match x {
        Ok(entry) => {
            if entry.file_name() != ".git" {
                acc.push(entry);
            }
            acc
        }
        Err(_) => acc,
    });
    entries.sort_by(|a, b| a.file_name().partial_cmp(&b.file_name()).unwrap());

    let mut tree_content: Vec<u8> = Vec::new();
    for entry in entries {
        let file_name = entry.file_name();
        let entry_path = entry.path();
        match entry_path.is_dir() {
            true => {
                let hash = write_tree(&entry_path)
                    .context(format!("write tree on path: {:?}", entry_path))?;

                // append tree entry
                if let Some(hash) = hash {
                    tree_content.extend(b"40000 ");
                    tree_content.extend(file_name.as_encoded_bytes());
                    tree_content.push(0);
                    tree_content.extend(hash);
                }
            }
            false => {
                let hash = Object::write(Kind::Blob, &fs::read(&entry_path)?, None)?;

                // append tree entry
                tree_content.extend(b"100644 ");
                tree_content.extend(file_name.as_encoded_bytes());
                tree_content.push(0);
                tree_content.extend(hash);
            }
        }
    }

    if tree_content.len() == 0 {
        return Ok(None);
    }

    let hash = Object::write(Kind::Tree, &tree_content, None)?;
    Ok(Some(hash))
}
