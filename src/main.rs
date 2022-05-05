#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
use futures::executor::block_on;
use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Cell, Ref, RefCell},
    rc::Rc,
    thread,
    time::Duration,
};
struct Student {
    name: &'static str,
    age: u32,
    score: i32,
}
impl Student {
    fn new(name: &'static str, age: u32, score: i32) -> Self {
        Student { name, age, score }
    }
    fn get_name(&self) -> &str {
        return self.name;
    }
}

pub trait Messenger {
    fn send(&self, msg: String);
}

use restaurant::eat_at_restaurant;

fn main() {
    // println!("Hello, world!");

    // struct相关
    // let score = 56;
    // let username = "Coda";
    // let age = 18;

    // let student = Student::new(
    //   username,
    //   age,
    //   score
    // );

    // println!("name:{}, score: {}", student.get_name(), student.score);

    // let student2 = Student{
    //   name: "GPP",
    //   age: 15,
    //   ..student
    // };
    // fn test(student: Student) {
    //   println!("name: {}, age: {}, score: {}", student.name, student.age, student.score);
    // }
    // test(student2);

    // Option相关
    // let x: i8 = 5;
    // let y: Option<i8> = None;
    // let sum = x + y.unwrap_or(0);
    // println!("sum: {}", sum);

    // 数组相关
    let a = [9, 8, 7, 6, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, [2, 3]);
    assert_eq!(slice, &[2, 3]);
    // let mut vec1 = vec![10,20,30];
    // for i in &mut vec1 {
    //   *i += 30;
    //   print!("{} ", i);
    // }

    // 测试一下所有权转移的特点
    // fn test1(str: &str) {
    //     println!("test1打印{}", str);
    // }
    // fn test2(str: &str) {
    //     println!("test2打印{}", str);
    // }
    // let s = String::from("Hello World");

    // test1(&s);
    // test2(&s);
    // println!("源main函数里打印{}", s);

    // Rc相关
    // let x = &false;
    // println!("{}", x);
    // let &x = &false;
    // println!("{}", x);
    // let ref x = &false;
    // println!("{}", x);

    // Cell相关
    // let c = Cell::new("asdf"); // &str实现了Copy Trait 特征
    // let one = c.get();
    // c.set("qwer");
    // println!("{}", one);

    // Cell存在的意义：可以允许多个引用；
    // let x = Cell::new(1);
    // let y = &x;
    // let z = &x;
    // x.set(2);
    // y.set(3);
    // z.set(4);
    // println!("{}", x.get());
    // let mut x = 1;
    // let y = &mut x;
    // let z = &mut x;
    // x = 2;
    // *y = 3;
    // *z = 4;

    // RefCell存在的意义，不可变借用
    // struct MsgQueue {
    //     msg_cache: RefCell<Vec<String>>,
    // }
    // impl Messenger for MsgQueue {
    //     fn send(&self, msg: String) {
    //         // self.msg_cache.push(msg) // 因为&sefl是不可变借用，所以没法可变的借用self.msg_cache
    //         self.msg_cache.borrow_mut().push(msg)
    //     }
    // }
    // let mq = MsgQueue {
    //     msg_cache: RefCell::new(Vec::new()),
    // };
    // mq.send("hello world".to_string());

    // Rc + RefCell
    // let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));
    // let s1 = s.clone();
    // let s2 = s.clone();
    // let ss: &RefCell<String> = &s2;
    // ss.borrow_mut().push_str(", on yeah!");
    // println!("{:?}\n{:?}\n{:?}", s, s1, s2);

    // 多线程
    // let thread_1 = thread::spawn(|| {
    //     for i in 1..5 {
    //         println!("number {} from thread_1", i);
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // thread_1.join().unwrap();

    // for i in 1..5 {
    //     println!("number {} from main thread", i);
    //     thread::sleep(Duration::from_secs(2));
    // }

    // 异步编程
    // async fn hello_async() {
    //     hello_cat().await;
    //     println!("hello async !");
    // }

    // async fn hello_cat() {
    //     println!("hello kitty !");
    // }

    // async fn hello_world() {
    //     println!("hello world !");
    // }

    // async fn run() {
    //     let f1 = hello_async();
    //     let f2 = hello_world();
    //     // block_on(future);
    //     futures::join!(f1, f2);
    // }

    // block_on(run());

    // 模块化
    eat_at_restaurant();
}
