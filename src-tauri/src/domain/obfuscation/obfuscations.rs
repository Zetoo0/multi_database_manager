use rand::Rng;
use serde_json::*;
use std::path::Path;
use tokio::fs::*;

/// Return a word with random characters from abc
///  and random length between 1 and word length
pub fn fixed_obfuscation(word: &str) -> String {
    let word_clone: &str = word;
    let mut ret_word = String::new();
    let mut rng = rand::thread_rng();
    let word_len = word_clone.len();
    let random_number: usize = rng.gen_range(1..word.len());
    for i in 0..random_number {
        ret_word.push(random_abc_char());
    }
    return ret_word;
}

pub fn random_abc_char() -> char {
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(97..123);
    return char::from(random_number);
}

//Input a word and transform it into character whats read from the json
pub async fn name_obfuscate(word: &str) -> Result<String> {
    let mut ret_word = String::new();
    let mut file_path = Path::new(file!()).to_path_buf();
    file_path = file_path.parent().unwrap().join("obfuscate-names.json");
    let json_read = read_to_string(file_path.clone()).await;
    log::info!("json_read: {:?}\n", json_read.as_ref().unwrap());
    let obfuscation: Value = serde_json::from_str(&json_read.unwrap()).unwrap();
    log::info!("obfuscation as value: {:?}\n", obfuscation);
    let obfuscation_vec = obfuscation.get("names").unwrap().as_array().unwrap();
    log::info!("obfuscation_vec: {:?}\n", obfuscation_vec);
    let index = rand::thread_rng().gen_range(0..obfuscation_vec.len());
    log::info!("index: {:?}\n", index);
    let name = obfuscation_vec.get(index).unwrap().get("name").unwrap().as_str().unwrap();
    ret_word = name.to_string();
    log::info!("return name: {:?}\n", name);
    //let obfuscation: Value = serde_json::from_str(&json_read.unwrap()).unwrap();
    //let obfuscation_vec = obfuscation.as_array().unwrap();
   // log::info!("obfuscation_vec: {:?}\n", obfuscation_vec);
    /*if json_read.is_ok() {
        let obfuscation: Value = serde_json::from_str(&json_read.unwrap()).unwrap();
        let obfuscation_vec = obfuscation.as_array().unwrap();
        log::info!("obfuscation_vec: {:?}\n", obfuscation_vec);
        ret_word = obfuscation_vec
            .get(0)
            .expect("expected index")
            .as_str()
            .unwrap()
            .to_string();
        Ok(ret_word)
    } else {
        Ok(word.to_string())
    }*/
    Ok(ret_word)
}
