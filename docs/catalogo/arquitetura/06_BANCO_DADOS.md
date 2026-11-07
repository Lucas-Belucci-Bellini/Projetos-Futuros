# 6. Banco de dados e persistência

## Diretriz
PostgreSQL é o alvo de produção. O SQLite existente continua útil para ferramentas e testes atuais até existir uma migração planejada e validada. Não substituir o banco em uso por uma mudança abrupta.

## Entidades principais
- **products:** identidade canônica e descrição curta própria.
- **product_variants:** variações comercialmente relevantes, como dimensão, cor, acabamento ou embalagem.
- **product_identifiers:** GTIN/EAN, SKU de origem ou código do fabricante com tipo, valor e fonte.
- **categories:** taxonomia hierárquica.
- **stores:** empresa e filial; manter estado de verificação.
- **sources:** origem, URL, tipo, método permitido, condições e evidências.
- **store_sources:** vínculo entre loja e fonte, com escopo e estado de aprovação.
- **source_products:** identificação do produto no catálogo original.
- **offers:** relação entre produto/variante e loja, preço, moeda, unidade, modalidade, disponibilidade declarada, URL e data de observação.
- **import_batches:** lote, origem, datas, estado, contagens e referência ao relatório.
- **import_errors:** erros e avisos associados ao lote.
- **audit_events:** alterações administrativas importantes, com dados mínimos necessários.

## Relações e integridade
- Uma loja pode ter várias fontes; uma fonte pode abranger mais de uma filial somente quando isso estiver comprovado.
- Um produto canônico pode ter várias variantes e várias ofertas.
- Oferta não deve existir sem produto, loja e fonte rastreáveis.
- IDs externos e identificadores globais devem ter restrições únicas adequadas ao seu escopo.
- Chaves estrangeiras e checks protegem integridade; regras de deduplicação ambígua exigem revisão, não exclusão automática.

## Preços e histórico
- Armazenar preço como decimal exato ou representação equivalente segura, nunca float binário.
- Registrar moeda, unidade de venda, quantidade por embalagem e modalidade (à vista, parcelado, por unidade etc.).
- Preservar histórico conforme política de retenção e permissões da fonte.
- Separar preço desconhecido de preço zero.
- Registrar `observed_at` e, quando aplicável, `valid_until`.
- Não assumir estoque disponível apenas porque uma página de produto existe.

## Índices planejados
- Nome normalizado e identificadores de produto.
- Categoria e marca quando presentes.
- Relações entre oferta, produto, loja e fonte.
- Data de observação e estado de publicação.
- Índice textual apropriado após medir consultas reais.

## Migração
1. Definir schema PostgreSQL e versão mínima suportada.
2. Criar migrations versionadas e reversão/recuperação documentada.
3. Criar fixtures sintéticas e testes de integridade.
4. Migrar amostra controlada do SQLite.
5. Comparar contagens, chaves e relatórios antes/depois.
6. Só então decidir se o SQLite será mantido para importações ou descontinuado.

Nenhuma migração de produção deve ser executada sem backup e plano de recuperação.
