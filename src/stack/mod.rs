

pub struct Stack<T> {
    vec: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            vec: Vec::new()
        }
    }
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn size(&self) -> u16 {
        self.vec.len() as u16
    }
    pub fn is_full(&self) -> bool {
        self.size() == 65534
    }

    pub fn pop(&mut self) -> T {
        if self.is_empty() {
            panic!("pop from empty stack (im fool)")
        }
        self.vec.pop().unwrap()
    }

    pub fn push(&mut self, val: T) {
        if self.is_full() {
            panic!("pushing to full stack")
        }
        self.vec.push(val);
    }
}