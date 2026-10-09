# Resumo operacional do catálogo — 2026-10-09

## Estado atual dos arquivos de trabalho

- **Meta de produtos canônicos:** 50.000.
- **Produtos candidatos no template:** 76 registros em nove lotes de descoberta.
- **Produtos verificados para a meta pública:** 0 até que os critérios de revisão sejam aplicados.
- **Ofertas registradas:** 6 observações de preço publicadas em páginas individuais da Romani, ainda sujeitas à confirmação de condições, unidade atendente, estoque e entrega; nenhuma parceria comercial foi presumida.
- **Empresas/unidades cadastradas no CSV:** 31.
- **Fontes cadastradas no CSV:** 21.
- **Status comercial:** os estabelecimentos continuam como `pending`; nenhum foi declarado parceiro.

## Lotes

- [Lote 001 — Leroy Merlin](LOTE_PILOTO_001.md): 5 candidatos.
- [Lote 002 — Romani e Econolux](LOTE_PILOTO_002.md): 10 candidatos.
- [Lote 003 — ferramentas do Depósito Eldorado](LOTE_PILOTO_003.md): 10 candidatos, com identificadores internos provisórios.
- [Lote 004 — hidráulica e elétrica](LOTE_PILOTO_004.md): 10 candidatos pendentes.
- [Lote 005 — cobertura, madeira, hidráulica e pisos](LOTE_PILOTO_005.md): 18 candidatos pendentes.
- [Lote 009 — cimento, tintas e rejuntes especiais](LOTE_PILOTO_009.md): 8 candidatos pendentes.
- [Lote 007 — pisos, revestimentos e ofertas observadas](LOTE_PILOTO_007.md): 5 candidatos novos e 6 observações de preço em páginas individuais.
- [Lote 006 — impermeabilização, limpeza e ferramentas](LOTE_PILOTO_006.md): 6 candidatos pendentes.

## Próximas ações de qualidade

1. Obter URLs individuais e SKUs oficiais para os lotes 003, 004, 005 e 006.
2. Rodar o validador CSV e confirmar que todos os `source_id` existem no registro de fontes.
3. Revisar campos e identificadores de cada produto, incluindo unidade e embalagem.
4. Conferir duplicatas entre lotes e corrigir a categoria principal.
5. Validar termos e escopo de uso de cada fonte.
6. Coletar observações atuais de oferta em separado, com URL, data, preço, condições, unidade e disponibilidade.
7. Validar a nova fonte Madeireira Luzitano e confirmar as duas unidades antes de eventual publicação.
8. Só então atualizar o estado dos produtos para verificado e publicar comparações.

## Limitação importante

Os números acima representam o estado dos arquivos de trabalho consultados, não uma auditoria automatizada completa nem uma confirmação de que o pipeline inteiro foi executado. Nenhum teste ou workflow é declarado aprovado por causa destas alterações.
