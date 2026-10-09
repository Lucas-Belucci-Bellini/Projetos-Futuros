# Roadmap e backlog priorizado

## Princípio
A meta de 50.000 produtos é uma meta de catálogo, não uma autorização para preencher a base com registros inventados, duplicados ou sem fonte. Crescer por etapas verificáveis.

## Fase 0 — Fundamentos
- [x] Documentação inicial do projeto.
- [x] Separação entre produto, oferta, loja e fonte.
- [x] Meta de 50.000 e taxonomia inicial.
- [x] Arquitetura lógica de ingestão.
- [x] Contrato inicial de CSV.
- [x] Registro de fontes e requisitos do MVP.
- [x] Plano de testes.
- [ ] Confirmar tecnologia e ambiente de desenvolvimento.
- [ ] Registrar decisões técnicas com justificativas.

**Saída:** regras claras para implementar sem misturar dados inventados e reais.

## Fase 1 — Fontes e lote piloto
- [x] Pesquisar um primeiro conjunto de fontes oficiais e lojas candidatas.
- [x] Criar CSVs iniciais de fontes e lojas, todos sem aprovação/confirmacão automática.
- [ ] Ampliar e validar a lista de lojas de Londrina por especialidade.
- [ ] Verificar sites, endereços e atendimento geográfico.
- [ ] Avaliar termos, licenças, feeds e parcerias.
- [x] Registrar estrutura de inventário e importador de fontes.
- [x] Criar importador de lojas com estado pendente.
- [ ] Obter pequeno lote com uso permitido.
- [ ] Importar lote piloto localmente.
- [ ] Revisar amostra manualmente.
- [ ] Medir duplicatas, campos ausentes e erros.

**Critério de saída:** ao menos uma fonte aprovada e um lote real reproduzível com relatório de qualidade.

## Fase 2 — Banco e importador
- [x] Definir schema SQLite inicial para fontes, lojas, produtos, ofertas e lotes.
- [ ] Avaliar se SQLite atende a primeira implantação ou se o projeto precisa começar diretamente com PostgreSQL.
- [ ] Implementar fontes, lojas, produtos, variantes, ofertas e lotes.
- [x] Implementar importação CSV idempotente inicial para SQLite.
- [x] Adicionar workflow de testes Python no GitHub Actions.
- [ ] Executar testes em CI e corrigir falhas encontradas.
- [x] Criar relatório de contagens e qualidade do banco SQLite.
- [ ] Implementar validação e normalização.
- [ ] Criar fila de revisão de conflitos.
- [ ] Criar testes automatizados.
- [ ] Documentar execução local e recuperação de falhas.

## Fase 3 — Catálogo navegável
- [ ] Busca e paginação.
- [ ] Filtros por categoria, loja, marca e preço disponível.
- [ ] Página de produto e lista de ofertas.
- [ ] Página de loja com dados verificados.
- [ ] Exibição de data e origem.
- [ ] Formulário de correção.
- [ ] Testes de acessibilidade e celular.

## Fase 4 — Crescimento controlado
- [ ] Adicionar fontes aprovadas gradualmente.
- [ ] Automatizar importações apenas quando permitido.
- [ ] Monitorar links quebrados e ofertas expiradas.
- [ ] Resolver duplicatas.
- [ ] Medir cobertura por categoria, loja e região.
- [ ] Criar relatório de progresso para 50.000.

## Fase 5 — Escala e operação
- [ ] Testar carga representativa.
- [ ] Definir frequência por fonte.
- [ ] Criar alertas de falha.
- [ ] Definir retenção do histórico de preços.
- [ ] Planejar backup e restauração.
- [ ] Negociar integrações diretas com lojas quando útil.

## Definição de pronto para os 50.000
Contar apenas produtos únicos com nome/categoria válidos, fonte rastreável, uso permitido dos campos publicados, sem duplicatas conhecidas e com variantes comerciais preservadas. Oferta é contada separadamente: um produto vendido em cinco lojas continua sendo um produto e pode ter cinco ofertas.

## Próxima tarefa recomendada
Implementar o registro de fontes, importador CSV e relatório de validação antes de construir uma interface completa ou prometer um volume ainda não coletado.
