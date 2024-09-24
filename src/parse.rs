use serde_json::{Value, Map};


pub fn parse<'a>(text: &'a str, parse_map: Map<String, Value>, conversion_map: Map<String, Value>) -> String {
    // parse to token vec
    let token_vec = tokenize(text);
    
    let mut output = String::with_capacity(token_vec.len()); // minimum capacity, can this be better?

    for token in token_vec.iter() {
        match token {
            Token::Space => output.push(' '),

            Token::Apostrophe => output.push('\''),

            Token::SpecialCharacter(c) => output.push(*c),

            Token::Word(v) => {
                let word_str = &token.to_string();

                if let Some(val) = parse_map.get(word_str) {
                    output.push_str(&convert_word(val[0].as_str().unwrap(), &conversion_map, v))
                }
                else {
                    output.push_str(&format!("[{}]", word_str));
                }
            },
        }

    }

    output
}




use std::fmt;
impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Space => write!(fmt, " "),
            Token::Apostrophe => write!(fmt, "'"),
            Token::SpecialCharacter(c) => write!(fmt, "{c}"),
            Token::Word(vec) => {
                let byte_arr: Vec<u8> = vec.iter().map(|c| c.character).collect();
                write!(fmt, "{}", String::from_utf8_lossy(byte_arr.as_slice()))
            },
        }
    }
}

#[derive(Clone, Debug)]
struct AsciiCharacter {
    character: u8,
    is_capitalized: bool,
}

#[derive(Debug)]
enum Token {
    Space,
    Apostrophe,
    SpecialCharacter(char),
    Word(Vec<AsciiCharacter>),
}




fn tokenize<'a>(input: &'a str) -> Vec<Token> {
    // final vec
    let mut token_vec: Vec<Token> = Vec::new();
    // temporary vec
    let mut word_vec = Vec::<AsciiCharacter>::new();


    for word in input.split(' ') {
        for (i, c) in word.chars().enumerate() {
            // if `c` is an apostrophe, it is either a quote or part of a contraction
            if c == '\'' {
                // it is a beginning quote or an end quote
                if word_vec.len() == 0 || word.len() == i + 1 {
                    token_vec.push(Token::Apostrophe);
                    continue;
                }

                word_vec.push(
                    AsciiCharacter {
                        character: b'\'',
                        is_capitalized: false,
                    }
                );
                continue;
            }

            // if `c` is a letter, it's part of a word
            if c.is_ascii_alphabetic() {
                let capped = c == c.to_ascii_uppercase();

                word_vec.push(
                    AsciiCharacter {
                        // is this optimal?
                        character: if capped { c.to_ascii_lowercase() as u8 } else { c as u8 },
                        is_capitalized: capped,
                    }
                );
                continue;
            }

            // at a non-alphabetic, non-apostrophe symbol, the word is over, so:
            if word_vec.len() > 0 {
                // push to token vec
                token_vec.push(Token::Word(word_vec.clone()));
                // reset word vec
                word_vec = Vec::new();
            }
            // and then push the non-alphabetic non-apostrophe character
            token_vec.push(Token::SpecialCharacter(c));
        }

        // if the last character was a letter and not a symbol
        if word_vec.len() > 0 { 
            token_vec.push(Token::Word(word_vec.clone()));
            word_vec = Vec::new();
        }
        token_vec.push(Token::Space);
    }

    
    // remove trailing space
    token_vec.pop();

    token_vec
}



fn convert_word<'a>(word_str: &'a str, conversion_map: &Map<String, Value>, char_vec: &Vec<AsciiCharacter>) -> String {
    let mut string_builder = String::with_capacity(word_str.len() /* >> 1 ? */);

    // if the original characters and new phonemes don't match up, we have to turn capitalization off (workshop this solution)
    let mut do_capitals = true;
    let num_phonemes: usize = word_str.bytes().filter(|b| *b == b' ').count();

    // check if there are more phonemes than there were characters
    if num_phonemes != char_vec.len() { do_capitals = false; }


    
    for (i, phoneme) in word_str.split(' ').enumerate() {
        // it is ok to slice the str because the map only has ascii
        let shortened: &str = if phoneme.len() == 3 { &phoneme[..2] } else { phoneme };
        let new_word: &str = if let Some(val) = conversion_map.get(shortened) { val.as_str().unwrap() }
                             else                                                         { shortened };

        let mut chars: std::str::Chars<'_> = new_word.chars();
        let mut first_char: char = chars.next().expect("Unexpected 0-length phoneme in dataset");


        // capitalize
        if let Some(ac) = char_vec.get(i) {
            if (do_capitals || i == 0) && ac.is_capitalized
            { first_char.make_ascii_uppercase(); }
        }


        // add to stringbuilder
        string_builder.push(first_char);

        if let Some(c) = chars.next()
        { string_builder.push(c); }
    }


    string_builder
}