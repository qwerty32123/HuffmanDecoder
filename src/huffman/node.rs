#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Node {
    pub char: Option<char>,
    pub freq: usize,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn leaf(c: char, freq: usize) -> Self {
        Node {
            char: Some(c),
            freq,
            left: None,
            right: None,
        }
    }

    pub fn internal(freq: usize, left: Node, right: Node) -> Self {
        Node {
            char: None,
            freq,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}