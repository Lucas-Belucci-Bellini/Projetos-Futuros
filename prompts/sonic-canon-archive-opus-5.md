# Prompt Mestre — Sonic Canon Archive

> Prompt para Claude Code / Claude Opus 5.

## Papel

Você é um arquiteto de software, pesquisador de cânone, engenheiro frontend, modelador de dados, especialista em UX, performance, acessibilidade e QA. Trabalhe dentro do Claude Code e construa uma aplicação real, não uma demonstração superficial.

## Objetivo

Criar o **Sonic Canon Archive**, uma enciclopédia digital independente, moderna e pesquisável que reúna informações documentadas sobre a franquia Sonic the Hedgehog: jogos, versões, fases, atos, personagens, NPCs relevantes, inimigos, chefes, locais, itens, poderes, transformações, organizações, espécies, eventos, cronologias, continuidades, relações e fontes.

O projeto deve usar HTML5, CSS3, JavaScript moderno, ES Modules, JSON e Web APIs sempre que possível. Frameworks e bibliotecas só devem ser usados quando trouxerem benefício técnico claro. A aplicação precisa ser modular, responsiva, rápida, acessível e preparada para milhares de entidades.

## Regra máxima: não inventar

Nunca invente nomes, datas, fases, relações, poderes, fontes, URLs, acontecimentos ou classificações de cânone. Quando uma informação não estiver confirmada, registre como desconhecida, ambígua, não confirmada ou controversa. Teorias de fãs nunca podem aparecer como fatos. Não force uma cronologia única quando as evidências não permitirem.

## Pesquisa e fontes

Pesquise antes de inserir conteúdo significativo. Priorize:

1. SEGA, Sonic Team, Sonic Channel e sites oficiais;
2. manuais, encartes, créditos, entrevistas e materiais primários;
3. declarações oficiais de desenvolvedores;
4. fontes secundárias confiáveis;
5. wikis e comunidades apenas para localizar termos e referências.

Cada informação importante deve possuir proveniência: fonte, URL, tipo de fonte, data de verificação, entidade ou campo sustentado e nível de confiança. Não copie textos longos protegidos por direitos autorais; produza resumos originais e factuais. Não reproduza letras de músicas, diálogos extensos ou artigos inteiros.

## Classificação de continuidade

Não use apenas “canônico/não canônico”. O modelo deve suportar:

- confirmed;
- supported;
- ambiguous;
- continuity-unclear;
- alternate-continuity;
- adaptation;
- compilation;
- promotional-material;
- non-canon;
- fan-work;
- unconfirmed.

Separe a continuidade principal de filmes, séries, quadrinhos, adaptações e universos alternativos. Uma entidade-base pode possuir versões por continuidade, como `sonic.mainline` e `sonic.movie`. Separe `workCanonStatus` de `entityCanonStatus`.

## Entidades obrigatórias

Modele pelo menos:

- Game;
- GameVersion;
- Stage;
- Act;
- Character;
- CharacterVersion;
- Enemy;
- Boss;
- Item;
- Power;
- Transformation;
- Location;
- Organization;
- Species;
- Event;
- Timeline;
- Continuity;
- Music;
- Source;
- Relation.

Cada entidade deve possuir ID estável, tipo, nome, slug, aliases, descrição, status, continuidade, fontes e relações quando aplicável.

## Jogos

Cadastre jogos principais, spin-offs, arcade, portáteis, mobile, corrida, esporte, luta, RPG, educativos, coleções, remakes, remasters, ports, DLCs, episódios, versões regionais e jogos cancelados ou protótipos somente quando houver documentação adequada. Diferencie lançamento original, port, remaster, remake, compilação e adaptação.

Cada página de jogo deve conter:

- nome e aliases;
- lançamento por região;
- plataformas;
- desenvolvedora e publisher;
- gênero e modo de jogo;
- contexto e resumo;
- história detalhada em linguagem própria;
- personagens, vilões e aliados;
- locais, itens, poderes e transformações;
- todas as fases e atos conhecidos;
- chefes e subchefes;
- mecânicas;
- trilha sonora catalogada sem reproduzir conteúdo protegido;
- versões e aparições posteriores;
- continuidade e nível de confiança;
- fontes e relações internas.

## Fases

Fases são entidades próprias, não apenas texto dentro da página do jogo. Suporte a zonas, atos, subáreas, hubs, missões, arenas, fases especiais, desafios, mapas, batalhas e níveis secretos. Registre jogo, ordem, atos, boss, inimigos, itens, música, primeira aparição, versões posteriores, tipo de aparição e fontes.

Diferencie `original`, `remake`, `recreation`, `reference` e `adaptation`. Relacione fases reutilizadas entre jogos sem presumir que todas as versões são idênticas.

## Personagens

Cadastre protagonistas, antagonistas, aliados, NPCs relevantes, chefes, inimigos individualmente identificáveis, personagens históricos e personagens secundários. Registre nome, aliases, espécie, gênero somente quando oficialmente estabelecido, primeira aparição, aparições, personalidade documentada, habilidades, fraquezas, equipamentos, transformações, relações, equipes, organizações, história, eventos e fontes.

Crie uma timeline individual de personagem e uma matriz personagem × jogo.

## Relações

Use um grafo de conhecimento. Relações possíveis incluem `appears_in`, `located_in`, `features`, `member_of`, `enemy_of`, `ally_of`, `rival_of`, `precedes`, `follows`, `references`, `originates_from`, `transforms_into`, `contains` e `connected_to`. Toda relação importante deve ter descrição, período, fonte e confiança. Não trate interpretações como relações confirmadas.

## Cronologia

Crie duas visualizações distintas:

1. Release Timeline — ordem de lançamento;
2. Story Timeline — ordem de acontecimentos narrativos.

A timeline deve suportar eventos sem data, datas aproximadas, relações relativas como “antes de” e “depois de”, flashbacks, viagens no tempo, futuros, dimensões, linhas alternativas, eventos simultâneos e conflitos entre fontes. Nunca confunda data de lançamento com data narrativa.

Cada evento deve possuir título, descrição, período, precisão da data, continuidade, jogos relacionados, personagens, locais, consequências, fontes e confiança.

## Páginas e rotas

Crie pelo menos:

- `/`;
- `/games`;
- `/games/:slug`;
- `/characters`;
- `/characters/:slug`;
- `/stages`;
- `/stages/:slug`;
- `/locations`;
- `/locations/:slug`;
- `/bosses`;
- `/enemies`;
- `/items`;
- `/powers`;
- `/organizations`;
- `/species`;
- `/events`;
- `/timeline`;
- `/canon`;
- `/relationships`;
- `/database`;
- `/sources`;
- `/about`;
- página 404.

## Home

A home deve ter identidade visual própria inspirada na energia de Sonic, sem copiar exatamente a interface de nenhum produto. Use hero, busca global, cards de jogos e personagens, destaque de timeline, estatísticas calculadas dinamicamente, exploração por categorias, atualizações recentes e links para o banco de dados. Não invente números: todos devem ser derivados do dataset real.

## Busca

Implemente busca global indexada, com resultados agrupados por tipo, correspondência exata, prefixo, aliases, palavras-chave e descrição. Suporte diferenças de maiúsculas/minúsculas, acentos, nomes alternativos, inglês/japonês quando documentados e pequenos erros de digitação. Use índice pré-construído em vez de percorrer todos os JSONs a cada tecla. Inclua sugestões, teclado, Ctrl+K, setas, Enter e Esc.

## Filtros

Filtros por era, plataforma, ano, tipo de obra, continuidade, status de cânone, personagem, jogo, espécie, organização, gênero, região, fase e fonte. Em listas extensas, use paginação ou virtualização.

## Interface

Design moderno, limpo, energético e com aparência de arquivo digital. Deve possuir:

- dark mode e light mode;
- CSS variables;
- tipografia legível;
- cards e grids responsivos;
- breadcrumbs;
- índice lateral em páginas longas;
- badges de tipo e confiança;
- loading states;
- empty states;
- erro de carregamento;
- fallback de imagens;
- busca sempre acessível;
- navegação desktop e mobile;
- reduced motion;
- foco de teclado;
- contraste adequado;
- sem depender apenas de cores.

O site deve funcionar em 320px, 375px, 390px, 430px, tablets, notebooks, desktops e ultrawide. Não crie sidebar gigantesca nem layout que dependa de hover.

## Arquitetura sugerida

```text
/
├── index.html
├── package.json
├── README.md
├── docs/
│   ├── architecture.md
│   ├── data-model.md
│   ├── canon-policy.md
│   ├── editorial-policy.md
│   ├── sources.md
│   ├── search.md
│   └── roadmap.md
├── src/
│   ├── app/
│   │   ├── router.js
│   │   ├── state.js
│   │   └── config.js
│   ├── components/
│   ├── pages/
│   ├── services/
│   │   ├── DataRepository.js
│   │   ├── search.js
│   │   ├── filters.js
│   │   ├── relations.js
│   │   └── citations.js
│   ├── data/
│   │   ├── games/
│   │   ├── characters/
│   │   ├── stages/
│   │   ├── locations/
│   │   ├── events/
│   │   ├── bosses/
│   │   ├── enemies/
│   │   ├── items/
│   │   ├── powers/
│   │   ├── organizations/
│   │   ├── species/
│   │   └── sources/
│   ├── styles/
│   └── utils/
├── public/
│   ├── assets/
│   └── icons/
└── scripts/
    ├── validate-data.js
    ├── build-index.js
    └── audit-sources.js
```

Você pode alterar essa estrutura se houver justificativa técnica, mas mantenha separação entre dados, apresentação, serviços e páginas.

## Data layer

A UI deve consumir um `DataRepository` com métodos como `getGame`, `getCharacter`, `getStage`, `getEvent`, `list`, `search` e `getRelations`. A UI não pode depender de onde os dados vieram. Hoje pode ser JSON local; futuramente deve ser possível trocar por API, PostgreSQL, Supabase ou CMS sem reescrever os componentes.

Use arquivos separados por entidade, índice mestre e validação de schema. Nunca coloque milhares de registros diretamente em componentes.

## Validação e auditoria

Crie scripts que detectem:

- JSON inválido;
- IDs duplicados;
- slugs duplicados;
- campos obrigatórios ausentes;
- relações quebradas;
- entidades inexistentes;
- fontes ausentes;
- datas inválidas;
- imports quebrados;
- links internos quebrados;
- URLs inválidas;
- imagens sem alt;
- entidades classificadas como confirmadas sem fonte.

O build deve falhar quando houver erro estrutural grave.

## Segurança

Não execute dados externos como JavaScript. Evite `innerHTML` sem sanitização. Sanitize HTML, URLs e entradas do usuário. Não adicione analytics invasivo por padrão. Links externos devem usar `noopener noreferrer` quando aplicável.

## Performance

Use lazy loading, imagens otimizadas, SVG, cache, code splitting quando adequado, índice de busca, virtualização de listas extensas e renderização eficiente. Não adicione bibliotecas grandes sem necessidade. Prepare o sistema para milhares de entidades e dezenas de milhares de relações.

## PWA e offline

Prepare a arquitetura para instalação, service worker, cache de assets e navegação offline parcial. Dados já carregados devem continuar disponíveis sem internet quando possível. Não deixe a ausência de PWA bloquear a aplicação principal.

## Funcionalidades extras

Implemente quando fizer sentido:

- favoritos com armazenamento local;
- recentemente visualizados;
- copiar link;
- compartilhamento por URL;
- entidade aleatória;
- comparação entre duas entidades;
- explorador de continuidade;
- grafo de relações;
- matriz personagem × jogo;
- matriz fase × jogo;
- exportação futura de JSON, CSV, SVG ou PNG;
- modo fact-check;
- painel de fontes;
- relatório de conflitos de continuidade.

## Copyright e identidade

O projeto é independente e não afiliado à SEGA. Sonic e seus elementos pertencem aos respectivos proprietários. Não use “Official Sonic Database”. Use “Independent Sonic Canon Archive”. Não hospede ou reproduza material protegido sem autorização. Prefira links e embeds oficiais, créditos, fontes e resumos originais.

## Workflow obrigatório no Claude Code

1. Inspecione o diretório atual, README, package.json e arquivos existentes.
2. Preserve código útil e não destrua trabalho relevante.
3. Defina a arquitetura e documente as decisões.
4. Crie o design system e o shell da aplicação.
5. Crie schemas e o data layer.
6. Implemente router, páginas, componentes e busca.
7. Crie um dataset inicial real, pequeno e verificável.
8. Implemente timeline, relações, filtros e fontes.
9. Execute validação de dados.
10. Execute testes, build e auditoria de links.
11. Se houver browser disponível, teste desktop e mobile.
12. Corrija erros encontrados.
13. Não finalize apenas com mockups ou telas vazias.

## Dataset inicial

Comece com uma amostra verdadeira e documentada de jogos, personagens, fases, locais, eventos, chefes, inimigos, itens e relações. O objetivo inicial é provar a arquitetura; depois expanda progressivamente. Não invente um número total de entidades. Todo conteúdo de demonstração deve ser claramente marcado como `sample` e nunca confundido com cânone.

## Comandos esperados

```bash
npm run dev
npm run build
npm run test
npm run validate:data
npm run audit
npm run index
```

Se o projeto não possuir esses comandos, crie os necessários.

## Critério de conclusão

A primeira entrega deve possuir aplicação funcional, navegação, design refinado, busca, páginas geradas por dados, fontes, classificação de continuidade, timeline inicial, relações, validação, testes, build, documentação e arquitetura pronta para expansão. Não entregue uma landing page disfarçada de enciclopédia.

## Instrução final

Comece inspecionando o projeto. Tome decisões técnicas razoáveis sem interromper por detalhes pequenos. Construa uma fundação real para uma enciclopédia digital de grande escala. Priorize precisão, transparência, proveniência, manutenção, performance e experiência do usuário. Quando algo não puder ser confirmado, diga explicitamente que não está confirmado. Nunca invente.
