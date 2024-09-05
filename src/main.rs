fn main() {

}





/* Consider moving below section to its own file later */

// #[derive(Debug)]
enum TokenType {
    Word,
    Letter,
    SpecialCharacter,
}

// #[derive(Debug)]
struct Token<'a> {
    tipe: TokenType,
    text: &'a str,
}


fn tokenize<'a>(input: &'a str) -> Vec<Token<'a>> {
    todo!()
}