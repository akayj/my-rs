extern crate lru;

use lru::LruCache;

pub fn cache() {
    let mut cache = LruCache::new(2);
    cache.put("apple", 3);
    cache.put("banana", 2);

    println!("cache is : {:?}", cache);

    {
        let v = cache.get_mut(&"banana").unwrap();
        *v = 6;
    }
}

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

// `&str` is an immutable reference,
// allowing us accept `&String` and `&str` as parameter.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // Following lines equals to the uncomment lines.
    // for (i, item) in bytes.iter().enumerate() {
    //     if *item == b' ' {
    //         return &s[..i];
    //     }
    // }

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

pub fn which_word() {
    let s = String::from("hello world");

    let word = first_word(&s);

    println!("word is {}", word);

    let str_literal = "hello world again";
    println!("str_literal = {}", str_literal);

    // Because string literals *are* string slice already,
    // this works too, without the slice syntax!
    let word2 = first_word(str_literal);
    println!("word2 = {}", word2);
}
