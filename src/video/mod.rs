use std::fs::{self, File};
use std::io::prelude::Seek;
use std::io::{BufReader, SeekFrom};
use std::path::PathBuf;

pub fn parse_mp4(path: &str) {
    let filepath = fs::canonicalize(&PathBuf::from(path)).unwrap();
    println!("parse mp4 file: {:?}", filepath);

    let f = File::open(filepath).unwrap();
    let mut reader = BufReader::new(f);

    let context = mp4parse::read_mp4(&mut reader).expect("read mp4 failed");

    // let udta = context
    //     .userdata
    //     .expect("didn't find udta")
    //     .expect("failed to parse udta");
    // let meta = udta.meta.expect("didn't find meta");
    // println!("Title: {:?}", meta.title.unwrap());
    // println!("Artist: {:?}", meta.artist.unwrap());

    for track in context.tracks {
        match track.track_type {
            mp4parse::TrackType::Video => {
                println!("track.duration: {:?}", track.duration.unwrap());
                println!("track.media_time: {:?}", track.media_time);
                println!("track.empty_duration: {:?}", track.empty_duration);
                println!("track.timescale: {:?}", track.timescale);

                let tkhd = track.tkhd.unwrap();
                println!("tkhd.disabled: {}", tkhd.disabled);
                println!("track duration: {}", tkhd.duration);

                // track.stsd part
                let stsd = track.stsd.expect("expected an stsd");
                let v = match stsd.descriptions.first().expect("expected a SampleEntry") {
                    mp4parse::SampleEntry::Video(v) => v,
                    _ => panic!("expected a VideoSampleEntry"),
                };
                println!("v = {} x {}", v.width, v.height);

                match v.codec_specific {
                    mp4parse::VideoCodecSpecific::AVCConfig(ref _avc) => {
                        println!("AVC");
                    }
                    mp4parse::VideoCodecSpecific::ESDSConfig(ref _mp4v) => {
                        println!("MP4V");
                    }
                    mp4parse::VideoCodecSpecific::H263Config(ref _h263) => {
                        println!("H263");
                    }
                    _ => {}
                }
            }

            _ => {}
        }
    }
}

pub fn read_file(filename: &str) {
    // TODO: what if file is too big?
    // let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    // println!("file '{}' {} bytes\n", filename, content.len());

    let mut f = fs::File::open(filename).expect("open file failed");
    let size = f.seek(SeekFrom::End(0)).unwrap();
    println!("file '{}' {} bytes\n", filename, size);
}

// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.contains(query) {
//             results.push(line);
//         }
//     }

//     results
// }
