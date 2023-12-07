use fif::FifVM;

fn main() {
    let mut vm = FifVM::new();
    vm.run(r#""world" "hello " + 420.0 0.69 +"#);
    println!("{vm:?}");
    // FifVM { stack: [Str("hello world"), Float(420.69)] }
}
