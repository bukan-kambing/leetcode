use std::cmp::{min, max};

struct MinStack {
  stack : Vec<(i32, i32)>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
      Self {
        stack: Vec::with_capacity(30_000)
      }
    }
    
    fn push(&mut self, val: i32) {
        match self.stack.is_empty() {
          true => self.stack.push((val, val)),
          false => {
            let min_val = min(val, self.get_min());
            self.stack.push((val, min_val));
          }
        }
    }
    
    fn pop(&mut self) {
      self.stack.pop();
    }
    
    fn top(&self) -> i32 {
      return match self.stack.last() {
        Some(val) => val.0,
        _ => 0,
      };
    }
    
    fn get_min(&self) -> i32 {
      return match self.stack.last() {
        Some(val) => val.1,
        _ => 0,
      };
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
