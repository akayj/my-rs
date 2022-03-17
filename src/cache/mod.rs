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
    let home_dir = dirs::home_dir().unwrap();
    println!("home_dir: {:?}", home_dir);

    let audio_dir = dirs::audio_dir().unwrap();
    println!("audio_dir: {:?}", audio_dir);

    let config_dir = dirs::config_dir().unwrap();
    println!("config_dir: {:?}", config_dir);

    if let Some(executable_dir) = dirs::executable_dir() {
        println!("executable_dir: {:?}", executable_dir);
    }
}
