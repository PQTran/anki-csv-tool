use read_input::prelude::*;
use crate::models::{Record, SelectionOption};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Phonetics {
    Pinyin,
    // EncodedPinyin,
    Zhuyin
}

impl Phonetics {
    fn from(s: &str) -> Option<Self> {
        match s {
            "Pinyin" => Some(Phonetics::Pinyin),
            "Zhuyin" => Some(Phonetics::Zhuyin),
            _ => None
        }
    }
}

impl std::fmt::Display for Phonetics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// check all syllables to be zhuyin or pinyin, if not then prop error
pub fn infer_phonetics(record: &Record) -> Result<Phonetics, Box<dyn std::error::Error>> {
    let pronunciation = record.get_data().get_pronunciation();

    if zhuyin::split(pronunciation).iter().all(|word| zhuyin::is_valid_word(&word)) {
        Ok(Phonetics::Zhuyin)
    } else {
        Ok(Phonetics::Pinyin)
    }

    // default is error because could not be zhuyin or pinyin
}

fn _print_output_phonetics_setting_prompt(options: &Vec<SelectionOption>) {
    let msg = SelectionOption::format_inline(&options[..]);
    println!("What phonetics system do you want to output? {}", msg);
}

pub fn select_from_output_phonetics_menu() -> Phonetics {
    let options = vec![
        SelectionOption::new("1".to_string(), Phonetics::Pinyin.to_string()),
        SelectionOption::new("2".to_string(), Phonetics::Zhuyin.to_string())
    ];
    _print_output_phonetics_setting_prompt(&options);

    let keys: Vec<String> = options.iter().map(|x| x.get_key().to_string()).collect();
    let default_err_msg = "Your response was not recognized. Please try again!";
    let input: String = input()
        .msg(">> ")
        .inside(keys)
        .err(default_err_msg)
        .get();

    let selected_option = options.iter().find(|&x| x.get_key() == input).unwrap();
    Phonetics::from(selected_option.get_value()).unwrap()
}
