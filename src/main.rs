mod graph;
mod product;

use graph::ProductGraph;
use product::Product;
use rand::{seq::SliceRandom, Rng};

fn main() {
    let mut graph = ProductGraph::new();
    let mut rng = rand::thread_rng();

    // Geração de 1000 produtos com nomes variados
    let categories = vec!["Eletrônicos", "Vestuário", "Decoração", "Alimentos", "Esportes"];
    let brands = vec!["TechBrand", "StyleCo", "HomeLux", "Foodies", "SportPro"];
    let mut products = Vec::new();

    for id in 1..=1000 {
        let category = categories.choose(&mut rng).unwrap().to_string();
        let brand = brands.choose(&mut rng).unwrap().to_string();
        let name = format!("Produto {} - {}", id, category);

        let product = Product {
            id,
            name,
            brand,
            category,
        };

        graph.add_product(product.clone());
        products.push(product);
    }

    // Criar conexões aleatórias entre produtos (recomendações)
    for product in &products {
        let num_recs = rng.gen_range(2..6); // cada produto recomenda de 2 a 5 outros
        let recs = products.choose_multiple(&mut rng, num_recs);

        for rec in recs {
            if product.id != rec.id {
                graph.add_edge(product.id, rec.id);
            }
        }
    }

    // Simular buscas por palavras-chave e mostrar recomendações
    let keywords = vec!["produto 10", "produto 500", "produto 999", "eletrônicos", "alimentos"];

    for keyword in keywords {
        println!("\n🔍 Busca por '{}':", keyword);
        let results = graph.search_by_name(keyword);

        if results.is_empty() {
            println!("Nenhum produto encontrado.");
        } else {
            for product in results {
                println!("→ {:?}", product);
                let recs = graph.recommend(product.id);
                if recs.is_empty() {
                    println!("   ⚠️ Nenhuma recomendação disponível.");
                } else {
                    println!("   💡 Recomendações:");
                    for rec in recs {
                        println!("     - {:?}", rec);
                    }
                }
            }
        }
    }
}