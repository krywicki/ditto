extern crate ditto;

use ditto::Torrent;


fn main() {

    match Torrent::from_file("this is a torrent") {
        Ok(_) => {
            println!("opened torrent file");
        },
        Err(ref e) => {
            println!("Error - {}", e.msg);
        }
    }
}
