use bit_vec::BitVec;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::{collections::HashMap, hash::Hash, error::Error};
use rmp_serde;
use serde::{Deserialize, Serialize};
\
use crate::huffman::huffman_tree;
use crate::huffman::*;
use Tree::*;

#[derive(Serialize, Deserialize)]
struct CompressedData<T: Eq + Hash> {
    encoder: HashMap<T, BitVec>,
    data: Vec<BitVec>,
}

fn compress<'a, T, FreqsF, TokenExtractor, TokensIter> (
    lines: &'a Vec<String>,
    get_freqs: FreqsF,
    lines_to_tokens: TokenExtractor,
) -> Result<Vec<u8>, Box<dyn Error>> 

where 
    T: Clone + Eq + Hash + Send + Sync + Serialize,
    FreqsF: Fn(&'a Vec<String>) -> HashMap<T, u64>,
    TokenExtractor: Fn(&'a str) -> TokensIter + Send + Sync,
    TokensIter: Iterator<Item = T>

{
    let freqs = get_freqs(lines);
    let tree = huffman_tree(&freqs); 
    let encoder = tree.to_encoder();

    let data = lines
        .par_iter()
        .map(|line| {
            lines_to_tokens(line)
                .map(|token| encoder.get(&token).unwrap().clone())
                .fold(
                    BitVec::new(),
                    |mut vec1, vec2| {
                        vec1.extend(vec2);
                        vec1
                    }
                )
        })
        .collect();
    
    let compressed_data = CompressedData { encoder, data };
    rmp_serde::encode::to_vec(&compressed_data).map_err(|err| err.into()) 
}

pub fn extract<'a, T, F> (
    data: &'a Vec<u8>,
    tokens_to_line: F,
) -> Result<Vec<String>, Box<dyn Error>> 

where 
    T: Clone + Eq + Hash + Send + Sync + Deserialize<'a>,
    F: Fn(Vec<T>) -> String + Send + Sync,
{

}

impl<T: Eq + Clone + Hash> Tree<T> {
    pub fn to_encoder(&self) -> HashMap<T, BitVec> {
        let mut encoder = HashMap::new();

        let mut stack = vec![(self, BitVec::new())];
        while !stack.is_empty() {
            let (node, path) = stack.pop().unwrap();
            match node {
                Leaf { token, .. } => {
                    encoder.insert(token.clone(), path.clone());
                }
                Node { left, right, .. } => {
                    let mut left_path = path.clone();
                    left_path.push(false);
                    stack.push((left, left_path));

                    let mut right_path = path.clone();
                    right_path.push(true);
                    stack.push((right, right_path));
                }
            }
        }
        encoder
    }
}
