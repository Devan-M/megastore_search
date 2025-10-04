# MegaStore Search

**MegaStore Search** é um sistema de recomendação de produtos utilizando grafos, desenvolvido em **Rust**. O objetivo do projeto é melhorar a eficiência e precisão da busca em grandes catálogos de produtos em plataformas de e-commerce, como o caso fictício da **MegaStore**.

---

## 🚀 Funcionalidades

- **Indexação eficiente** de produtos por múltiplos critérios (nome, marca, categoria, etc.).
- **Busca rápida e precisa** em grandes volumes de dados.
- **Escalabilidade**, permitindo o crescimento sem perder performance.
- **Recomendações baseadas em grafos**, para melhorar a experiência de compra do usuário.

---

## ⚙️ Tecnologias Usadas

- **Rust**: Linguagem de programação de sistemas de alto desempenho.
- **Cargo**: Gerenciador de pacotes e build para Rust.
- **Graph Databases**: Utiliza conceitos de grafos para otimizar buscas e recomendações.
- **Serde**: Para serialização e deserialização de dados.

---

## 📦 Como Instalar

### Requisitos

- **Rust** (versão >= 1.56)
- **Cargo** (geralmente já vem com o Rust)

### Passos para Instalação

1. **Clone o repositório:**
   ```bash
   git clone https://github.com/Devan-M/megastore_search.git
   ```

2. **Entre no diretório do projeto:**
   ```bash
   cd megastore_search
   ```

3. **Instale as dependências e compile o projeto:**
   ```bash
   cargo build --release
   ```

4. **Execute o projeto:**
   ```bash
   cargo run
   ```
   Isso vai compilar e rodar o sistema localmente.

---

## 🛠️ Como Usar

Para usar o **MegaStore Search**, o processo básico envolve:

- **Adicionar produtos ao banco de dados:** Os produtos podem ser indexados com base em diversos critérios (nome, categoria, marca).
- **Buscar por produtos:** Utilize a função de busca para encontrar produtos com base em diferentes parâmetros (como nome, categoria ou marca).
- **Recomendações:** O sistema pode gerar recomendações baseadas nos dados de busca e histórico de compras, utilizando algoritmos baseados em grafos.

### Exemplo de Uso

```rust
use megastore_search::search_product;

let produtos = search_product("nome_do_produto");

for produto in produtos {
    println!("{}", produto.nome);
}
```

### Parâmetros de Busca

- `nome`: Nome do produto a ser pesquisado.
- `categoria`: Categoria de produtos.
- `marca`: Marca do produto.

---

## 📚 Estrutura de Diretórios

| Diretório/Arquivo | Descrição |
|--------------------|-----------|
| `src/`             | Contém o código-fonte do sistema. |
| `main.rs`          | Arquivo principal que executa o sistema de busca e indexação. |
| `search.rs`        | Contém a lógica de busca e recomendação de produtos. |
| `database.rs`      | Gerencia a persistência de dados. |
| `Cargo.toml`       | Arquivo de configuração de dependências do Cargo. |
| `Cargo.lock`       | Arquivo gerado automaticamente pelo Cargo que especifica as versões exatas das dependências. |

---

## 🔧 Como Contribuir

1. Faça um **fork** do repositório.
2. Crie uma **branch** para a sua feature:
   ```bash
   git checkout -b minha-feature
   ```
3. **Commit** suas mudanças:
   ```bash
   git commit -am 'Adiciona nova funcionalidade'
   ```
4. **Push** para o branch:
   ```bash
   git push origin minha-feature
   ```
5. Crie um **pull request**.

---

## 📝 Licença

Este projeto está licenciado sob a **MIT License** — veja o arquivo [LICENSE](LICENSE) para mais detalhes.
