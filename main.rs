enum Status {
    Success,
    Duplicate,
}

struct Tree<T>(Box<Option<(T, Tree<T>, Tree<T>)>>);

impl<T: std::cmp::PartialEq + std::cmp::PartialOrd> Tree<T>
{
    fn new() -> Tree<T>
    {
        Tree(Box::new(None))
    }

    fn insert(mut self: &mut Tree<T>, val: T) -> Status
    {
        while let Some((ref data, ref mut left, ref mut right)) = *self.0
        {
            if *data == val { return Status::Duplicate }
            self = if *data > val { left } else { right }
        }
        *self = Tree(Box::new(Some((val, Tree::new(), Tree::new()))));
        return Status::Success
    }
}

fn main()
{
    let mut b = Tree::new();
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
