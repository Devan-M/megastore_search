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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::Product;

    fn sample_products() -> Vec<Product> {
        vec![
            Product { id: 1, name: "Smartphone X".into(), brand: "TechBrand".into(), category: "Eletrônicos".into() },
            Product { id: 2, name: "Smartphone Y".into(), brand: "OtherBrand".into(), category: "Eletrônicos".into() },
            Product { id: 3, name: "Capinha X".into(), brand: "TechBrand".into(), category: "Acessórios".into() },
            Product { id: 4, name: "Carregador Turbo".into(), brand: "PowerPlus".into(), category: "Acessórios".into() },
        ]
    }

    #[test]
    fn test_add_product_and_search() {
        let mut graph = ProductGraph::new();
        for p in sample_products() {
            graph.add_product(p);
        }

        let results = graph.search_by_name("smartphone");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_no_match() {
        let mut graph = ProductGraph::new();
        graph.add_product(Product { id: 1, name: "Smartphone X".into(), brand: "TechBrand".into(), category: "Eletrônicos".into() });

        let results = graph.search_by_name("geladeira");
        assert!(results.is_empty());
    }

    #[test]
    fn test_recommendations() {
        let mut graph = ProductGraph::new();
        let products = sample_products();

        for p in &products {
            graph.add_product(p.clone());
        }

        graph.add_edge(1, 3);
        graph.add_edge(1, 4);

        let recs = graph.recommend(1);
        let rec_names: Vec<&str> = recs.iter().map(|p| p.name.as_str()).collect();

        assert!(rec_names.contains(&"Capinha X"));
        assert!(rec_names.contains(&"Carregador Turbo"));
        assert_eq!(rec_names.len(), 2);
    }

    #[test]
    fn test_recommend_no_edges() {
        let mut graph = ProductGraph::new();
        graph.add_product(Product { id: 1, name: "Smartphone X".into(), brand: "TechBrand".into(), category: "Eletrônicos".into() });

        let recs = graph.recommend(1);
        assert!(recs.is_empty());
    }

    #[test]
    fn test_recommend_nonexistent_product() {
        let graph = ProductGraph::new();
        let recs = graph.recommend(999);
        assert!(recs.is_empty());
    }

    #[test]
    fn test_graph_integrity() {
        let mut graph = ProductGraph::new();
        let p1 = Product { id: 1, name: "Smartphone X".into(), brand: "TechBrand".into(), category: "Eletrônicos".into() };
        let p2 = Product { id: 2, name: "Capinha X".into(), brand: "TechBrand".into(), category: "Acessórios".into() };

        graph.add_product(p1.clone());
        graph.add_product(p2.clone());
        graph.add_edge(p1.id, p2.id);

        assert!(graph.nodes.contains_key(&1));
        assert!(graph.edges.contains_key(&1));
        assert!(graph.edges.get(&1).unwrap().contains(&2));
    }
}