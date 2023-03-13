use std::collections::VecDeque;

struct MyQueue {
    stack: VecDeque<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Self { stack: VecDeque::new() }
    }
    
    fn push(&mut self, x: i32) {
        self.stack.push_back(x);
    }
    
    fn pop(&mut self) -> i32 {
        self.stack.pop_front().unwrap()
    }
    
    fn peek(&self) -> i32 {
        *self.stack.get(0).unwrap()
    }
    
    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}


#[cfg(test)]
mod tests {
    /*
    * Your MyQueue object will be instantiated and called as such:
    * let obj = MyQueue::new();
    * obj.push(x);
    * let ret_2: i32 = obj.pop();
    * let ret_3: i32 = obj.peek();
    * let ret_4: bool = obj.empty();
    */
    use super::*;

    #[test]
    fn test_queue() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert_eq!(q.empty(), false);
        assert_eq!(q.pop(), 2);
        assert_eq!(q.empty(), true);

    }
}
