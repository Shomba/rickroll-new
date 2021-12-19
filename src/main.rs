use std::{fs,fs::File,io::BufReader,thread,time};
use rodio::{source::Source, Decoder, OutputStream};
use rust_embed::RustEmbed;
use flate2::read::GzDecoder;
use tar::Archive;
#[derive(RustEmbed)]
#[folder = "perm/"]
struct Asset;
fn main() {
    let video = Asset::get("out.tar.gz").unwrap();
    let audio = Asset::get("audio.mp3").unwrap();
    let tempdir = std::env::temp_dir();
    fs::write(&tempdir.join("i.tar.gz"), video.data);
    fs::write(&tempdir.join("a.mp3"), audio.data);
    let mut f  = File::open(tempdir.join("i.tar.gz")).unwrap();
    let mut data = GzDecoder::new(f);
    let mut archie = Archive::new(data);
    archie.unpack(tempdir);
    let tmp = std::env::temp_dir();
    let mut frames: Vec<_> = std::fs::read_dir(&tmp.join("out/")).unwrap().map(|r|r.unwrap()).collect();
    frames.sort_by_key(|dir|dir.path());
    let (strean, strean_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(tmp.join("a.mp3")).unwrap());
    let src = Decoder::new(file).unwrap();
    strean_handle.play_raw(src.convert_samples());
    for elem in frames{
        print!("\x1B[2J");
        let pic = std::fs::read_to_string(elem.path()).expect("Unable to read file");
        println!("{}", pic);
        thread::sleep(time::Duration::from_millis(100))
    }


}
