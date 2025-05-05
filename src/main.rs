use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Produto {
    id: u32,
    nome: String,
}

struct Recomendador {
    produtos: HashMap<u32, Produto>,
    arestas: HashMap<u32, HashSet<u32>>,
}

impl Recomendador {
    fn new() -> Self {
        Recomendador {
            produtos: HashMap::new(),
            arestas: HashMap::new(),
        }
    }

    fn adicionar_produto(&mut self, produto: Produto) {
        self.produtos.insert(produto.id, produto.clone());
        self.arestas.entry(produto.id).or_insert_with(HashSet::new);
    }

    fn adicionar_similaridade(&mut self, id1: u32, id2: u32) {
        self.arestas.entry(id1).or_insert_with(HashSet::new).insert(id2);
        self.arestas.entry(id2).or_insert_with(HashSet::new).insert(id1);
    }

    fn recomendar(&self, produto_id: u32) -> Vec<String> {
        let mut recomendacoes = Vec::new();
        if let Some(similares) = self.arestas.get(&produto_id) {
            for id in similares {
                if let Some(produto) = self.produtos.get(id) {
                    recomendacoes.push(produto.nome.clone());
                }
            }
        }
        recomendacoes
    }
}

fn main() {
    let mut sistema = Recomendador::new();

    sistema.adicionar_produto(Produto { id: 1, nome: "Notebook Gamer".to_string() });
    sistema.adicionar_produto(Produto { id: 2, nome: "Mouse Gamer".to_string() });
    sistema.adicionar_produto(Produto { id: 3, nome: "Teclado Mecânico".to_string() });
    sistema.adicionar_produto(Produto { id: 4, nome: "Monitor 144Hz".to_string() });

    sistema.adicionar_similaridade(1, 2);
    sistema.adicionar_similaridade(1, 3);
    sistema.adicionar_similaridade(2, 4);

    let produto_alvo = 1;
    let recomendacoes = sistema.recomendar(produto_alvo);

    println!("Recomendações para '{}':", sistema.produtos[&produto_alvo].nome);
    for nome in recomendacoes {
        println!("- {}", nome);
    }
}
