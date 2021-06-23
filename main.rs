struct Tree(Box<Option<(i32, Tree, Tree)>>);

fn allocate(x: &mut Tree) {
    println!("x data: {:p}", x.0);

    if let None = *x.0 {
        *x = Tree(Box::new(Some((35, Tree(Box::new(None)), Tree(Box::new(None))))));
    }
}

fn main() {
    let mut b = Tree(Box::new(None));
    allocate(&mut b);
    if let Some((i, _, _)) = *b.0 {
        println!("root data: {}", i);
    }
}
