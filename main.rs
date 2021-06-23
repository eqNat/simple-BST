struct Tree(Box<Option<(i32, Tree, Tree)>>);

fn insert(mut x: &mut Tree, val: i32) {
    while let Some((v, ref mut left, ref mut right)) = *x.0 {
        x =  if v > val {println!("left"); left } else {println!("right"); right };
    }
    *x = Tree(Box::new(Some((val, Tree(Box::new(None)), Tree(Box::new(None))))));
    println!("inserted");
}

fn main() {
    let mut b = Tree(Box::new(None));
    insert(&mut b, 100);
    insert(&mut b, 50);
    insert(&mut b, 25);
    insert(&mut b, 150);
    insert(&mut b, 125);
    insert(&mut b, 175);
    if let Some((i, _, _)) = *b.0 {
        println!("root data: {}", i);
    }
}
