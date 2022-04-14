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

fn is_hello<T: AsRef<str>>(s: T) {
    assert_eq!("hello", s.as_ref());
}

pub fn ref_main() {
    let s = "hello";
    is_hello(s);

    let s = "hello".to_string();
    is_hello(s);
}
