# Backrooms — Levels 0–99

Esta branch é um projeto isolado dentro do repositório `Projetos-Futuros`, dedicado à documentação detalhada dos Levels 0–99 e das sub-seções relacionadas identificadas na fonte.

## Branch

`backrooms-levels-0-99`

A branch foi criada a partir da `main` e mantém uma árvore específica para este projeto.

## Objetivo

Construir uma base documental que registre, para cada nível e sub-seção pesquisada:
- identidade e estado editorial;
- aparência e condições ambientais;
- estrutura, regiões e pontos de interesse;
- entidades;
- recursos;
- bases, comunidades e instalações;
- entradas e saídas;
- subníveis;
- mídia, autoria e licenças;
- canon, histórico e incertezas.

O objetivo é maximizar a cobertura **sem inventar dados**.

## Fontes e direitos

A fonte principal é a Backrooms Wiki. O projeto produz sínteses editoriais próprias e não copia artigos integralmente.

A licença da página não deve ser assumida como licença de todas as imagens incorporadas. Cada mídia precisa de autoria, origem e licença próprias. Consulte `ATTRIBUTION.md`.

## Stack

- **Markdown:** fonte documental primária.
- **Rust:** validação, parsing, indexação e futuras ferramentas.
- **HTML/CSS:** apresentação estática e visualização.
- **TypeScript:** somente se uma necessidade real não puder ser atendida de forma razoável por Rust, geração estática ou HTML/CSS.
- **JavaScript:** evitado diretamente.

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
├── tools/
│   ├── backrooms_audit.rs
│   └── backrooms_inventory.rs
├── docs/
│   ├── LEVEL_AUDIT_SCHEMA.md
│   ├── CANON_STATUS_MATRIX.md
│   ├── LEVEL_CONNECTIVITY_GRAPH.md
│   ├── SUBLEVEL_REGISTRY.md
│   ├── SECOND_PASS_GAPS.md
│   └── IMAGE_CATALOG_*.md
├── levels/
│   ├── level-00.md
│   ├── ...
│   └── level-99.md
└── web/
    ├── index.html
    └── styles.css
```

## Estado real em 2026-10-02

- **100/100** níveis principais 0–99 possuem arquivo.
- **58** sub-seções/localizações aparecem no snapshot do índice oficial.
- **26** dessas sub-seções já possuem arquivo dedicado nesta branch.
- **32** ainda aguardam expansão dedicada.
- O grafo de conectividade está documentado, mas ainda precisa de reconciliação final de confiança/proveniência e arestas externas.
- Catálogos de mídia existem, mas a auditoria individual de todas as licenças ainda não foi encerrada.
- Os validadores Rust existem e estão preparados para execução local/CI; a branch não deve ser considerada 100% validada até o job global terminar com sucesso.

## Regra editorial central

Uma informação desconhecida é melhor do que uma informação inventada.

Valores ausentes devem ser rotulados como:
- Não documentado;
- Não confirmado;
- Licença pendente;
- Versão histórica;
- Rota não consolidada.

Quando duas versões divergem, elas permanecem separadas por estado editorial.

## Continuidade

Qualquer outra pessoa ou IA deve conseguir retomar o trabalho pela combinação de:
`RESEARCH_STATUS.md` → `docs/SUBLEVEL_REGISTRY.md` → `docs/SECOND_PASS_GAPS.md` → `docs/LEVEL_CONNECTIVITY_GRAPH.md` → arquivos em `levels/`.

A prioridade é sempre fechar lacunas verificáveis antes de aumentar artificialmente a contagem de “concluído”.

## Comandos de validação

```text
cargo run --bin backrooms-levels-catalog
cargo run --bin backrooms-audit -- levels
cargo run --bin backrooms-inventory -- levels
```

A validação automática da branch executa esses três binários.

