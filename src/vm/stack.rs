use crate::STACK_SIZE;

impl super::VM {
    #[allow(clippy::missing_panics_doc)]
    #[allow(clippy::cast_sign_loss)]

    pub fn pop(&mut self) -> u16 {
        assert!(self.stack_pointer > -1, "Stack empty, attempted top POP");

        self.stack_pointer -= 1;
        self.stack[(self.stack_pointer + 1) as usize]
    }

    #[allow(clippy::missing_panics_doc)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_wrap)]
    pub fn push(&mut self, value: u16) {
        assert!(
            self.stack_pointer < STACK_SIZE as i8,
            "Stack full, attempted to PUSH"
        );

        self.stack_pointer += 1;
        self.stack[self.stack_pointer as usize] = value;
    }
}
#[cfg(test)]
mod test {
    use crate::VM;

    #[test]
    fn test_max_stack() {
        let mut vm = VM::new();

        for x in 0..16 {
            vm.push(x);
        }

        for x in (0..16).rev() {
            assert_eq!(x, vm.pop());
        }
    }

    #[test]
    #[should_panic]
    fn test_stack_overflow() {
        let mut vm = VM::new();

        for x in 0..17 {
            vm.push(x);
        }
    }
}
