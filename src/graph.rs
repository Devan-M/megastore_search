use std::collections::{HashMap, HashSet};
use crate::product::Product;

pub struct ProductGraph {
    pub nodes: HashMap<usize, Product>,
    pub edges: HashMap<usize, HashSet<usize>>,
}

impl ProductGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
    let product_id = product.id;
    self.nodes.insert(product_id, product);
    self.edges.entry(product_id).or_insert(HashSet::new());
}


    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.entry(from).or_insert(HashSet::new()).insert(to);
    }

    pub fn search_by_name(&self, query: &str) -> Vec<&Product> {
        self.nodes.values()
            .filter(|p| p.name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    pub fn recommend(&self, product_id: usize) -> Vec<&Product> {
        self.edges.get(&product_id)
            .map(|neighbors| {
                neighbors.iter()
                    .filter_map(|id| self.nodes.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
}