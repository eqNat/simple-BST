enum Status {
    Unique,
    Duplicate,
}

struct Tree<T>(Box<Option<(T, Tree<T>, Tree<T>)>>);

impl<T: std::cmp::PartialEq + std::cmp::PartialOrd + std::fmt::Display> Tree<T>
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
        return Status::Unique
    }

    fn find(mut self: &mut Tree<T>, val: T) -> Status
    {
        while let Some((ref data, ref mut left, ref mut right)) = *self.0
        {
            if *data == val { return Status::Duplicate }
            self = if *data > val { left } else { right }
        }
        return Status::Unique
    }

    fn print(self)
    {
        if let Some((data, left, right)) = *self.0
        {
            Tree::print(left);
            println!("{}", data);
            Tree::print(right);
        }
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
    b.insert(75);

    if let Status::Duplicate = b.find(125) {
        println!("125 is found");
    } else {
        println!("125 is not found");
    }

    if let Status::Duplicate = b.find(127) {
        println!("127 is found");
    } else {
        println!("127 is not found");
    }

    b.print();
}
