use rand::Rng;
use rand::*;

pub fn replace_obfuscation_word(words:Vec<String>)->Vec<String>{
    /*let randwordz = vec!["sasa","lala","mimi","baba","kaka","daka","yoka"];
    let mut words_clone = words.clone();
    let mut rng = rand::thread_rng();
    let mut random_index = rng.gen_range(0..randwordz.len());
    for mut word in words_clone{
        word = gec[random_index];
    }
    return words_clone;*/
    todo!()
}

/// Return a word with random characters from abc
///  and random length between 1 and word length
pub fn fixed_obfuscation(word:&str)->String{
    let mut word_clone:&str = word.clone();
    let mut ret_word = String::new();
    let mut rng = rand::thread_rng();
    let mut word_len = word_clone.len();
    let mut random_number: usize = rng.gen_range(1..word.len());
    for i in 0..random_number{
        ret_word.push(random_abc_char());
    }
    return ret_word;
}

pub fn random_abc_char()->char{
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(97..123); 
    return char::from(random_number);
}