use fif::{
    lexer::{Lexer, Token},
    FifVM,
};

fn main() {
    // let mut vm = FifVM::new();
    // vm.run(
    //     r#"
    //         1 2 +    // add one and two
    //         dupe     // duplicate the 3 that is now on the stack
    //         *        // multiply the two 3's that are on the stack together
    //     "#,
    // );
    // println!("{vm:?}");
    // // FifVM { stack: [Str("hello world"), Float(420.69)] }

    let mut lexer = Lexer::new(
        r#"
            1 2 +    // add one and two
            dupe     // duplicate the 3
            *        // multiply the two 3's
        "#,
    );

    loop {
        let token = lexer.next_token();
        println!("{token:?}");
        if token == Token::Eof {
            break;
        }
    }
}
