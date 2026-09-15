# Visão e Requisitos

## Objetivo

Criar um sistema operacional desktop baseado em Linux que combine:

1. experiência simples e familiar para quem vem do Windows 11;
2. capacidades Linux para desenvolvimento, servidores e personalização;
3. excelente suporte a jogos e criação de conteúdo;
4. segurança moderna por hardware, kernel, aplicações e rede;
5. atualizações confiáveis com rollback;
6. aparência militar/industrial, mas sem copiar identidade visual de terceiros;
7. operação local/offline sempre que possível.

## Personas

- **Gamer:** Steam, Proton, drivers GPU, periféricos e baixa latência.
- **Desenvolvedor:** Git, containers, IDEs, compiladores, Python, Node, Rust, Java e ferramentas Linux.
- **Criador:** Blender, áudio, vídeo, arte e 3D.
- **Operador técnico:** administração, logs, rede, backups e diagnóstico.
- **Usuário comum:** navegador, arquivos, mídia, impressoras e configurações sem precisar do terminal.

## Requisitos funcionais

### Desktop

- menu de aplicativos moderno;
- busca global;
- barra de tarefas configurável;
- suporte a múltiplos monitores;
- snapping de janelas;
- desktops virtuais;
- centro de notificações;
- explorador de arquivos com abas;
- histórico de clipboard;
- gerenciamento de energia;
- painel de configurações unificado;
- pesquisa de configurações;
- atalhos de teclado consistentes;
- terminal integrado opcional.

### Administração

- atualização gráfica;
- logs legíveis;
- modo diagnóstico;
- restauração para ponto anterior;
- recuperação de drivers;
- gerenciamento de serviços;
- controle de permissões por aplicação.

### Desenvolvimento

- toolchains Linux;
- containers;
- virtualização opcional;
- suporte a VS Code/IDE equivalentes;
- Git;
- SSH;
- SDKs para C/C++, Rust, Python, Java e JavaScript/TypeScript;
- documentação local.

### Jogos

- Steam como software suportado, sem dependência proprietária do SO;
- Proton;
- Vulkan;
- SDL;
- suporte a controle;
- overlay e captura opcional;
- perfis de desempenho;
- modo jogo de baixa interferência.

## Requisitos não funcionais

- segurança por padrão;
- boot verificável;
- atualizações assinadas;
- recuperação funcional sem internet;
- boa acessibilidade;
- baixo uso de memória em idle;
- diagnóstico reproduzível;
- logs sem segredos;
- componentes modulares.

## Princípio de compatibilidade

O sistema deve preferir padrões abertos e APIs documentadas. Recursos exclusivos de software proprietário só entram quando legalmente e tecnicamente possível e sem criar dependência arquitetural irreversível.
