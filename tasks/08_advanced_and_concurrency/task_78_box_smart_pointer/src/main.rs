#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    println!("=== SMART POINTERS (BOX<T>) ===");

    // TODO 1: Allocate integer 100 on the heap with Box::new
    let boxed_val = Box::new(100);
    println!("Heap Allocated Integer:  {}", *boxed_val);

    // TODO 2: Construct linked list: 10 -> 20 -> 30 -> Nil
    let my_list = Cons(10, Box::new(Cons(20, Box::new(Cons(30, Box::new(Nil))))));
    println!("Constructed Linked List: {:?}", my_list);

    println!("===============================");
}

