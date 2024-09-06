fn main() {
    let stir = "Hi. T!h!e ostrick-catcher wasn't dead!!";

    let token_vec = tokenize(stir);

    println!("{:?}", token_vec);
}





/* Consider moving below section to its own file later */

#[derive(Debug)]
enum Token<'a> {
    Space,
    SpecialCharacter(char),
    Word(&'a str),
}


fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    let separated: std::str::Split<'a, char> = input.split(' ');
    let mut output: Vec<Token<'a>> = Vec::new();

    // for this preliminary version we will assume that all punctuation will be at the end of the words
    for word in separated {
        let mut start_index: usize = 0;

        for (i, care) in word.chars().enumerate() {
            // TODO: could this be a match statement?
            if care == '-' {
                // check if start_index is behind the current index
                if start_index != i {
                    output.push(Token::Word(&word[start_index..i]));
                }
                // move start_index up past the '-'
                start_index = i + 1;
                output.push(Token::SpecialCharacter(care));
                continue;
            }

            if care.is_ascii_alphabetic() || care == '\'' { continue; }

            // if it is not a letter, dash, or apostrophe, then it is some other symbol that means the word is over
            if start_index != i
            { output.push(Token::Word(&word[start_index..i])); }
            output.push(Token::SpecialCharacter(care));

            start_index = i + 1;
        }

        // in case the word ends in a letter and not a punctuation mark or some symbol
        if start_index != word.len()
        { output.push(Token::Word(&word[start_index..])); }



        output.push(Token::Space);
    }

    output
}