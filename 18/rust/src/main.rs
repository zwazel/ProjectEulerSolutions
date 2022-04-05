fn main() {
    // create a pyramid using PyramidNode struct
    let mut starter = PyramidNode::new(3);

    starter.add_left(7);
    starter.add_right(4);

    let mut left = starter.left.as_mut().unwrap();
    left.add_left(2);
    left.add_right(6);

    let mut right = starter.right.as_mut().unwrap();
    right.add_left(4);
    right.add_right(6);

    // print the pyramid
    starter.print();
}

struct PyramidNode {
    value: i32,
    left: Option<Box<PyramidNode>>,
    right: Option<Box<PyramidNode>>,
}

impl PyramidNode {
    fn new(value: i32) -> PyramidNode {
        PyramidNode {
            value,
            left: None,
            right: None,
        }
    }

    fn add_left(&mut self, value: i32) {
        self.left = Some(Box::new(PyramidNode::new(value)));
    }

    fn add_right(&mut self, value: i32) {
        self.right = Some(Box::new(PyramidNode::new(value)));
    }

    fn print(&self) {
        println!("{}", self.value);
        if let Some(ref left) = self.left {
            left.print();
        }
        if let Some(ref right) = self.right {
            right.print();
        }
    }
}