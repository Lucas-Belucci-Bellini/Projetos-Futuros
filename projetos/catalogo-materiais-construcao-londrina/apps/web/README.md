# Frontend web

React + TypeScript + Vite + Tailwind CSS v4. A primeira tela é um scaffold visual; a busca ainda não consulta produtos porque o endpoint de catálogo será implementado depois.

## Requisitos
Node.js LTS e npm.

## Executar

    npm install
    npm run dev

Validar tipos e compilação:

    npm run typecheck
    npm run build

Por padrão, as requisições /api são encaminhadas para http://127.0.0.1:8080. Para alterar o destino do proxy, defina CATALOG_API_URL no ambiente do processo Vite.

Não há catálogo real embutido. Os cartões de categoria são elementos de apresentação e estão identificados como planejados.
