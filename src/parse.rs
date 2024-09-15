use serde_json::{Value, Map};


pub fn parse<'a>(text: &'a str, parse_map: Map<String, Value>, conversion_map: Map<String, Value>) -> String {
    // parse to token vec
    let (token_vec, capital_vec) = tokenize(text);
    
    let mut space_count = 0_usize;
    let mut output = String::with_capacity(token_vec.len());

    for token in token_vec.iter() {
        match token {
            Token::Space => {
                space_count += 1;
                output.push(' ')
            },
            Token::SpecialCharacter(c) => output.push(*c),
            Token::Word(s) => {
                if let Some(val) = parse_map.get(&s.to_lowercase()) {
                    output.push_str(&translate_word(val[0].as_str().unwrap(), &conversion_map, &capital_vec[space_count]))
                }
                else {
                    output.push_str(&format!("[{s}]"));
                }
            }
        }
    }

    output
}




#[derive(Debug)]
pub enum Token<'a> {
    Space,
    SpecialCharacter(char),
    Word(&'a str),
}

fn tokenize<'a>(input: &'a str) -> (Vec<Token<'a>>, Vec<Vec<usize>>) {
    let separated: std::str::Split<'a, char> = input.split(' ');
    let separated_size = separated.clone().count();
    let mut token_vec: Vec<Token<'a>> = Vec::new();

    // `capital_vec` is the same length as `separated`
    // meaning even if two words are separated by a '-' or another non-space character
    // `capital_vec` will consider them the same word
    let mut capital_vec: Vec<Vec<usize>> = Vec::with_capacity(separated_size);
            capital_vec.resize(separated_size, Vec::new());

    for (i, word) in separated.enumerate() {
        let mut start_index: usize = 0;

        for (j, c) in word.chars().enumerate() {
        // TODO: could this all be a match statement?
            // if `c` is a letter or apostrophe, it's part of a word
            if c.is_ascii_alphabetic() || c == '\'' {
                if c == c.to_ascii_uppercase() { capital_vec[i].push(j); }
                continue;
            }

            // otherwise, the word is over, so push it into the token vector
            if start_index != j {
                token_vec.push(Token::Word(&word[start_index..j]));
            }
            // and then push the non-letter non-apostrophe character
            token_vec.push(Token::SpecialCharacter(c));

            // and reset `start_index`
            start_index = j + 1;
        }


        if start_index != word.len() { 
            token_vec.push(Token::Word(&word[start_index..])); 
        }
        token_vec.push(Token::Space);
    }

    // remove trailing space
    token_vec.pop();

    (token_vec, capital_vec)
}


fn translate_word<'a>(word_str: &'a str, conversion_map: &Map<String, Value>, capitals: &Vec<usize>) -> String {
    let mut iterator: std::slice::Iter<'_, usize> = capitals.iter();

    // if there are no capitals left in the word then stop trying iterator.next()
    let mut are_capitals_remaining = true;

    // manual iteration because it felt like a rust thing to do
    let mut current_capital_index = match iterator.next() {
        Some(i) => *i,
        None => { are_capitals_remaining = false; 0 }
    };

    let mut string_builder = String::with_capacity(word_str.len() /* >> 1 ? */);

    for (i, phoneme) in word_str.split(' ').enumerate() {
        // if phoneme length is 3 it has a number on the end we don't use (it is ok to slice the str because the map only has ascii)
        let shortened: &str = if phoneme.len() == 3 { &phoneme[..2] } else { phoneme };

        let capped = if are_capitals_remaining && current_capital_index == i {
            current_capital_index = match iterator.next() {
                Some(i) => *i,
                None => { are_capitals_remaining = false; 0 }
            };
            &shortened.to_ascii_uppercase()
        }
        else { shortened };


        if let Some(val) = conversion_map.get(capped)
        {
            string_builder.push_str(val.as_str().unwrap());
        }
        else 
        {
            string_builder.push_str(capped);
        }
    }

    string_builder
}