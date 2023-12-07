use fif::lexer::{Lexer, Token};

fn main() {
    let mut l = Lexer::new(r#"1 2 "hello" 10 69 420.69"#);

    loop {
        let token = l.next_token();

        println!("{token:?}");
        if token == Token::Eof {
            break;
        }
    }
}
