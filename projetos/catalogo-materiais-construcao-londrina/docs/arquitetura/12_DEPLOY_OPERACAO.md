# 12. Deploy e operação

## Ambientes
- Local para desenvolvimento.
- CI para verificação automatizada.
- Staging para testar integração, migrations e configuração.
- Produção para tráfego real e dados aprovados.

## Componentes de produção
- Frontend estático hospedado em serviço compatível com Vite.
- API Rust em serviço que suporte processo persistente e configuração de porta/interface.
- PostgreSQL gerenciado ou administrado com backup e monitoramento.
- Execução Python agendada separadamente, somente após autorização e validação do pipeline.
- Domínio HTTPS e CORS alinhados à origem da web.

A plataforma específica de hospedagem permanece uma decisão pendente. Não assumir que a API Rust funciona como função serverless sem validar limites de runtime, inicialização e conexão com banco.

## Configuração
- URL pública da API configurada por ambiente.
- URL de banco e credenciais fora do código.
- Bind address e porta configuráveis para o provedor.
- Migrations executadas por processo controlado, nunca implicitamente em cada requisição.
- Configuração de CORS por ambiente.
- Segredos diferentes entre local, staging e produção.

## Observabilidade
- Logs estruturados com request ID e duração.
- Métricas de latência, erro, volume de busca e falhas de importação.
- Health check de processo e readiness de dependências separados.
- Alertas para indisponibilidade, crescimento de erro, ofertas envelhecidas e falhas de pipeline.
- Não incluir dados sensíveis nos logs.

## Backup e recuperação
- Definir frequência, retenção e responsável.
- Testar restauração em ambiente isolado.
- Documentar perda de dados aceitável e tempo de recuperação pretendido.
- Manter backups protegidos e fora do mesmo domínio de falha quando possível.
- Não declarar estratégia pronta até realizar um teste de restauração.

## Lançamento gradual
1. Deploy de staging com dados sintéticos.
2. Teste de integração e migrações.
3. Piloto com dados reais autorizados e limitados.
4. Revisão de qualidade e monitoramento.
5. Lançamento público com cobertura e limitações explicadas.
