struct Tree(Box<Option<(i32, Tree, Tree)>>);

impl Tree {
    fn insert(mut self: &mut Tree, val: i32) {
        while let Some((v, ref mut left, ref mut right)) = *self.0 {
            if v == val {
                return
            }
            self =  if v > val { left } else { right };
        }
        *self = Tree(Box::new(Some((val, Tree(Box::new(None)), Tree(Box::new(None))))));
    }
}

fn main() {
    let mut b = Tree(Box::new(None));
    b.insert(100);
    b.insert(50);
    b.insert(25);
    b.insert(150);
    b.insert(125);
    b.insert(175);
    if let Some((i, _, _)) = *b.0 {
        println!("root data: {}", i);
    }
}
