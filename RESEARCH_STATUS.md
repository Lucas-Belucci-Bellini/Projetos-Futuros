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
- 15 possuem arquivo dedicado nesta branch.
- 43 continuam pendentes.

Registro completo: docs/SUBLEVEL_REGISTRY.md

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
1. Criar os 43 arquivos de sub-seções faltantes.
2. Fazer auditoria individual de mídia.
3. Criar catálogos globais de entidades.
4. Criar catálogos globais de bases e instalações.
5. Fechar confiança/proveniência do grafo.
6. Executar a validação Rust global.
7. Fazer reconciliação final entre níveis, subníveis e conexões externas.
