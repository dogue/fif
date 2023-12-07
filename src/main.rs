use fif::FifVM;

fn main() {
    let mut vm = FifVM::new();
    vm.run(r#"1.0 2 +"#);
    println!("{vm:?}");
    // FifVM { stack: [Str("hello world"), Float(420.69)] }
}
