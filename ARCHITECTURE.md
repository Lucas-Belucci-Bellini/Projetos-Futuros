# Arquitetura do projeto

## 1. Objetivo técnico

Construir uma base documental dos Levels 0–99 que possa futuramente alimentar:

- site estático;
- catálogo pesquisável;
- API;
- gerador de páginas;
- banco de dados;
- ferramenta CLI;
- visualizador offline;
- integração com projetos de jogos.

## 2. Linguagens permitidas

### Preferência 1 — Rust

Usar Rust para:
- validação;
- parsing;
- geração;
- indexação;
- ferramentas CLI;
- transformação de Markdown;
- futura API/backend;
- processamento pesado.

### Preferência 2 — HTML/CSS

Usar HTML e CSS para:
- apresentação;
- documentação navegável;
- protótipos estáticos.

### Preferência 3 — TypeScript

Só introduzir TypeScript quando uma função de navegador realmente exigir lógica cliente que não seja prática em Rust/WebAssembly ou geração estática.

### JavaScript

Evitar código JavaScript escrito diretamente.

## 3. Fonte de verdade

Cada arquivo em `levels/` é a unidade documental primária de um Level.

## 4. Convenção de nomes

`levels/level-NN.md`

NN sempre possui dois dígitos entre 00 e 99.

## 5. Metadados mínimos

Todo arquivo deve conter:
- número;
- título;
- estado editorial;
- página oficial;
- índice;
- data-base.

## 6. Evolução

Uma futura migração para dados estruturados poderá usar Rust para extrair os metadados Markdown e gerar JSON, SQL ou HTML automaticamente.
