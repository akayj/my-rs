use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
