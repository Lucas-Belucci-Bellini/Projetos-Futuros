# Resumo operacional do catálogo — 2026-10-09

## Estado atual dos arquivos de trabalho

- **Meta de produtos canônicos:** 50.000.
- **Produtos candidatos no template:** 25 registros em três lotes de descoberta.
- **Produtos verificados para a meta pública:** 0 até que os critérios de revisão sejam aplicados.
- **Ofertas registradas:** 0; não usar preços de catálogo sem confirmação atual das condições.
- **Empresas/unidades cadastradas no CSV:** 24.
- **Fontes cadastradas no CSV:** 16.
- **Status comercial:** os estabelecimentos continuam como `pending`; nenhum foi declarado parceiro.

## Lotes

- [Lote 001 — Leroy Merlin](LOTE_PILOTO_001.md): 5 candidatos.
- [Lote 002 — Romani e Econolux](LOTE_PILOTO_002.md): 10 candidatos.
- [Lote 003 — ferramentas do Depósito Eldorado](LOTE_PILOTO_003.md): 10 candidatos, com identificadores internos provisórios.

## Próximas ações de qualidade

1. Obter URLs individuais e SKUs oficiais para o lote 003.
2. Rodar o validador CSV e confirmar que todos os `source_id` existem no registro de fontes.
3. Revisar campos e identificadores de cada produto, incluindo unidade e embalagem.
4. Conferir duplicatas entre lotes e corrigir a categoria principal.
5. Validar termos e escopo de uso de cada fonte.
6. Coletar observações atuais de oferta em separado, com URL, data, preço, condições, unidade e disponibilidade.
7. Só então atualizar o estado dos produtos para verificado e publicar comparações.

## Limitação importante

Os números acima representam o estado dos arquivos de trabalho consultados, não uma auditoria automatizada completa nem uma confirmação de que o pipeline inteiro foi executado. Nenhum teste ou workflow é declarado aprovado por causa destas alterações.
