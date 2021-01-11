extern crate ditto;

use ditto::torrent;

fn main() {
    match torrent::Torrent::from_file("soul.torrent") {
        Ok(ref t) => {
            println!("{}", t);
        },
        Err(ref e) => {
            match e.kind {
                torrent::ErrorKind::DecodeFailure => println!("Bencode Failure - {}", e.msg),
                torrent::ErrorKind::IOError => println!("IO Error - {}", e.msg)
            }
        }
    }
}
