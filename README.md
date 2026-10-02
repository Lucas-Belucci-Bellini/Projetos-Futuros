# Backrooms — Levels 0–99

Esta branch é um projeto isolado dentro do repositório `Projetos-Futuros`, criado especificamente para documentar os Levels 0 a 99 da Backrooms Wiki.

## Branch

`backrooms-levels-0-99`

Ela foi criada a partir da `main`, mas a árvore deste projeto foi reconstruída do zero para evitar carregar os outros projetos do repositório.

## Objetivo

Criar uma base extremamente detalhada, organizada e pesquisável dos Levels 0–99, priorizando:

- Markdown para documentação;
- Rust para validação, automação e futuras ferramentas;
- HTML e CSS para visualização;
- TypeScript somente se existir alguma necessidade que não seja razoável atender com Rust.

## Estrutura

```text
.
├── README.md
├── ATTRIBUTION.md
├── ARCHITECTURE.md
├── RESEARCH_STATUS.md
├── Cargo.toml
├── src/
│   └── main.rs
├── levels/
│   ├── level-00.md
│   ├── ...
│   └── level-99.md
└── web/
    ├── index.html
    └── styles.css
```

## Princípios

1. Máximo de detalhes possíveis.
2. Nenhuma invenção de lore.
3. Nenhuma cópia integral dos artigos.
4. Sempre registrar a fonte.
5. Tratar páginas em reescrita como conteúdo instável.
6. Manter a branch independente da `main`.
7. Preferir Rust a JavaScript.
8. Manter documentação suficiente para qualquer outra IA ou desenvolvedor continuar o trabalho.

## Estado atual

- Estrutura 0–99: concluída.
- 100 arquivos Markdown: concluídos.
- Metadados e URLs oficiais: concluídos.
- Classificação editorial básica: concluída.
- Ferramenta Rust inicial: concluída.
- Interface HTML/CSS inicial: concluída.
- Expansão profundamente detalhada de cada página: em andamento.
