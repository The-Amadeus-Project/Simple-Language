mod lexer;

fn main() {
    let file = "main.sl";
    let content= std::fs::read_to_string(file).expect("couldnt open file");
    let mut the_lexer = lexer::Lexer::new();
    let res = the_lexer.lex_text(content);

}
