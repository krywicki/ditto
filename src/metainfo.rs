use std::io::{self, Stderr};
use std::str;

use reqwest::Url;
use serde::{self, de, Deserialize, Deserializer, Serialize};
use serde_bencode as ben;
use serde_bencode::value::Value as BenValue;

type Sha1Hash = [u8; 20];

#[derive(Deserialize)]
pub struct MetaInfo {
    pub name: String,
    pub info_hash: Sha1Hash,
    pub pieces: Vec<u8>,
    pub pieces_len: u32,
    pub files: Vec<FileInfo>,

    #[serde(deserialize_with = "parse_trackers")]
    pub trackers: Vec<Url>,
}

#[derive(Serialize, Deserialize)]
pub struct FileInfo {}

fn parse_url(url: &Vec<u8>) -> Result<Url, ben::Error> {
    let url = str::from_utf8(url)
        .map_err(|_| ben::Error::InvalidValue("Tracker Url is invalid utf-8".into()))?;
    let url =
        Url::parse(url).map_err(|e| ben::Error::InvalidValue("Invalid tracker url".into()))?;

    Ok(url)
}

fn parse_url_list(values: &Vec<BenValue>) -> Result<Vec<Url>, ben::Error> {
    let mut items: Vec<Url> = vec![];
    for value in values {
        match value {
            BenValue::List(ref v) => {
                let urls = parse_url_list(v)?;
                items.extend_from_slice(&urls);
            }
            BenValue::Bytes(ref url) => items.push(parse_url(&url)?),
            _ => return Err(ben::Error::InvalidType("Expected List ".into())),
        }
    }

    return Ok(items);
}

fn parse_trackers<'de, D>(deserializer: D) -> Result<Vec<Url>, D::Error>
where
    D: Deserializer<'de>,
{
    let url_list: Vec<BenValue> = Deserialize::deserialize(deserializer)?;
    Ok(parse_url_list(&url_list).map_err(de::Error::custom)?)
}
