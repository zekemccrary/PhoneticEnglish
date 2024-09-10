#[derive(Debug)]
pub enum Token<'a> {
    Space,
    SpecialCharacter(char),
    Word(&'a str),
}


pub fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    let separated: std::str::Split<'a, char> = input.split(' ');
    let mut output: Vec<Token<'a>> = Vec::new();

    for word in separated {
        let mut start_index: usize = 0;

        for (i, care) in word.chars().enumerate() {
        // TODO: could this all be a match statement?
            // if `care` is a letter or apostrophe, it's part of a word
            if care.is_ascii_alphabetic() || care == '\'' { continue; }

            // otherwise, the word is over, so push it into the token vector
            if start_index != i {
                output.push(Token::Word(&word[start_index..i]));
            }
            // and then push the non-letter non-apostrophe character
            output.push(Token::SpecialCharacter(care));

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