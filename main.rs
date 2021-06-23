enum Status {
    Success,
    Duplicate,
}

struct Tree<T>(Box<Option<(T, Tree<T>, Tree<T>)>>);
impl<T: std::cmp::PartialEq + std::cmp::PartialOrd> Tree<T> {
    fn insert(mut self: &mut Tree<T>, val: T) -> Status {
        while let Some((ref v, ref mut left, ref mut right)) = *self.0 {
            if *v == val {
                return Status::Duplicate
            }
            self =  if *v > val { left } else { right };
        }
        *self = Tree(Box::new(Some((val, Tree(Box::new(None)), Tree(Box::new(None))))));
        return Status::Success
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
