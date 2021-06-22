fn insert(node: & Box<Option<i32>>) {
    match **node {
        Option::Some(i) => {
            println!("insert: node data: {}", i);
            println!("insert: &node address: {:p}", &i);
            println!("insert: &&node address: {:p}", &&i);
        },
        Option::None => println!("nothing here in 'insert'")
    }
}

fn insert2(node: & Box<i32>) {
    println!("insert2: node data: {}", node);
    println!("insert2: node address: {:p}", node);
    println!("insert2: *node address: {:p}", *node);
}

struct Tree(Box<Option<(i32, Tree, Tree)>>);
impl Tree {
    fn testing(&self) {
        println!("in Tree: {:p}", self);
    }
}

struct Node(Box<Option<i32>>);
impl Node {
    fn print(&self) {
        match *self.0 {
            Some(i) => println!("in Node: {}", i),
            None => println!("empty in Node")
        }
    }
}

fn main() {
    let stack_var: i32 = 10;
            
    println!("stack_var: {:p}", &stack_var);

    println!("");

    let node = Box::new(Some(78));
    println!("node: {:p}", node);
    println!("&node: {:p}", &node);
    insert(&node);

    println!("");

    let node2 = Box::new(80);
    println!("node2: {:p}", node);
    println!("&node2: {:p}", &node);
    insert2(&node2);

    println!("");

    let n = Node(Box::new(Some(65)));
    n.print();
}
