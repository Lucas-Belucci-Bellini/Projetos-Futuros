# Registro de fontes de dados

## Propósito
Manter um inventário auditável das fontes usadas para descobrir lojas, produtos e ofertas. Cada fonte precisa ter método de acesso e uso pretendido avaliados antes de entrar na importação.

## Estados
- discovered: fonte encontrada, ainda não avaliada.
- under_review: termos, licença ou método de coleta em análise.
- approved: método e uso pretendido aprovados para o escopo registrado.
- restricted: uso limitado a links, consulta manual ou campos específicos.
- blocked: não coletar/importar pelo método avaliado.
- inactive: fonte descontinuada ou indisponível.

## Campos por fonte

| Campo | Descrição |
|---|---|
| source_id | Identificador interno estável |
| organization_name | Empresa ou fabricante |
| source_type | site, API, feed, planilha autorizada ou catálogo manual |
| official_url | Página oficial confirmada |
| product_listing_url | Página do catálogo, se houver |
| geographic_scope | Londrina, região, Paraná, nacional etc. |
| access_method | API, feed, arquivo recebido, consulta manual ou método permitido |
| permission_status | Um dos estados definidos acima |
| permission_evidence_url | Termos, licença ou autorização registrada |
| permitted_fields | Campos que podem ser armazenados/publicados |
| media_permission | Condição específica para imagens |
| expected_refresh | Frequência desejada, sujeita aos termos |
| last_checked_at | Última verificação |
| contact_channel | Contato comercial público, se necessário |
| notes | Restrições, exceções e dúvidas |

## Modelo para cada registro

### SOURCE_ID — Nome da empresa
- **Status:** discovered
- **Tipo:** site / API / feed / planilha autorizada / manual
- **URL oficial:**
- **URL do catálogo:**
- **Abrangência geográfica:**
- **Método de acesso:**
- **Evidência de permissão/termos:**
- **Campos permitidos:**
- **Imagens:** não avaliadas
- **Frequência de atualização:**
- **Última verificação:**
- **Próxima ação:**

## Regras de aprovação
1. Site público não significa autorização para copiar ou republicar todo o conteúdo.
2. Links de referência podem ser úteis; textos e imagens exigem avaliação própria.
3. Não contornar login, CAPTCHA, bloqueio, limite de requisições ou outro controle técnico.
4. Priorizar APIs, feeds e planilhas cedidas pela própria empresa.
5. Guardar evidência da autorização e o escopo permitido.
6. Se a permissão não estiver clara, limitar-se à pesquisa manual e aos links ou manter a fonte em revisão.
7. Revisar periodicamente as fontes e seus termos.

## Fontes iniciais para avaliar
- Balaroti: https://www.balaroti.com.br/
- Lojas Balaroti em Londrina: https://lojas.balaroti.com.br/parana/londrina

Esses links são pontos de partida para pesquisa, não autorização automática para extração em massa.


## Registro inicial
Veja [PESQUISA_FONTES_LOCAIS.md](PESQUISA_FONTES_LOCAIS.md) e os modelos `templates/fontes.csv` e `templates/lojas.csv`. Eles documentam candidatos e URLs oficiais para revisão. Os estados permanecem `discovered` e `pending` até a verificação.
