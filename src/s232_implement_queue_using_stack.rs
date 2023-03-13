
#[derive(Default, Debug)]
struct MyQueue {
    front: Vec<i32>,
    back: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Self::default()
    }
    
    fn push(&mut self, x: i32) {
        self.back.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        if self.front.is_empty() {
            while let Some(val) = self.back.pop() {
                self.front.push(val);
            }
        }
        self.front.pop().unwrap()
    }
    
    fn peek(&mut self) -> i32 {
        if self.front.is_empty() {
            while let Some(val) = self.back.pop() {
                self.front.push(val);
            }
        }
        *self.front.last().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.front.is_empty() && self.back.is_empty()
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
