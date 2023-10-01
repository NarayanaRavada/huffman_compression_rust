use std::{collections::{HashMap, BinaryHeap}, cmp::Reverse};
use Tree::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tree<T> {
    Leaf {
        freq: u64,
        token: T,
    },
    Node {
        freq: u64,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}

impl <T: Clone> Tree<T> {
    pub fn freq(&self) -> u64 {
        match self {
            Leaf { freq, .. } => *freq,
            Node { freq, .. } => *freq,
        }
    }

    pub fn token(&self) -> Option<T> {
        match self {
            Leaf { token, .. } => Some(token.clone()),
            Node { .. } => None, 
        }
    }

    pub fn left(&self) -> Option<&Tree<T>> {
        match self {
            Node { left, .. } => Some(left),
            Leaf { .. } => None,
        }
    }

    pub fn right(&self) -> Option<&Tree<T>> {
        match self {
            Node { right, .. } => Some(right),
            Leaf { .. } => None,
        }
    }
}

impl<T: Clone + Eq> Ord for Tree<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq().cmp(&other.freq())
    }
}

impl<T: Clone + Eq> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn huffman_tree<T: Clone + Eq>(freqs: &HashMap<T, u64>) -> Tree<T> {
    let mut heap = BinaryHeap::new();
    for (token, freq) in freqs {
        let (freq, token) = (*freq, token.clone());
        heap.push(Reverse(Leaf { freq, token }));
    }

    while heap.len() > 1 {
        let node1 = heap.pop().unwrap().0;
        let node2 = heap.pop().unwrap().0;

        let merged_node = Node {
            freq: node1.freq() + node2.freq(),
            left: Box::new(node1),
            right: Box::new(node2),
        };
        
        heap.push(Reverse(merged_node));
    }

    heap.pop().unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frequency::*;

    #[test]
    fn find_freqs() {
        let input = vec!["this a quote".to_string(), "dont cry because its over, smile because it happend".to_string()];
        let freqs = char_freqs(&input);
        assert_eq!(freqs[&' '], 10);
        assert_eq!(freqs[&'t'], 5);
        assert_eq!(freqs[&','], 1);
        assert_eq!(freqs[&'q'], 1);
    }

    #[test]
    fn huffman_tree_works() {
        let mut freqs = HashMap::new();
        freqs.insert('a', 40);
        freqs.insert('b', 30);
        freqs.insert('c', 20);
        freqs.insert('d', 10);

        let tree = huffman_tree(&freqs);

        assert_eq!(tree.freq(), 100);

        // most frequent char => 1bit
        assert_eq!(tree.left().and_then(|node| node.token()), Some('a'));
        assert_eq!(tree.left().map(|node| node.freq()), Some(40));

        // 2ns most frequent char => 2bits
        assert_eq!(
            tree.right()
                .and_then(|node| node.left())
                .and_then(|node| node.token()),
            Some('b')
        );
        assert_eq!(
            tree.right()
                .and_then(|node| node.left())
                .map(|node| node.freq()),
            Some(30)
        );

        // least frequency 3bits
        assert_eq!(
            tree.right()
                .and_then(|node| node.right())
                .and_then(|node| node.left())
                .and_then(|node| node.token()),
            Some('d')
        );
        assert_eq!(
            tree.right()
                .and_then(|node| node.right())
                .and_then(|node| node.left())
                .map(|node| node.freq()),
            Some(10)
        );

        assert_eq!(
            tree.right()
                .and_then(|node| node.right())
                .and_then(|node| node.right())
                .and_then(|node| node.token()),
            Some('c')
        );
        assert_eq!(
            tree.right()
                .and_then(|node| node.right())
                .and_then(|node| node.right())
                .map(|node| node.freq()),
            Some(20)
        );
    }
}
