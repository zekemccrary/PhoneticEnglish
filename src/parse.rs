use serde_json::{Value, Map};


pub fn parse<'a>(text: &'a str, parse_map: Map<String, Value>, conversion_map: Map<String, Value>) -> String {
    // parse to token vec
    let token_vec = tokenize(text);
    
    let mut output = String::with_capacity(token_vec.len()); // minimum capacity, can this be better?

    for token in token_vec.iter() {
        match token {
            Token::Space => output.push(' '),

            Token::SpecialCharacter(c) => output.push(*c),

            Token::Word(v) => {
                let word_str = &token.to_string();

                if let Some(val) = parse_map.get(word_str) {
                    output.push_str(&translate_word(val[0].as_str().unwrap(), &conversion_map, v))
                }
                else {
                    output.push_str(&format!("[{}]", word_str));
                }
            }
        }
    }

    output
}




use std::fmt;
impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Space => write!(fmt, " "),
            Token::SpecialCharacter(c) => write!(fmt, "{c}"),
            Token::Word(vec) => {
                let byte_arr: Vec<u8> = vec.iter().map(|c| c.character).collect();
                write!(fmt, "{}", String::from_utf8_lossy(byte_arr.as_slice()))
            },
        }
    }
}


#[derive(Clone, Debug)]
struct Character{
    character: u8,
    is_capitalized: bool,
}
#[derive(Debug)]
enum Token {
    Space,
    SpecialCharacter(char),
    Word(Vec<Character>),
}

fn tokenize<'a>(input: &'a str) -> Vec<Token> {
    let mut token_vec: Vec<Token> = Vec::new();
    let mut word_vec = Vec::<Character>::new();


    for word in input.split(' ') {
        for c in word.chars() {
        // TODO: could this all be a match statement?
            // if `c` is a letter or apostrophe, it's part of a word
            if c.is_ascii_alphabetic() || c == '\'' {
                let capped = c == c.to_ascii_uppercase();
                word_vec.push(
                    Character{
                        character: if capped { c.to_ascii_lowercase() as u8 } else { c as u8 },
                        is_capitalized: capped,
                    }
                );
                continue;
            }

            // otherwise, the word is over, so push it into the token vector
            if word_vec.len() > 0 {
                token_vec.push(Token::Word(word_vec.clone()));
                word_vec = Vec::new();
            }
            // and then push the non-letter non-apostrophe character
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






fn translate_word<'a>(word_str: &'a str, conversion_map: &Map<String, Value>, char_vec: &Vec<Character>) -> String {
    let mut string_builder = String::with_capacity(word_str.len() /* >> 1 ? */);

    for (phoneme, c) in word_str.split(' ').zip(char_vec) {
        // if phoneme length is 3 it has a number on the end we don't use (it is ok to slice the str because the map only has ascii)
        let shortened: &str = if phoneme.len() == 3 { &phoneme[..2] } else { phoneme };

        let new_word = if let Some(val) = conversion_map.get(shortened) { val.as_str().unwrap() }
                             else                                                          { shortened };


        if c.is_capitalized {
            string_builder.push_str(&new_word.to_ascii_uppercase());
        }
        else {
            string_builder.push_str(new_word);
        }
    }

    string_builder
}