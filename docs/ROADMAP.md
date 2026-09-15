# Roadmap

## Fase 0 — Especificação

- nome/codename;
- requisitos;
- modelo de ameaças;
- decisões de arquitetura;
- identidade visual;
- lista de hardware de teste.

## Fase 1 — Boot mínimo

Objetivo: iniciar um Linux customizado em VM e hardware de teste.

Entregáveis:

- imagem inicializável;
- UEFI;
- kernel;
- init;
- shell básico;
- logs.

## Fase 2 — Desktop

- Wayland;
- compositor;
- painel;
- menu;
- notificações;
- configurações;
- file manager inicial.

## Fase 3 — Sistema de aplicações

- instalação;
- atualização;
- sandbox;
- permissões;
- loja/catálogo opcional.

## Fase 4 — Segurança

- Secure Boot;
- assinatura de artefatos;
- TPM/measured boot;
- criptografia;
- sandbox;
- hardening;
- recuperação.

## Fase 5 — Gaming

- Vulkan;
- Steam;
- Proton;
- controles;
- Game Mode;
- teste de jogos;
- matriz de compatibilidade.

## Fase 6 — Developer Mode

- terminal;
- Git;
- containers;
- toolchains;
- SDK manager;
- integração com IDEs.

## Fase 7 — Release Candidate

- hardware matrix;
- regressão;
- segurança;
- recuperação;
- documentação;
- instalador;
- atualizações OTA/desktop;
- política de suporte.

## Fase 8 — Stable 1.0

Somente depois que o sistema:

- instalar de forma confiável;
- atualizar com rollback;
- recuperar de falhas comuns;
- executar jogos testados;
- proteger o sistema por padrão;
- manter documentação suficiente para diagnóstico.

## Pós-1.0

- integração com a Base Móvel;
- edição workstation;
- edição portátil;
- ferramentas de IA local;
- gerenciamento de clusters;
- sincronização opcional.
