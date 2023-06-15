impl super::VM {
    #[allow(clippy::missing_panics_doc)]
    pub fn pop(&mut self) -> u16 {
        assert!(self.stack_pointer > 0, "Stack empty, attempted top POP");

        self.stack_pointer -= 1;
        self.stack[(self.stack_pointer + 1) as usize]
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn push(&mut self, value: u16) {
        assert!(self.stack_pointer < 31, "Stack full, attempted to PUSH");

        self.stack_pointer += 1;
        self.stack[self.stack_pointer as usize] = value;
    }
}
