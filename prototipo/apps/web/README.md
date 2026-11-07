# Frontend web

React + TypeScript + Vite + Tailwind CSS v4. A interface consulta a API para
pesquisa/detalhe de produtos e apresenta erros, catálogo vazio e indisponibilidade
sem preencher o catálogo com dados fictícios. O painel Git lista alterações
restritas ao projeto e permite criar commit local de até 200 arquivos após
seleção e confirmação.

## Organização do código

- `src/api/http.ts`: transporte JSON e interpretação comum de erros HTTP.
- `src/features/catalog/types.ts`: contratos da API e validação de respostas externas.
- `src/features/catalog/api.ts`: operações de saúde, categorias, busca e detalhe.
- `src/features/catalog/components/`: apresentação de resultados e detalhes.
- `src/features/git/api.ts` e `src/features/git/components/GitPanel.tsx`: contrato e interface do painel Git, isolados do catálogo.
- `src/features/library/LibraryWorkspace.tsx`: notas e importação explícita de arquivos de texto, busca local e pesquisa web opcional pelo Hermes.
- `src/App.tsx`: coordenação de estado e composição da página.

As dependências de frontend estão fixadas no `package.json` e no lockfile. Use
`npm ci` para instalações reproduzíveis.

## Requisitos
Node.js LTS e npm.

## Executar

    npm ci
    npm run dev

Validar tipos e compilação:

    npm run typecheck
    npm run build

Por padrão, as requisições `/api` são encaminhadas para `http://127.0.0.1:8080`.
Para alterar o destino do proxy, defina `CATALOG_API_URL` no ambiente do processo
Vite.

O banco começa vazio. Só são exibidos produtos com fonte aprovada e permissão
autorizada; ofertas também dependem de fonte aprovada e loja confirmada. O painel
Git cria commits locais dos arquivos escolhidos, mas não envia alterações ao
GitHub e não armazena credenciais.

## Biblioteca local

Use **Minha biblioteca** para criar, pesquisar e organizar notas locais ou
importar individualmente arquivos TXT, Markdown, CSV, JSON e HTML de até 48 KB.
Os dados ficam no SQLite local e não são sincronizados. A pesquisa Hermes é
opcional e envia somente a pergunta digitada para a web; veja
[`docs/catalogo/operacao/BIBLIOTECA_LOCAL_E_HERMES.md`](../../../docs/catalogo/operacao/BIBLIOTECA_LOCAL_E_HERMES.md)
para configurar Hermes no Windows e conhecer os limites atuais.
