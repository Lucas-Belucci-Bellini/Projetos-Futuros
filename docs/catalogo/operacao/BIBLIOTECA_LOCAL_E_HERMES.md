# Biblioteca local e Hermes

## Protótipo implementado

A opção **Minha biblioteca** acrescenta uma biblioteca local ao catálogo existente. É possível criar, editar, excluir e pesquisar notas; também é possível importar individualmente arquivos TXT, Markdown, CSV, JSON ou HTML de até 48 KB e 12.000 caracteres. O arquivo original não é movido nem alterado. O texto salvo fica em um banco SQLite separado no perfil do usuário do Windows (`%LOCALAPPDATA%\CatalogoMateriais\biblioteca.sqlite3`), fora do repositório do projeto. Defina `LIBRARY_DB_PATH` para escolher outro local.

A busca local cobre título, conteúdo e etiquetas. Ela não percorre o disco nem indexa pastas em segundo plano. Importe apenas arquivos que você escolher. O protótipo aceita texto com limite de 12.000 caracteres por registro.

## Hermes no Windows

O Hermes é opcional. A instalação e configuração não são executadas pelo app:

1. Leia o [guia oficial de instalação](https://hermes-agent.nousresearch.com/docs/getting-started/installation).
2. Execute `prototipo/scripts/install-hermes-windows.ps1` em um PowerShell iniciado por você. O script oficial é baixado para um arquivo temporário, exibido no Bloco de Notas para revisão e só executado depois que você digitar `EXECUTAR`.
3. Depois da instalação, execute `hermes setup` conforme solicitado pelo assistente oficial.
4. Volte à biblioteca e confirme que o status do Hermes aparece como instalado.
5. Escreva uma pergunta e acione **Pesquisar na web** para cada consulta que deseja enviar.

O app usa o comando oficialmente documentado `hermes chat -q`, limitado ao conjunto de ferramentas `web` e a três turnos. A pergunta é enviada ao Hermes e aos provedores de pesquisa/web configurados; nenhum conteúdo da biblioteca ou arquivo local é incluído. A resposta é transitória e deve ter as fontes conferidas. Sem Hermes, a biblioteca local continua utilizável.

## Execução e testes

Na pasta `prototipo/apps/api`, execute `cargo test` e depois `cargo run`. Em outra janela, na pasta `prototipo/apps/web`, execute `npm run dev`. Abra o endereço do Vite indicado no terminal e escolha **Minha biblioteca**.

## Sincronização GitHub

A sincronização do banco pessoal com um repositório GitHub privado ainda não está implementada neste protótipo. Por enquanto, os registros permanecem neste computador; os commits do painel Git existente se referem ao código do projeto, não aos dados da biblioteca. Uma sincronização futura deve ser opt-in, verificar a autenticação com o GitHub CLI, criar o repositório como privado e exigir confirmação antes de enviar os dados. O app não deve armazenar tokens.

Veja o [plano completo do organizador de planilhas](../plano/PLANO_ORGANIZADOR_PLANILHAS.md) para as fases de varredura, análise de Excel, sincronização privada, recuperação e exportação de PDF/imagem.
