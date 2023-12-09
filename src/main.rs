use fif::FifVM;

fn main() {
    let mut vm = FifVM::new();
    vm.run(
        r#"
        1 2 swap dupe
        "#,
    );
    println!("{vm:?}");
    // FifVM { stack: [Str("hello world"), Float(420.69)] }

    // let mut lexer = Lexer::new(
    //     r#"
    //         1 2 +    // add one and two
    //         1.3
    //         dupe     // duplicate the 3
    //         *        // multiply the two 3's
    //     "#,
    // );

    // loop {
    //     let token = lexer.next_token().unwrap();
    //     println!("{token:?}");
    //     if token == Token::Eof {
    //         break;
    //     }
    // }
}
