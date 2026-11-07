# 15. Decisões técnicas e perguntas em aberto

Este documento separa decisões tomadas de decisões que ainda precisam de evidência. Uma proposta não deve ser tratada como implementação concluída.

## Decisões já tomadas
| Tema | Decisão | Motivo |
|---|---|---|
| Frontend | React + TypeScript + Vite | Ecossistema amplo e tipagem estática |
| CSS | Tailwind CSS v4 | Utilitários consistentes e prototipação rápida |
| Backend | Rust + Axum | API tipada e desempenho |
| Automação | Python | Ecossistema de dados e scripts existente |
| Banco alvo | PostgreSQL | Integridade relacional e crescimento |
| API | REST versionada em `/api/v1` | Contrato simples para web e outros clientes |
| Dados | Separar produto, variante, oferta, loja e fonte | Evita confundir identidade e preço por loja |
| Coleta | Somente métodos permitidos | Reduz risco legal, operacional e de bloqueio |

## Decisões ainda pendentes
1. **PostgreSQL driver:** escolher biblioteca Rust, compatibilidade assíncrona, pooling e manutenção.
2. **Migrations:** escolher ferramenta e política de rollback/recuperação.
3. **Busca:** começar com pesquisa textual PostgreSQL ou serviço externo? Decidir após medir volume e qualidade necessária.
4. **Hospedagem:** selecionar frontend, API e banco conforme custo, limites de runtime, região e operação.
5. **Autenticação administrativa:** escolher provedor e papéis antes de publicar ações de aprovação.
6. **Histórico de preços:** definir retenção e política de exclusão compatível com direitos e custos.
7. **Imagens:** definir se serão links externos, imagens próprias ou assets licenciados; não baixar/republicar por padrão.
8. **Analytics:** decidir se é necessário, com minimização e transparência.
9. **Frequência de atualização:** negociar ou documentar por fonte, sem presumir permissão.
10. **Critério de produto único:** formalizar quando uma diferença constitui variante separada.
11. **Escopo geográfico:** definir se entram apenas lojas físicas em Londrina ou também lojas online que entregam na cidade.
12. **Unidades comparáveis:** definir conversões seguras por categoria (m², metro, litro, kg, unidade, pacote etc.).

## Registro de decisão
Para cada decisão relevante, registrar:
- ID e data;
- contexto e problema;
- opções consideradas;
- decisão e justificativa;
- consequências e riscos;
- responsável pela revisão;
- condição que justificaria reabrir a decisão.

## Não fazer ainda
- Não criar scraping massivo antes da aprovação de fonte.
- Não colocar preços fictícios para preencher telas.
- Não publicar painel administrativo sem autenticação/autorização.
- Não migrar dados de produção sem backup e reconciliação.
- Não declarar build ou testes verdes sem resultado verificável.
