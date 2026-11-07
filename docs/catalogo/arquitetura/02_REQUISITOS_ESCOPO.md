# 2. Requisitos e escopo

## Requisitos funcionais

### Catálogo público
- Pesquisar por nome, marca, SKU/EAN ou identificador disponível.
- Navegar por categorias e subcategorias.
- Filtrar por loja, marca, faixa de preço, unidade e disponibilidade declarada.
- Ordenar por relevância, nome e preço quando houver ofertas comparáveis.
- Abrir página de produto com especificações, variantes e ofertas.
- Exibir fonte, data da observação e condições relevantes de cada oferta.
- Abrir a página original da loja para confirmação.
- Mostrar estados claros de carregamento, erro, resultado vazio e informação desatualizada.
- Permitir reportar correção ou link quebrado, sujeito a moderação.

### Operação interna
- Registrar fontes e lojas em estado pendente.
- Aprovar manualmente a permissão e o método de coleta.
- Importar lote em modo dry-run.
- Visualizar erros, avisos, duplicatas candidatas e contagens.
- Revisar e aprovar lote antes de publicar.
- Desativar fonte, oferta ou produto sem apagar o histórico necessário à auditoria.
- Gerar relatórios de cobertura e qualidade.

## Requisitos não funcionais
- Interface responsiva e navegável por teclado.
- API versionada, documentada e com paginação limitada.
- Consultas ao banco parametrizadas.
- Logs sem segredos e sem dados pessoais desnecessários.
- Backups e restauração testados antes de produção.
- Migrações versionadas e reproduzíveis.
- CI executando testes e verificações de formatação.
- Erros com mensagens úteis sem expor detalhes internos.
- Observabilidade básica: saúde, latência, falhas e resultado dos lotes.

## Prioridades
**P0 — Fundação:** documentação, scaffold, contratos, validação de CSV, CI confiável.

**P1 — Piloto real:** validar ao menos uma fonte permitida, importar um lote pequeno, revisar qualidade.

**P2 — Catálogo utilizável:** banco PostgreSQL, busca real, paginação, categorias e ofertas.

**P3 — Administração:** revisão de fontes e lotes, autenticação e trilha de auditoria.

**P4 — Escala:** novas fontes, histórico de preços, monitoramento, otimização e meta de 50.000.

## Critérios de aceite
Cada requisito precisa de teste ou procedimento de verificação. “A tela existe” não significa “a funcionalidade está concluída”; por exemplo, busca só é pronta quando consulta dados reais pela API e trata paginação, erros e lista vazia.
