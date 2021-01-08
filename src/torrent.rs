use std::path::PathBuf;
use std::{fs};

use serde_bencode::{ de };

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
    path: PathBuf,

    length: i64,

    md5sum: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Info {
    name: String,

    #[serde(with="serde_bytes")]
    pieces: Vec<u8>,

    #[serde(rename="piece_length")]
    piece_length: i64,

    md5sum: Option<String>,

    length: Option<i64>,

    files: Option<Vec<File>>,

    path: Option<PathBuf>
}

#[derive(Debug, Deserialize)]
pub struct Torrent {
    pub info: Info,

    pub announces: Option<Vec<String>>,

    pub creation_date: Option<i64>,

    pub comment: Option<String>,

    pub created_by: Option<String>,
}

impl Torrent {
    pub fn from_file<T: AsRef<str>>(f: T) -> Result<Torrent> {
        let data = fs::read_to_string(f.as_ref())?;

        Ok(de::from_str::<Torrent>(&data)?)
    }
}
