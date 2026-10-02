# Status de pesquisa — Backrooms Levels 0–99

Data-base: 2026-10-02.

## Primeira passagem
- 0–99: 100/100 níveis principais documentados em primeira passagem.

## Segunda passagem — níveis principais
- 0–3: concluído.
- 4–10: concluído.
- 11–15: concluído.
- 16–25: concluído.
- 26–30: concluído.
- 31–40: concluído e corrigido nesta rodada.
- 41–50: concluído.
- 51–60: concluído.
- 61–70: concluído.
- 71–80: concluído.
- 81–90: concluído.
- 91–99: concluído.

## Correções desta rodada
- Level 2 recebeu catálogo de entidades, bases, subníveis e conectividade ampliada.
- Level 3 recebeu estrutura atual, Base Gamma, entidades e inventário inicial de mídia.
- Level 12 recebeu as seções obrigatórias que faltavam.
- Level 37 foi reestruturado para o estado atual da página.
- Level 37.1 foi enriquecido e separado corretamente do nível-pai.
- Levels 38, 39 e 40 foram convertidos do formato antigo de “Metadados/Escopo” para o schema completo.
- Level 91 deixou de usar checkbox pendente dentro da auditoria.

## Sub-seções
O índice oficial consultado contém 58 sub-seções/localizações no recorte 0–99.
- 27 possuem arquivo dedicado nesta branch.
- 31 continuam pendentes.

Registro completo: docs/SUBLEVEL_REGISTRY.md

## Catálogo transversal

- `docs/ENTITY_BASE_CATALOG_0_99.md` agora reúne as seções de Entidades e Bases/Instalações dos 100 níveis principais e das 26 sub-seções que já possuem arquivo dedicado.
- O catálogo é derivado dos próprios Markdown e não substitui a ficha individual.

## Mídia
Catálogos existentes:
- 11–15.
- 16–25.
- 26–30.
- 31–35.
- 36–40.
- 41–50.
- 51–60.
- 61–70.
- 71–80.
- 81–90.
- 91–99.

Pendências:
- auditoria individual de licenças;
- substituição/referência de imagens não reutilizáveis;
- auditoria completa de subníveis.

## Conectividade
- Grafo inicial: presente.
- Segunda passagem 4–90: presente.
- Pendências: confiança/proveniência por aresta, arestas externas a 0–99, distinção definitiva entre entrada, saída, rota bidirecional e rota histórica, e métodos específicos de subníveis.

## Validação Rust
tools/backrooms_audit.rs exige dez seções estruturais em cada Markdown, além de detectar TODO/TBD/[ ].
O código está pronto, mas a execução global dos arquivos ainda não foi concluída nesta rodada. Não declarar 100% validado sem executar o binário.

## Próximas frentes

1. Criar os 32 arquivos de sub-seções faltantes.
2. Fechar a auditoria individual de mídia/licenças.
3. Fechar confiança/proveniência do grafo.
4. Reconciliar rotas externas a 0–99 e métodos de subníveis.
5. Executar a validação Rust no CI.
6. Fazer reconciliação final entre níveis, subníveis e conexões externas.

### Verificação de ferramenta

- `rustc` não está instalado no ambiente desta sessão; a compilação local dos validadores não pôde ser executada.
- GitHub Actions foi configurado para executar `cargo build --all-targets`, `backrooms-levels-catalog`, `backrooms-audit` e `backrooms-inventory`.
- Não há status de CI exposto para esta branch pela integração atual; portanto, a validação global permanece como pendência real até um job concluir com sucesso.
