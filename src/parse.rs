use serde_json::{Value, Map};


pub fn parse<'a>(text: &'a str, parse_map: Map<String, Value>, conversion_map: Map<String, Value>) -> String {
    // parse to token vec
    let token_vec = tokenize(text);
    
    let mut output = String::with_capacity(token_vec.len());

    for token in token_vec.iter() {
    // TODO: remove Space enum variant
        match token {
            Token::Space => output.push(' '),
            Token::SpecialCharacter(c) => output.push(*c),
            Token::Word(s) => {
                if let Some(val) = parse_map.get(*s) {
                    output.push_str(&translate_word(val.get(0).unwrap().as_str().unwrap(), &conversion_map))
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

fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    let separated: std::str::Split<'a, char> = input.split(' ');
    let mut output: Vec<Token<'a>> = Vec::new();

    for word in separated {
        let mut start_index: usize = 0;

        for (i, chair) in word.chars().enumerate() {
        // TODO: could this all be a match statement?
            // if `chair` is a letter or apostrophe, it's part of a word
            if chair.is_ascii_alphabetic() || chair == '\'' { continue; }

            // otherwise, the word is over, so push it into the token vector
            if start_index != i {
                output.push(Token::Word(&word[start_index..i]));
            }
            // and then push the non-letter non-apostrophe character
            output.push(Token::SpecialCharacter(chair));

            // and reset `start_index`
            start_index = i + 1;
        }


        if start_index != word.len() { 
            output.push(Token::Word(&word[start_index..])); 
        }
        output.push(Token::Space);
    }

    // remove trailing space
    output.pop();

    output
}


fn translate_word<'a>(word_str: &'a str, conversion_map: &Map<String, Value>) -> String {
    let mut string_builder = String::with_capacity(word_str.len() /* >> 1 ? */);

    for phoneme in word_str.split(' ') {
        // if phoneme length is 3 it has a number on the end we don't use (it is ok to slice the str because the map only has ascii)
        let shortened = if phoneme.len() == 3 { &phoneme[..2] } else { phoneme };

        if let Some(val) = conversion_map.get(shortened) {
            string_builder.push_str(val.as_str().unwrap())
        }
        else { string_builder.push_str(shortened) }
    }

    string_builder
}



/*
fn read_json<'a>(filepath: &'a str) -> Map<String, Value> {
    // TODO: since we know Value will always be the Array(Vec<Value>) variant,
    //       change the Map<String, Value> to Map<String, Vec<String>> somehow
    let cmu_db = fs::read_to_string(filepath).expect("Could not read file");
    
    serde_json::from_str(&cmu_db).expect("Failed to read json")
}
    */