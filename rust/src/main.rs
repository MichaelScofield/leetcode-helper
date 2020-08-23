mod helper;

struct Solution;

struct MyQueue {
    pub stack: Vec<i32>,
    pub reverse_stack: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack: Vec::<i32>::new(),
            reverse_stack: Vec::<i32>::new(),
        }
    }

    fn flush_stack(&mut self) {
        while let Some(v) = self.stack.pop() {
            self.reverse_stack.push(v);
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x)
    }

    fn pop(&mut self) -> i32 {
        match self.reverse_stack.pop() {
            Some(e) => { e }
            None => {
                self.flush_stack();
                self.reverse_stack.pop().unwrap()
            }
        }
    }

    fn peek(&mut self) -> i32 {
        *match self.reverse_stack.last() {
            Some(e) => { e }
            None => {
                self.flush_stack();
                self.reverse_stack.last().unwrap()
            }
        }
    }

    fn empty(&self) -> bool {
        self.reverse_stack.is_empty() && self.stack.is_empty()
    }
}

fn main() {
    let queue = &mut MyQueue::new();
    println!("{}", queue.empty());
    queue.push(1);
    queue.push(2);
    println!("{}", queue.peek());
    println!("{}", queue.pop());
    println!("{}", queue.pop());
    println!("{}", queue.empty());
    queue.push(3);
    println!("{}", queue.peek());
    println!("{}", queue.empty());
}
