use types::Type;

mod types;

#[derive(Debug)]
pub struct FifVM {
    pub stack: Vec<Type>,
}

impl FifVM {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn run(&mut self) {}

    fn pop(&mut self) -> Type {
        self.stack.pop().unwrap()
    }

    fn pop_two(&mut self) -> (Type, Type) {
        (self.pop(), self.pop())
    }

    fn push(&mut self, t: Type) {
        self.stack.push(t);
    }

    fn swap(&mut self) {
        let top = self.stack.len() - 1;
        let next = top - 1;
        self.stack.swap(top, next);
    }

    fn duplicate(&mut self) {
        if let Some(top) = self.stack.last().cloned() {
            self.stack.push(top);
        }
    }

    fn add(&mut self) {
        let (a, b) = self.pop_two();
        self.push(a + b);
    }

    fn sub(&mut self) {
        let (a, b) = self.pop_two();
        self.push(a - b);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_int() {
        let mut vm = FifVM::new();
        vm.push(Type::Int(1));
        vm.push(Type::Int(2));
        vm.add();

        assert_eq!(vm.stack[0], Type::Int(3));
    }

    #[test]
    fn test_add_float() {
        let mut vm = FifVM::new();
        vm.push(Type::Float(1.0));
        vm.push(Type::Float(2.0));
        vm.add();

        assert_eq!(vm.stack[0], Type::Float(3.0));
    }

    #[test]
    fn test_add_str() {
        let mut vm = FifVM::new();
        vm.push(Type::Str("world".to_string()));
        vm.push(Type::Str("hello ".to_string()));
        vm.add();

        assert_eq!(vm.stack[0], Type::Str("hello world".to_string()));
    }

    #[test]
    fn test_sub_int() {
        let mut vm = FifVM::new();
        vm.push(Type::Int(2));
        vm.push(Type::Int(3));
        vm.sub();

        assert_eq!(vm.stack[0], Type::Int(1));
    }

    #[test]
    fn test_sub_float() {
        let mut vm = FifVM::new();
        vm.push(Type::Float(2.0));
        vm.push(Type::Float(3.0));
        vm.sub();

        assert_eq!(vm.stack[0], Type::Float(1.0));
    }

    #[test]
    fn test_sub_str() {
        let mut vm = FifVM::new();
        vm.push(Type::Str("deez".to_string()));
        vm.push(Type::Str("nuts".to_string()));
        vm.sub();

        assert_eq!(vm.stack[0], Type::Invalid);
    }
}
