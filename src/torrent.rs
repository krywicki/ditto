use std::path::PathBuf;
use chrono::NaiveDateTime;
use std::{fs, io, fmt};
use std::io::Read;

use serde_bencode::{ de };

use crate::helpers::deserialize::{serde_datetime, serde_pathbuf, serde_announce_list};

/*******************************************
 * Result Structs
 *******************************************/

 #[derive(Debug)]
pub enum ErrorKind {
    IOError,
    DecodeFailure
}

impl ErrorKind {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::IOError => "io error",
            ErrorKind::DecodeFailure => "decode failure"
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub msg: String
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error {
            kind: ErrorKind::IOError,
            msg: error.to_string()
        }
    }
}

impl From<serde_bencode::Error> for Error {
    fn from(error: serde_bencode::Error) -> Self {
        Error {
            kind: ErrorKind::DecodeFailure,
            msg: error.to_string()
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/*******************************************
 * Torrent Structs
 *******************************************/

#[derive(Debug, Deserialize)]
pub struct File {
    #[serde(with="serde_pathbuf")]
    pub path: PathBuf,

    pub length: i64,

    pub md5sum: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub name: String,

    #[serde(with="serde_bytes")]
    pub pieces: Vec<u8>,

    #[serde(rename="piece length")]
    pub piece_length: i64,

    pub md5sum: Option<String>,

    pub length: Option<i64>,

    pub files: Option<Vec<File>>,

    //pub path: Option<PathBuf>
}

#[derive(Debug, Deserialize)]
pub struct Torrent {
    #[serde(rename="creation date", with="serde_datetime")]
    pub creation_date: Option<NaiveDateTime>,

    #[serde(rename="announce-list", flatten)]
    pub announce_list: Option<Vec<String>>,

    #[serde(rename="created by")]
    pub created_by: Option<String>,

    pub announce: Option<String>,

    pub info: Info,

    pub comment: Option<String>,
}

impl Torrent {
    pub fn from_file<T: AsRef<str>>(f: T) -> Result<Torrent> {
        let file = fs::File::open(f.as_ref())?;
        let mut reader = io::BufReader::new(file);
        let mut bytes = vec!();

        reader.read_to_end(&mut bytes)?;
        Ok(de::from_bytes::<Torrent>(&bytes)?)
    }
}

/*******************************************
 * Display Impl
 *******************************************/

fn display_announce_list(t: &Torrent) -> String {
    let mut s = String::new();
    if let Some(ref announce_list) = t.announce_list {
        for announce in announce_list {
            s.push_str(format!("\n{:>15} {}", "", announce).as_ref());
        }
    }

    return s;
}

fn to_mem_size(size:i64) -> String {
    const KIB:i64 = 1024;
    const MIB:i64 = KIB * 1000;
    const GIB:i64 = MIB * 1000;
    const TIB:i64 = GIB * 1000;

    if size >= KIB && size < MIB {
        return format!("{} KiB", size / KIB);
    } else if size >= MIB && size < GIB {
        return format!("{} MiB", size / MIB);
    } else if size >= GIB && size < TIB {
        return format!("{} GiB", size / GIB);
    } else {
        return format!("{} TiB", size / TIB)
    }
}

impl fmt::Display for Torrent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut num_files = 1;
        let mut files_str = String::new();

        //get number of files and file names
        if let Some(ref files) = self.info.files {
            num_files = files.len();

            if num_files > 0 {

                let p = files[0].path.to_str().unwrap_or_default();
                let length = to_mem_size(files[0].length);
                files_str.push_str(format!(
                    "[{}] {}\n", length, p
                ).as_ref());

                for file in &files[1..] {
                    let length = to_mem_size(file.length);
                    files_str.push_str(format!(
                        "                    [{}] {}\n", length, file.path.to_str().unwrap_or_default()
                    ).as_ref());
                }
            }
        } else {
            files_str.push_str(self.info.name.as_ref());
        }

        //write out to screen
        writeln!(f, "name              : {}", self.info.name)?;
        writeln!(f, "comment           : {}", self.comment.as_deref().unwrap_or_default())?;
        writeln!(f, "creation-date     : {}", self.creation_date.unwrap_or(NaiveDateTime::from_timestamp(0,0)))?;
        writeln!(f, "announce          : {}", self.announce.as_deref().unwrap_or_default())?;
        writeln!(f, "announce list     : {}", display_announce_list(&self))?;
        writeln!(f, "files {:<5}       : {}", format!("({})", num_files), files_str)?;

        Ok(())
    }
}
