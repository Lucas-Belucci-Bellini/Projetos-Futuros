# Plano — organizador de planilhas local com GitHub e Hermes

## Visão do produto

Transformar o protótipo em um aplicativo Windows que ajude cada pessoa a localizar, entender, versionar e compartilhar planilhas que já possui. O app deve ser configurável no primeiro uso, local-first e explícito sobre quais arquivos serão lidos ou enviados.

Cada usuário conecta uma conta GitHub e seleciona um repositório privado existente ou autoriza o app a criar um repositório privado. A biblioteca funciona sem conexão durante o uso diário; alterações aguardam em uma fila local até a próxima sincronização.

Hermes pode pesquisar na web quando solicitado e ajudar a descrever ou classificar uma planilha. O app continua responsável pela varredura, banco local, commits, push, verificação do repositório e restauração. Hermes não recebe acesso irrestrito ao terminal nem credenciais GitHub.

## Estado do protótipo atual

Já existe uma primeira biblioteca local na opção **Minha biblioteca**:

- notas e arquivos de texto selecionados pelo usuário;
- busca em título, conteúdo e etiquetas;
- dados em SQLite separado, no perfil do usuário;
- pesquisa web opcional pelo Hermes, enviando somente a pergunta digitada.

Ainda não existem varredura de planilhas, leitor de Excel, integração GitHub da biblioteca, sincronização, PDFs ou capturas de áreas de planilhas. O painel Git existente faz commits do código do projeto e não deve ser confundido com o futuro backup dos dados pessoais.

## Configuração inicial

O assistente de primeira execução deve apresentar etapas curtas, permitir revisão e permitir alterar cada opção depois:

1. **Perfil e armazenamento:** explicar que a biblioteca ficará no computador e que o backup GitHub exige uma conta e um repositório privado.
2. **GitHub:** autenticar pelo GitHub CLI (`gh auth login`) ou fluxo oficial equivalente; selecionar um repositório privado existente ou confirmar a criação de um novo repositório privado. Consultar a visibilidade remota antes de enviar qualquer arquivo.
3. **Pastas e unidades:** selecionar pastas ou iniciar, por ação explícita, a busca nas unidades locais fixas. Não incluir discos de rede, dispositivos removíveis ou pastas de nuvem sem escolha expressa.
4. **Tipos e limites:** escolher extensões, tamanho máximo por arquivo, pastas excluídas, política de duplicatas e frequência de sincronização.
5. **Hermes:** opcional. Mostrar se o CLI está instalado, abrir as instruções oficiais e explicar quais perguntas são enviadas a serviços externos.
6. **Teste:** apresentar um resumo do escopo e quantidades estimadas, testar gravação local e confirmar o destino privado antes de iniciar uma importação.

O repositório precisa estar conectado antes de ativar o backup remoto. Se a rede ou GitHub estiver indisponível depois da configuração, o app continua localmente e mostra claramente os itens pendentes de sincronização.

## Descoberta e organização de planilhas

### Varredura consentida

- Nunca percorrer o computador silenciosamente durante a instalação ou a inicialização.
- O botão **Localizar planilhas** inicia uma varredura somente depois de mostrar as unidades/pastas incluídas, exclusões, tipos de arquivo e limites configurados.
- Cobrir `.xlsx`, `.xls`, `.xlsm`, `.xlsb`, `.ods` e, por decisão do usuário, `.csv`.
- Não executar macros, fórmulas, links externos, scripts ou conteúdo embutido. Arquivos criptografados, corrompidos, inacessíveis ou acima do limite ficam identificados como não processados, com motivo.
- Ignorar por padrão Windows, arquivos de sistema, caches, pastas de instalação, repositórios Git e arquivos temporários. Disponibilizar exclusões editáveis, incluindo pastas sensíveis e de clientes.
- Criar uma prévia antes de importar: caminho, nome, extensão, tamanho, data, duplicatas prováveis e destino. O usuário confirma o lote; cancelamento não altera os originais.
- Revarreduras são incrementais, usando caminho normalizado, identificador do arquivo quando disponível e hash de conteúdo. Renomeações, cópias e versões precisam ser apresentadas sem apagar dados.

### Organização de cada planilha

O repositório privado poderá usar uma estrutura previsível, com identificador estável para evitar colisões de nomes:

```text
workbooks/
  <workbook-id>/
    original/<nome-original>.xlsx
    metadata.json
    exports/
index/
  documents.jsonl
```

O app conserva o caminho original no banco local para abrir o arquivo de origem, quando ele ainda existir. A cópia no repositório é uma versão de backup e nunca substitui ou move silenciosamente o original. O usuário escolhe se uma importação cria cópia ou apenas registra o caminho.

## Modelo de dados e pesquisa

SQLite é o índice operacional local, não uma planilha de controle editável pelo usuário. O schema deve relacionar:

- planilha: ID estável, nome, formato, tamanho, hash, datas e estado de leitura;
- local de origem: caminho normalizado, unidade e existência atual;
- conteúdo identificado: nomes das abas, cabeçalhos, dimensões, fórmulas detectadas e texto pesquisável dentro dos limites;
- organização: etiquetas, categoria sugerida, observações e confirmação do usuário;
- backup: repositório, caminho relativo, hash remoto, último commit/push e pendências;
- exportações: planilha, aba/intervalo, formato, opções e data.

O banco SQLite local pode ser reconstruído a partir dos metadados versionados. Não se deve sincronizar o arquivo SQLite ativo (incluindo WAL) como se fosse um banco multiusuário: ele é binário, não tem merge seguro e pode estar em gravação. Para GitHub, persistir arquivos de metadados JSON/JSONL estáveis junto aos originais; versionar alterações do schema por migrações SQL.

A busca começa por nome, caminho, etiqueta, aba e cabeçalhos; depois pode indexar texto de células com limites e controles para conteúdo sensível. Nunca enviar conteúdo de planilhas ao Hermes sem uma ação separada, resumo do que será compartilhado e consentimento explícito.

## GitHub, commits e sincronização

### Responsabilidades

- O app usa Git e GitHub CLI instalados/autenticados pelo usuário; tokens não são gravados no SQLite, nos metadados, logs ou prompts do Hermes.
- Verificar proprietário, URL remota, branch e visibilidade **privada** antes de cada primeira publicação. Se o repositório não for privado ou mudar de destino, bloquear o push e explicar como corrigir.
- Criar commits em lotes confirmados, não um commit por arquivo durante uma varredura grande. Mostrar arquivos, número, tamanho e destino antes do primeiro push; depois, permitir política manual ou automática configurada pelo usuário.
- Hermes pode sugerir etiquetas e metadados. O app valida e aplica as alterações; uma integração Hermes futura recebe apenas ferramentas delimitadas (por exemplo, sugerir classificação ou solicitar sincronização), sem acesso a comandos Git arbitrários.
- Exibir estados distintos: local, aguardando envio, enviando, sincronizado, conflito e erro. Push com falha nunca deve aparecer como sucesso.
- Em conflito, manter as duas versões, mostrar um fluxo de resolução e não usar `reset`, `clean` ou force-push automático.
- Verificar o resultado remoto por hash e oferecer restauração para uma pasta escolhida. Manter cópia local e exportação manual.

### Limites que precisam de decisão antes do lançamento

Um repositório privado reduz exposição, mas **não garante sozinho** que os dados nunca serão perdidos ou vistos por terceiros: conta comprometida, exclusão da conta, colaboradores, falhas de configuração e histórico Git continuam relevantes. Planilhas podem conter dados pessoais, financeiros ou de clientes; GitHub recebe as cópias enviadas. O onboarding deve explicar isso e oferecer exclusões, restauração e, antes de armazenar dados sensíveis, uma decisão sobre criptografia local antes do commit.

Git não é ideal para arquivos grandes e armazena versões antigas no histórico. Definir limites por arquivo/lote, aviso para arquivos grandes, estratégia para Git LFS ou armazenamento alternativo e política de retenção. Nunca apagar histórico remoto automaticamente para “economizar espaço”.

## PDF e captura de área para clientes

- Permitir selecionar uma planilha, aba e intervalo explícito, visualizar a saída e então gerar PDF ou imagem.
- PDF deve permitir orientação, tamanho do papel, escala, margens, cabeçalhos e rodapés.
- A captura deve representar somente a área selecionada, não uma captura indiscriminada da tela ou de outras janelas.
- Mostrar prévia e alertar sobre abas ocultas, comentários, fórmulas, metadados e informações fora do intervalo. Não enviar ou publicar automaticamente.
- Exportar para local escolhido pelo usuário; preservar o arquivo original. Oferecer redigir/cobrir células sensíveis antes de exportar.
- Investigar renderização com LibreOffice em modo controlado e exportação de páginas para imagem. Não depender de automação COM de uma sessão do Excel aberta. Se o renderizador não estiver instalado ou falhar, reportar claramente e não gerar arquivo incompleto.

## Fases de entrega e critérios de aceite

### Fase 1 — Conectar armazenamento privado

Criar fluxo de autenticação, selecionar/criar repositório privado, gravar um arquivo de teste sem dados pessoais, validar sua visibilidade e apagá-lo apenas após mostrar o resultado.

**Aceite:** repositório público ou destino inesperado é bloqueado; nenhum token aparece em disco ou logs; falha de rede é exibida como pendência.

### Fase 2 — Varredura e índice local

Implementar seleção de pastas/unidades, exclusões, prévia, limites, hash, detecção de duplicatas, migrações SQLite e busca por nome/aba/cabeçalho.

**Aceite:** sem confirmação não há leitura persistente nem upload; os arquivos de origem permanecem inalterados; pasta excluída e formatos não permitidos não são importados; revarrer sem mudanças não cria duplicatas.

### Fase 3 — Importação e sincronização Git

Copiar os arquivos confirmados para IDs estáveis, gerar metadados, agrupar commits e enviar com verificação remota, fila offline, conflitos e recuperação.

**Aceite:** o usuário vê exatamente quais itens serão enviados; o hash remoto corresponde ao arquivo local; queda de rede não perde a fila; sincronização repetida é idempotente; não há force-push automático.

### Fase 4 — Hermes restrito

Adicionar sugestões de etiquetas/resumos e pesquisa web. Qualquer envio de conteúdo de planilha exige prévia e consentimento independentes da consulta web.

**Aceite:** Hermes não executa comandos Git arbitrários, não recebe credenciais e não acessa arquivos fora do conjunto explicitamente autorizado.

### Fase 5 — Exportações

Gerar PDF e imagem de uma aba/intervalo selecionado com prévia, redaction e mensagens de erro claras.

**Aceite:** exportação contém somente a seleção confirmada, abre em leitores comuns e não altera nem envia a planilha original.

### Fase 6 — Backup e restauração

Adicionar verificação de integridade, histórico de sincronização, restauração para pasta escolhida, exportação do índice e teste de recuperação documentado.

**Aceite:** recuperar uma planilha e seus metadados a partir de uma instalação limpa, sem depender do banco SQLite antigo.

## Fora do primeiro MVP

- Varrer ou enviar todos os arquivos automaticamente sem prévia.
- Compartilhar planilhas com Hermes por padrão.
- Compartilhar links com clientes ou enviar e-mail automaticamente.
- Tratar GitHub privado como cópia infalível ou único backup.
- Sincronização simultânea de escrita no mesmo arquivo SQLite.
- Executar macros, fórmulas externas ou documentos baixados.
