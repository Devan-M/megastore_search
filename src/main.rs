mod graph;
mod product;

use graph::ProductGraph;
use product::Product;

fn main() {
    let mut graph = ProductGraph::new();

    // Produtos de exemplo
    let smartphone = Product {
        id: 1,
        name: "Smartphone X".to_string(),
        brand: "TechBrand".to_string(),
        category: "Eletrônicos".to_string(),
    };

    let capinha = Product {
        id: 2,
        name: "Capinha X".to_string(),
        brand: "TechBrand".to_string(),
        category: "Acessórios".to_string(),
    };

    let carregador = Product {
        id: 3,
        name: "Carregador Turbo".to_string(),
        brand: "PowerPlus".to_string(),
        category: "Acessórios".to_string(),
    };

    // Adiciona produtos ao grafo
    graph.add_product(smartphone.clone());
    graph.add_product(capinha.clone());
    graph.add_product(carregador.clone());

    // Define relações entre produtos
    graph.add_edge(smartphone.id, capinha.id);
    graph.add_edge(smartphone.id, carregador.id);

    // Busca por nome
    println!("🔍 Resultados da busca por 'smartphone':");
    for p in graph.search_by_name("smartphone") {
        println!("{:?}", p);
    }

    // Recomendações
    println!("\n🤝 Recomendações para '{}':", smartphone.name);
    for p in graph.recommend(smartphone.id) {
        println!("{:?}", p);
    }
}

