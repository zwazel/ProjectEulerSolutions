extern crate core;

use core::panicking::panic;

fn main() {
    // create a pyramid using PyramidNode struct
    let mut pyramid = vec![75, 95, 64, 17, 47, 82, 18, 35, 87, 10, 20, 04, 82, 47, 65, 19, 01, 23, 75, 03, 34, 88, 02, 77, 73, 07, 63, 67, 99, 65, 04, 28, 06, 16, 70, 92, 41, 41, 26, 56, 83, 40, 80, 70, 33, 41, 48, 72, 33, 47, 32, 37, 16, 94, 29, 53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14, 70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57, 91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48, 63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31, 04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23];
    let mut starter = PyramidNode::build(&mut pyramid);

    // print the pyramid
    let mut starter = starter.unwrap();
    let left = starter.left.unwrap();
    println!("root left: {}, left: {}, right: ", left.value, left.left.unwrap().value);
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

    // function to build a pyramid from a vector of integers
    fn build(pyramid: &mut Vec<i32>) -> Option<Box<PyramidNode>> {
        if pyramid.len() == 0 {
            return None;
        }

        let mut pyramid_node = PyramidNode::new(pyramid.remove(0));
        pyramid_node.left = PyramidNode::build(pyramid);
        pyramid_node.right = PyramidNode::build(pyramid);
        Some(Box::new(pyramid_node))
    }

    fn add_left(&mut self, value: i32) {
        self.left = Some(Box::new(PyramidNode::new(value)));
    }

    fn set_left(&mut self, pyramid_node: PyramidNode) -> Option<&Box<PyramidNode>> {
        let node = Some(Box::new(pyramid_node));

        self.left = node;
        node.as_ref()
    }

    fn add_right(&mut self, value: i32) {
        self.right = Some(Box::new(PyramidNode::new(value)));
    }

    fn set_right(&mut self, pyramid_node: PyramidNode) -> Option<&Box<PyramidNode>> {
        let node = Some(Box::new(pyramid_node));

        self.right = node;
        node.as_ref()
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