use fif::FifVM;

fn main() {
    let mut vm = FifVM::new();
    vm.run(
        r#"
        1 2               // numeric literals get pushed on to the stack
        +                 // simple addition, now 3 is at the top of the stack
        dupe              // duplicate the 3 on the top of the stack
        *                 // multiply the 3's together, now 9 is on top of the stack
        5.0 /             // if you mix floats and ints, the result will be a float
        "hello"           // strings get pushed as well
        " world" swap +   // you can concatenate strings with addition
        print
        debug
        "#,
    );
    // println!("{vm:?}");
    // FifVM { stack: [Float(0.5555556), Str("hello world")] }
}
