# Checklist operacional do lote piloto

Este documento define como sair dos CSVs vazios para um primeiro lote real sem fabricar dados, sem aprovar fontes automaticamente e sem publicar informação cuja reutilização não foi autorizada.

## 1. Antes de importar qualquer dado

- [ ] Confirmar por escrito quem fornece os dados e quem pode autorizar a reutilização.
- [ ] Registrar a evidência da autorização em `templates/fontes.csv` e verificar se ela cobre nome, descrição, código, preço, estoque, imagem e link — cada campo pode ter uma permissão diferente.
- [ ] Confirmar a abrangência geográfica: Londrina, região, estoque por filial ou somente venda online.
- [ ] Ler os termos de uso, política de privacidade, regras de API/feed e limites de acesso aplicáveis.
- [ ] Não contornar login, CAPTCHA, bloqueios, limites técnicos ou controles de acesso.
- [ ] Não copiar imagens ou descrições completas sem permissão correspondente.
- [ ] Registrar a data em que cada informação foi observada e a URL de origem.
- [ ] Usar dados de teste apenas em testes automatizados; não misturá-los ao catálogo real.

## 2. Preparar banco e registrar candidatos

Execute a partir da raiz do repositório. Os comandos registram candidatos como pendentes; eles não aprovam fontes nem confirmam lojas.

```bash
python projetos/catalogo-materiais-construcao-londrina/scripts/registrar_fontes_csv.py projetos/catalogo-materiais-construcao-londrina/templates/fontes.csv --db catalogo-materiais.sqlite3
python projetos/catalogo-materiais-construcao-londrina/scripts/registrar_lojas_csv.py projetos/catalogo-materiais-construcao-londrina/templates/lojas.csv --db catalogo-materiais.sqlite3
python projetos/catalogo-materiais-construcao-londrina/scripts/relatorio_catalogo.py --db catalogo-materiais.sqlite3 --output relatorio-inicial.json
```

Confira o JSON retornado por cada comando e o relatório. Se um comando falhar, interrompa o fluxo e corrija a causa antes de continuar.

## 3. Aprovar uma fonte manualmente

A aprovação deve ocorrer somente após revisão humana documentada. Não use um CSV para aprovar a fonte.

1. Conferir a identidade do fornecedor e a URL oficial.
2. Verificar a evidência de autorização e os campos permitidos.
3. Confirmar se o método de coleta é permitido e quais limites devem ser respeitados.
4. Documentar responsável, data, escopo, restrições e próxima data de revisão.
5. Atualizar o registro no banco por uma operação administrativa revisada, com backup prévio.
6. Reexecutar o relatório e verificar o status salvo.

Se URL oficial, página de listagem, método de acesso, evidência de permissão, campos autorizados ou permissão de mídia mudar, a fonte deve voltar para revisão. O importador de fontes implementa essa proteção para mudanças nos campos críticos.

## 4. Primeiro lote

- [ ] Começar com um lote pequeno e representativo, por exemplo 50–200 registros autorizados.
- [ ] Guardar o arquivo de origem e sua data, sem alterar o original.
- [ ] Rodar o validador de CSV.
- [ ] Executar o importador em `--dry-run`.
- [ ] Revisar as rejeições e uma amostra de registros aceitos.
- [ ] Importar somente depois de resolver os erros bloqueantes.
- [ ] Gerar relatório depois da importação e comparar as contagens.
- [ ] Registrar problemas conhecidos e decisões de normalização.

Exemplo de comandos — substitua os identificadores e caminhos por dados reais autorizados:

```bash
python projetos/catalogo-materiais-construcao-londrina/scripts/validar_catalogo_csv.py caminho/para/produtos.csv --report validacao-produtos.json
python projetos/catalogo-materiais-construcao-londrina/scripts/importar_catalogo_csv.py caminho/para/produtos.csv --db catalogo-materiais.sqlite3 --source-id ID_DA_FONTE --dry-run
# Somente depois de revisar o dry-run:
python projetos/catalogo-materiais-construcao-londrina/scripts/importar_catalogo_csv.py caminho/para/produtos.csv --db catalogo-materiais.sqlite3 --source-id ID_DA_FONTE
python projetos/catalogo-materiais-construcao-londrina/scripts/relatorio_catalogo.py --db catalogo-materiais.sqlite3 --output relatorio-pos-importacao.json
```

Ofertas e preços são importados separadamente pelo script de ofertas. Nunca trate preço sem data de observação como preço atual.

## 5. Critérios mínimos para aceitar o piloto

- 100% dos registros publicados têm uma fonte rastreável e uso permitido documentado.
- 0 registros inventados ou preenchidos com valores presumidos.
- 0 fontes não aprovadas usadas para importação de produtos/ofertas.
- 0 lojas publicadas como confirmadas sem verificação.
- Duplicatas conhecidas e campos ausentes quantificados.
- Preços associados a data de observação, moeda e condições quando aplicável.
- Imagens publicadas somente quando a permissão de uso estiver coberta.
- Relatório pré e pós-importação guardados para auditoria.
- Procedimento de remoção/correção definido para solicitações de fornecedores.

## 6. Escala para 50.000 itens

Não multiplique um lote pequeno para aparentar progresso. Avance por fonte e categoria, medindo produtos únicos, cobertura por loja, duplicatas, campos completos, idade dos preços, falhas de links e permissões. A meta só conta registros reais que passam pelos critérios de qualidade e permissão definidos no projeto.
