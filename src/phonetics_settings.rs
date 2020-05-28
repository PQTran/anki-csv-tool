use read_input::prelude::*;

#[derive(Debug)]
pub enum Phonetics {
    Pinyin,
    Zhuyin
}

pub fn select_from_phonetics_menu() -> Phonetics {
    println!("What is the phonetics system used? [1] Pinyin, [2] Zhuyin");

    let default_err_msg = "Your response was not recognized. Please try again!";
    let input: u8 = input()
        .msg(">> ")
        .inside(1..=2)
        .err(default_err_msg)
        .get();

    if input == 1 {
        Phonetics::Pinyin
    } else {
        Phonetics::Zhuyin
    }
}
