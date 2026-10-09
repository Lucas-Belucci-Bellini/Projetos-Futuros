# 10. Segurança e privacidade

## Princípios
- Menor privilégio em contas, tokens, banco e pipeline.
- Segredos somente em variáveis de ambiente ou gerenciador de segredos.
- Nenhum segredo em Git, logs, relatórios públicos ou mensagens de erro.
- Separar permissões de leitura pública e operações administrativas.
- Validar entrada no frontend e, obrigatoriamente, no backend.
- Consultas SQL parametrizadas.
- Atualizações administrativas com trilha de auditoria.

## API
- Limitar paginação, tamanho de requisição, timeout e custo de filtros.
- CORS restrito aos domínios esperados em produção.
- Rate limiting quando a exposição pública e o risco justificarem.
- Headers de segurança e HTTPS no ambiente publicado.
- Não revelar stack trace, query SQL, caminhos de arquivo ou configuração.
- Não confiar em validação feita somente no navegador.
- Evitar endpoints administrativos até haver autenticação e autorização implementadas.

## Administração
Antes de permitir edição ou publicação:
- escolher provedor/método de autenticação;
- definir papéis e permissões;
- proteger ações de aprovação e remoção;
- registrar autor, data e justificativa das decisões relevantes;
- proteger contra CSRF quando aplicável ao modelo de sessão;
- limitar tentativas e monitorar falhas de autenticação.

## Privacidade
O catálogo não precisa de dados pessoais para pesquisar produtos. Minimizar coleta de analytics e formulários. Definir aviso de privacidade, finalidade, retenção e canal de contato antes de lançar formulários públicos ou ferramentas de análise.

## Segurança da cadeia de fornecimento
- Revisar dependências e atualizar vulnerabilidades.
- Fixar versões por lockfile quando a stack estiver pronta para builds reproduzíveis.
- Não executar arquivos de origem desconhecida durante importação.
- Validar tamanho e tipo de arquivos importados.
- Isolar credenciais de produção da CI de pull requests não confiáveis.

## Resposta a incidentes
Documentar como revogar credenciais, suspender coleta, retirar dados publicados, restaurar backup e comunicar falhas relevantes. Testar o procedimento antes de depender dele.
