enum Status {
    Unique,
    Duplicate,
}

enum Tree<T> {
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
    Nil,
}

impl<T: std::cmp::PartialEq + std::cmp::PartialOrd + std::fmt::Display> Tree<T>
{
    fn new() -> Tree<T>
    {
        Tree::Nil
    }

    fn insert(mut self: &mut Tree<T>, val: T) -> Status
    {
        while let Tree::Node(ref data, ref mut left, ref mut right) = *self
        {
            if *data == val { return Status::Duplicate }
            self = if *data > val { left } else { right }
        }
        *self = Tree::Node(val, Box::new(Tree::Nil), Box::new(Tree::Nil));
        return Status::Unique
    }

    fn find(mut self: &Tree<T>, val: T) -> Status
    {
        while let Tree::Node(ref data, ref left, ref right) = *self
        {
            if *data == val { return Status::Duplicate; }
            self = if *data > val { left } else { right }
        }
        return Status::Unique
    }

    fn print(self)
    {
        if let Tree::Node(data, left, right) = self
        {
            Tree::print(*left);
            println!("{}", data);
            Tree::print(*right);
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
