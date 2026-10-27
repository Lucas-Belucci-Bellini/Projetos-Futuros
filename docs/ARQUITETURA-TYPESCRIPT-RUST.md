# Arquitetura Técnica — Site de Promoções e Orçamentos

**Status:** proposta de arquitetura; ainda não implementada nem validada em execução.  
**Branch de planejamento:** `planejamento-stack-typescript-rust-site`  
**Produto:** site brasileiro de comparação de preços, promoções e montagem de orçamentos de PCs, notebooks e componentes.

## 1. Decisões principais

1. **Frontend:** TypeScript com Next.js (React), usando App Router e componentes acessíveis.
2. **Backend principal:** Rust com Axum sobre Tokio, separando API HTTP, domínio e persistência.
3. **Fallback:** se uma necessidade específica não tiver solução Rust madura e compatível, avaliar TypeScript/Node.js para aquele serviço isolado. Não trocar toda a arquitetura por causa de um recurso pontual.
4. **Banco:** PostgreSQL; SQLx para acesso assíncrono e consultas verificáveis em compilação quando aplicável.
5. **Contratos:** API documentada em OpenAPI; gerar/validar tipos TypeScript a partir do contrato em vez de manter modelos duplicados manualmente.
6. **Coleta de preços:** serviço/worker separado da API pública; preferir APIs oficiais e fontes autorizadas.
7. **Implantação:** serviços desacoplados, variáveis de ambiente e segredos fora do repositório.

Essas são escolhas propostas para o plano. Não significam que os serviços já existam.

## 2. Visão de componentes

```text
Navegador
   |
   v
Frontend Next.js + TypeScript
   | páginas, SEO, formulários, orçamento interativo
   v
API HTTP Rust + Axum + Tokio
   | autenticação futura, validação, regras de negócio
   +---------------------+
   |                     |
   v                     v
PostgreSQL             Fila/worker de tarefas
   |                     |
   |                     v
   |                Coletores autorizados
   |                     |
   +<---- preços, estoque, histórico, eventos
```

O frontend não deve acessar diretamente credenciais privilegiadas do banco nem executar coleta de preços em requisições de usuário.

## 3. Frontend TypeScript

### Stack proposta
- Next.js + React + TypeScript.
- App Router para páginas e layouts.
- CSS Modules ou Tailwind CSS: escolher uma única abordagem após um pequeno protótipo de acessibilidade e manutenção.
- Biblioteca de formulários e validação TypeScript a escolher após definir formulários reais; não adicionar dependências sem necessidade.
- Cliente HTTP tipado gerado a partir do OpenAPI.
- Testes unitários para utilitários e componentes críticos; testes de navegador para fluxos principais.

### Páginas previstas
- Página inicial e busca.
- Promoções e filtros.
- Produto: preço atual, histórico, lojas, frete, garantia e alternativas.
- Comparador de produtos.
- Assistente de orçamento.
- Configuração detalhada e compatibilidade.
- Perfis por finalidade/curso.
- Alertas e lista de acompanhamento (fase posterior).
- Transparência, metodologia e política de afiliados.

### Regras
- A interface diferencia preço atualizado, preço antigo e preço histórico.
- Exibe origem e data/hora da última atualização.
- Nunca promete FPS ou compatibilidade sem base documentada.
- Layout responsivo, teclado navegável, contraste adequado e estados de carregamento/erro/vazio.

## 4. Backend Rust

### Stack proposta
- Rust estável.
- Axum para rotas HTTP, extração de dados e middleware.
- Tokio para I/O assíncrono.
- Serde para serialização.
- SQLx com PostgreSQL para persistência.
- Tracing para logs estruturados.
- Validação explícita de entrada e erros HTTP consistentes.
- OpenAPI por crate compatível a selecionar no protótipo.

### Organização de módulos

```text
backend/
  crates/
    api/              # rotas HTTP e inicialização
    domain/           # regras puras de produto e orçamento
    application/      # casos de uso e orquestração
    persistence/      # PostgreSQL e repositórios
    integrations/     # adaptadores para lojas e serviços externos
    jobs/             # tarefas assíncronas e processamento de eventos
  tests/
```

A estrutura acima é uma proposta lógica; pode ser simplificada no MVP para evitar abstração prematura.

### Domínios iniciais
- Catálogo e especificações de produtos.
- Lojas e ofertas.
- Preço atual e histórico.
- Disponibilidade/estoque.
- Orçamentos e itens.
- Regras de compatibilidade.
- Alertas e eventos (fase posterior).
- Origem, confiança e horário de coleta de cada dado.

## 5. Fallback TypeScript para backend

O backend Rust é a escolha padrão. Usar TypeScript no servidor somente quando uma capacidade necessária não tiver biblioteca Rust suficientemente madura, mantida, segura ou compatível com a implantação.

Processo de decisão:
1. Descrever a necessidade e critérios de aceite.
2. Pesquisar crates Rust e verificar manutenção, licença, segurança, compatibilidade e testes.
3. Fazer um protótipo pequeno.
4. Se Rust não atender, criar serviço TypeScript isolado com contrato OpenAPI.
5. Não duplicar regras de negócio entre serviços.
6. Documentar a exceção em ADR, incluindo custo operacional e plano de revisão.

O fallback pode ser Next.js/Node.js para um serviço específico, mas não deve ser usado só por familiaridade se Axum atender ao requisito.

## 6. Contratos e comunicação

- API versionada sob `/api/v1`.
- JSON para operações comuns.
- OpenAPI como contrato verificável.
- Tipos do frontend gerados do contrato.
- Paginação, filtros e ordenação explícitos.
- Erros padronizados com código estável, mensagem segura e identificador de requisição.
- Timeouts, limites de requisição e rate limiting nas rotas públicas.
- Idempotência para operações de escrita que possam ser repetidas.
- Nenhum segredo ou credencial de loja no bundle do frontend.

## 7. Dados e persistência

Entidades iniciais:
- `products`: produto canônico e categoria.
- `product_specs`: especificações normalizadas.
- `stores`: lojas e identificadores.
- `offers`: oferta, URL, preço, frete, vendedor e condição.
- `price_observations`: histórico imutável de observações.
- `stock_observations`: histórico de disponibilidade.
- `builds` e `build_items`: orçamento e componentes escolhidos.
- `compatibility_rules`: regras e proveniência.
- `data_sources`: origem, termos, limite de uso e método de coleta.

O esquema final deve ser detalhado em documento próprio antes da implementação. Armazenar timestamps em UTC e apresentar horário local ao usuário. Valores monetários devem usar representação decimal apropriada, nunca ponto flutuante binário para cálculo financeiro.

## 8. Segurança, privacidade e conformidade

- Validar toda entrada no servidor.
- Segredos apenas em ambiente/secret manager.
- Privilégio mínimo para usuários do banco.
- TLS em trânsito e backups testáveis.
- Logs sem senhas, tokens ou dados pessoais desnecessários.
- Proteção contra abuso, rate limits e limites para consultas caras.
- Coleta conforme termos de uso, APIs oficiais, robots.txt e limites publicados; não contornar CAPTCHA, login ou bloqueios.
- Identificar afiliados e patrocínios; comissão não altera secretamente o ranking.
- Coletar o mínimo de dados pessoais e definir retenção, exclusão e política de privacidade.

## 9. Observabilidade e qualidade

- Logs estruturados com `request_id`.
- Métricas de latência, erros, disponibilidade da API e falhas de coleta.
- Health/readiness checks sem expor segredos.
- Testes unitários de domínio, integração com PostgreSQL e testes de contrato API/frontend.
- CI: format, lint, typecheck, testes, auditoria de dependências e build.
- Não declarar desempenho, segurança ou cobertura sem executar e registrar os testes correspondentes.

## 10. Estratégia de implantação

### Desenvolvimento
- Docker Compose para PostgreSQL e serviços locais necessários.
- `.env.example` com nomes de variáveis, sem valores secretos.
- Migrações versionadas.
- Dados de demonstração claramente marcados como fictícios.

### Produção
- Frontend e API podem ser implantados separadamente.
- Banco não exposto publicamente.
- Backups automatizados e teste periódico de restauração.
- Migrações com plano de rollback.
- Workers escalados independentemente da API.
- Alertas para falhas de atualização de preços e dados vencidos.

## 11. Critérios para aceitar a arquitetura

- Frontend consome API usando tipos gerados/verificados.
- API funciona sem depender de uma loja específica.
- Falha de um coletor não derruba navegação e consulta do catálogo.
- Cada preço tem origem e timestamp.
- Regras de compatibilidade são testáveis isoladamente.
- É possível substituir uma integração externa sem reescrever o domínio.
- O MVP não exige autenticação para navegação básica.
- Nenhum segredo é versionado.
