# MegaStore Recommender

Este é um sistema de recomendação de produtos baseado em grafos e tabelas hash, desenvolvido como parte de um trabalho acadêmico. O objetivo do projeto é otimizar o processo de busca por produtos no e-commerce MegaStore.

## Funcionalidades

- **Cadastro de produtos**: O sistema permite cadastrar produtos com informações como ID, nome e descrição.
- **Sistema de recomendação**: Utiliza grafos para recomendar produtos com base nas interações entre eles.
- **Armazenamento de produtos**: Os produtos são armazenados usando um HashMap, onde o ID do produto é a chave.

## Estrutura do projeto

### Arquivos principais

- `src/main.rs`: O código principal onde a lógica de recomendação é implementada.
- `Cargo.toml`: Arquivo de configuração do projeto Rust.

### Dependências

O projeto é construído utilizando o Rust, e para rodá-lo, você precisa ter o Rust instalado em seu sistema.

## Como executar o projeto

### 1. Instalar o Rust

Para rodar este projeto, você precisa ter o Rust instalado. Se ainda não tiver, siga as instruções de instalação em [https://www.rust-lang.org/](https://www.rust-lang.org/).

### 2. Clonar o repositório

Para clonar o repositório para a sua máquina, execute o seguinte comando:

```bash
git clone https://github.com/GuilhermeToneli/megastore_recommender.git
