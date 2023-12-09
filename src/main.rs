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
        print             // consume and print the top value from the stack
        var x 5           // create a variable called x and set it's value to 5
        x                 // push the value of the variable x on to the stack (does not drop the variable)
        drop              // pop and discard the top of the stack
        drop              // discard a previously defined variable
        "#,
    );
    println!("{vm:#?}");
    // FifVM { stack: [Float(0.5555556), Str("hello world")] }
}
