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
    fn insert(&mut self, data: i32) {
        while let Some((val, l, r)) = &*self.0 {
            println!("loop");
        }
    }
}

struct Node(Box<Option<(i32, i32)>>);
impl Node {
    fn print(&mut self) {
        while let Some((i, j)) = *self.0 {
            println!("in Node: {} {}", i, j);
            break;
        }
    }
}

fn main() {
    let mut stack_var: i32 = 10;
            
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

    let mut n = Node(Box::new(Some((65, 32))));
    n.print();

}
