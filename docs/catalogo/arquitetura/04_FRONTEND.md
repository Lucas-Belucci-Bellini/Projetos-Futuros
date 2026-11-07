# 4. Arquitetura do frontend

## Stack
- React + TypeScript + Vite.
- Tailwind CSS v4 para utilitários de estilo.
- Biblioteca de componentes acessíveis somente se houver necessidade demonstrada; evitar dependências redundantes.
- Testes planejados: Vitest e Testing Library.
- Comunicação exclusivamente via API REST, exceto assets estáticos.

## Rotas planejadas
- `/`: início, pesquisa principal e categorias populares.
- `/buscar?q=`: resultados, filtros, ordenação e paginação.
- `/produto/:slugOuId`: detalhes, variantes e ofertas.
- `/categoria/:slug`: navegação por taxonomia.
- `/loja/:slug`: perfil da loja e ofertas conhecidas.
- `/sobre`: metodologia, cobertura e limitações.
- `/fontes-e-atualizacao`: explicação de proveniência e atualização.
- `/reportar-correcao`: formulário moderado.
- Área administrativa separada e protegida em fase posterior.

## Componentes previstos
- Header, busca global, navegação de categorias e rodapé.
- ProductCard e ProductList.
- SearchFilters e SortControl.
- Pagination.
- OfferTable/OfferCard com loja, preço, unidade, condições, data e link original.
- FreshnessLabel e SourceAttribution.
- LoadingState, ErrorState, EmptyState e Offline/APIUnavailable.
- Formulários acessíveis com validação e mensagens de erro.

## Estado e URL
- A consulta e os filtros compartilháveis devem ficar representados na URL quando fizer sentido.
- Evitar armazenar em estado global informações deriváveis da URL ou da resposta da API.
- Cancelar requisições obsoletas durante mudanças rápidas de busca.
- Tratar timeout, resposta inválida, erro HTTP e indisponibilidade.
- Não mostrar dados fictícios em produção. Fixtures de desenvolvimento devem ser identificadas.

## Diretrizes visuais
- Layout mobile-first e responsivo.
- Hierarquia visual simples, com foco em pesquisa e comparação.
- Preço sempre acompanhado da unidade comercial e contexto da oferta.
- Contraste adequado; foco de teclado visível.
- Não usar logotipos de lojas como se houvesse parceria.
- Não imitar a identidade visual de uma loja específica.

## Critérios de pronto
- TypeScript estrito sem erros.
- Build de produção executado na CI.
- Busca, filtros e paginação testados contra API real ou mocks controlados.
- Estados de erro, vazio, carregamento e informação desatualizada cobertos.
- Verificação de teclado, leitor de tela e telas pequenas.
