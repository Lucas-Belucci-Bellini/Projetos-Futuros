# Registro de decisões técnicas

Este arquivo registra decisões provisórias e os motivos para que mudanças futuras sejam explícitas.

## DEC-001 — Produto, oferta e fonte são entidades separadas
**Estado:** aceita.

**Decisão:** manter o produto observado na origem separado da oferta comercial e da loja.

**Motivo:** o mesmo produto pode aparecer em diversas lojas, com preço, unidade de venda, condição e disponibilidade diferentes. Uma oferta não deve ser confundida com uma identidade de produto.

## DEC-002 — SQLite para o primeiro protótipo de ingestão
**Estado:** provisória.

**Decisão:** usar SQLite para desenvolver e testar schema/importador localmente.

**Motivo:** permite testar integridade, transações e importação sem exigir infraestrutura de servidor.

**Limite:** isso não determina a tecnologia final de produção. PostgreSQL poderá ser mais adequado se houver concorrência, operação multiusuário, importações frequentes ou implantação remota.

## DEC-003 — CSV como primeiro contrato de entrada
**Estado:** aceita para o piloto.

**Decisão:** oferecer CSV UTF-8 como formato inicial de importação.

**Motivo:** é simples de inspecionar, produzir e trocar com lojas. APIs e feeds poderão ser adicionados por adaptadores sem mudar o modelo de dados central.

## DEC-004 — Não contar linhas como produtos únicos
**Estado:** aceita.

**Decisão:** medir separadamente produtos observados, produtos canônicos e ofertas.

**Motivo:** múltiplas lojas e múltiplas variantes podem gerar muitas linhas sem representar a mesma quantidade de produtos únicos.

## DEC-005 — Não importar fonte não aprovada
**Estado:** aceita.

**Decisão:** o importador exige que a fonte esteja registrada e aprovada no banco.

**Motivo:** a origem, o método de acesso e os campos reutilizáveis precisam ser verificados antes da publicação.

## DEC-006 — Sem preços ou disponibilidade inventados
**Estado:** aceita.

**Decisão:** dados ausentes permanecem nulos ou desconhecidos; toda oferta publicada mantém URL e data de observação.

**Motivo:** uma comparação de preços incorreta prejudica o usuário e a credibilidade do catálogo.

## Decisões pendentes
- Banco de produção e hospedagem.
- Framework de interface e estratégia de API.
- Frequência de atualização por fonte.
- Política de retenção do histórico de preços.
- Processo de correção e remoção de dados solicitado por lojas.
- Métrica operacional para definir quando uma oferta está desatualizada.
