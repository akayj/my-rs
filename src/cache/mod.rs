use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn list_dirs() {
    let home_dir = dirs::home_dir().unwrap_or_default();
    println!("home_dir: {:?}", home_dir);

    let audio_dir = dirs::audio_dir().unwrap_or_default();
    println!("audio_dir: {:?}", audio_dir);

    let config_dir = dirs::config_dir().unwrap_or_default();
    println!("config_dir: {:?}", config_dir);

    if let Some(executable_dir) = dirs::executable_dir() {
        println!("executable_dir: {:?}", executable_dir);
    }

    if let Some(font_dir) = dirs::font_dir() {
        println!("font_dir: {:?}", font_dir);
    }

    if let Some(dl_dir) = dirs::download_dir() {
        println!("download_dir: {:?}", dl_dir);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// pub fn lines(filename: &str) -> Vec<&str> {
//     let mut lines_vec = vec![];

//     if let Ok(lines) = read_lines(filename) {
//         for line in lines {
//             if let Ok(li) = line {
//                 lines_vec.push(li.clone().as_str());
//             }
//         }
//     }

//     lines_vec
// }
