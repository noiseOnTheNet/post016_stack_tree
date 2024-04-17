
pub struct BStack {
    stack: usize
}

#[derive(Clone, Copy,Debug, PartialEq)]
pub enum BStackError{
    EmptyStack,
    FullStack
}
use BStackError::*;

impl BStack {
    pub fn new() -> BStack {
        BStack { stack: 1 }
    }

    pub fn push(& mut self, value: bool) -> Result<bool, BStackError>{
        if self.size() == usize::BITS -1 {
            return Err(FullStack)
        }
        self.stack <<= 1;
        if value{
            self.stack += 1;
        }
        Ok(value)
    }

    pub fn top(& self) -> Result<bool, BStackError> {
        if self.stack == 1 {
            return Err(EmptyStack)
        }
        Ok((self.stack & 1) == 1)
    }

    pub fn pop(& mut self) -> Result<bool, BStackError> {
        if self.stack == 1 {
            return Err(EmptyStack)
        }
        let result = (self.stack & 1) == 1;
        self.stack >>= 1;
        Ok(result)
    }

    pub fn size(& self) -> u32 {
        usize::BITS - usize::leading_zeros(self.stack) - 1
    }

    pub fn get_state(& self) -> usize {
        self.stack ^ (1 << self.size())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_pop() {
        let mut stack = BStack::new();
        stack.push(false);
        stack.push(true);
        let result1 = stack.pop();
        let result2 = stack.pop();
        assert_eq!(result1, Ok(true));
        assert_eq!(result2, Ok(false));
    }

    #[test]
    fn size_increase_when_push() {
        let mut stack = BStack::new();
        stack.push(false);
        let result1 = stack.size();
        stack.push(true);
        let result2 = stack.size();
        assert_eq!(result2, result1 + 1);
    }

    #[test]
    fn push_and_top() {
        let mut stack = BStack::new();
        stack.push(false);
        let result1 = stack.top();
        stack.push(true);
        let result2 = stack.top();
        assert_eq!(result1, Ok(false));
        assert_eq!(result2, Ok(true));
    }

    #[test]
    fn size_decreases_when_pop() {
        let mut stack = BStack::new();
        stack.push(false);
        stack.push(true);
        let result1 = stack.size();
        stack.pop();
        let result2 = stack.size();
        assert_eq!(result1, result2 + 1);
    }

    #[test]
    fn empty_when_created() {
        let result = BStack::new().size();
        assert_eq!(result, 0);
    }

    #[test]
    fn empty_does_not_pop() {
        let mut stack = BStack::new();
        let result = stack.pop();
        assert_eq!(result, Err(EmptyStack));
    }

}
