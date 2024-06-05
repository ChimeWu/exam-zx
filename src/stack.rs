#[derive(Debug)]
pub struct Stack<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        } else {
            self.top -= 1;
        }
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        self.data.get(self.top - 1)
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn size(&self) -> usize {
        self.top
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Stack::new()
    }
}

impl<T> Iterator for Stack<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.top == 0 {
            return None;
        }
        self.top -= 1;
        self.data.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);

        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert!(!stack.is_empty());
        assert_eq!(stack.size(), 3);

        assert_eq!(stack.peek(), Some(&3));
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.pop(), Some(1));
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }
}
