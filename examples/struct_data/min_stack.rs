// https://leetcode-cn.com/problems/min-stack/submissions/

#![allow(unused)]
struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        let stack = Vec::new();
        let min_stack = Vec::new();
        MinStack { stack, min_stack }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);

        // standard
        if self.min_stack.is_empty() || val <= *self.min_stack.last().unwrap() {
            self.min_stack.push(val);
        }

        // mine
        // if self.min_stack.len() == 0 {
        //     self.min_stack.push(val);
        // } else {
        //     // 从大到小push
        //     // 遍历min_stack, 找到val的位置，push val；
        //     for index in 0..self.min_stack.len() {
        //         let value = self.min_stack.get(index);
        //         if *value.unwrap() < val {
        //             self.min_stack.insert(index, val);
        //             break;
        //         } else {
        //             if index == self.min_stack.len() - 1 {
        //                 self.min_stack.push(val);
        //             }
        //         }
        //     }
        // }
    }

    fn pop(&mut self) {
        if self.min_stack.is_empty() {
            return;
        }
        // standard
        if self.stack.pop().unwrap() == *self.min_stack.last().unwrap() {
            self.min_stack.pop();
        }

        // mine
        // let val = self.stack.pop().unwrap();
        // // 遍历min_stack, 找到val的位置，del val；
        // for index in 0..self.min_stack.len() {
        //     if val == *self.min_stack.get(index).unwrap() {
        //         self.min_stack.remove(index);
        //         break;
        //     }
        // }
    }

    fn top(&self) -> i32 {
        let temp = self.stack.last();
        return *temp.unwrap();
    }

    fn get_min(&self) -> i32 {
        return *self.min_stack.last().unwrap();
    }
}

fn main() {
    let mut min_stack = MinStack::new();
    min_stack.push(0);
    min_stack.push(1);
    min_stack.push(2);
    min_stack.push(-2);
    min_stack.push(-3);
    println!("get_min :{}", min_stack.get_min());
    println!("top :{}", min_stack.top());
    min_stack.pop();
    println!("top :{}", min_stack.top());
    println!("get_min :{}", min_stack.get_min());
}
