# Compatibilidade e Jogos

## Objetivo

Tornar o sistema uma plataforma Linux de desktop que seja confortável para jogos modernos, sem sacrificar segurança ou estabilidade.

## Stack

- Vulkan;
- Mesa para hardware AMD/Intel quando apropriado;
- drivers NVIDIA oficiais quando necessários;
- XWayland para aplicações legadas;
- Proton para jogos Windows compatíveis;
- GameMode ou equivalente para perfis de desempenho;
- PipeWire para áudio.

## Steam

O Steam deve ser tratado como uma aplicação importante, mas o OS deve continuar utilizável sem ele.

## Proton

Objetivos:

- instalação simples;
- seleção automática de versão recomendada;
- fallback manual;
- indicação de compatibilidade;
- logs técnicos acessíveis;
- perfil por jogo.

## Anti-cheat

Não prometer compatibilidade universal com anti-cheats. Alguns jogos dependem de componentes específicos do Windows ou de decisões do desenvolvedor.

O OS deve fornecer uma base previsível:

- Secure Boot;
- kernel íntegro;
- drivers assinados/verificados quando possível;
- APIs estáveis;
- baixa fragmentação.

## Drivers

Gerenciamento de drivers deve ter:

- versão atual;
- versão recomendada;
- origem;
- assinatura;
- data de instalação;
- opção de rollback;
- aviso de incompatibilidade.

## Game Mode

O modo jogo poderá:

- mudar o perfil de energia;
- priorizar recursos do jogo;
- reduzir tarefas não essenciais;
- controlar atualizações para evitar interrupção;
- coletar métricas locais opcionais.

Nunca desabilitar silenciosamente mecanismos essenciais de segurança.

## Hardware alvo

### Básico

- GPU integrada moderna;
- 16 GB RAM;
- SSD.

### Recomendado

- 32 GB RAM;
- GPU dedicada AMD/NVIDIA;
- SSD NVMe;
- TPM 2.0;
- UEFI Secure Boot.

### Workstation

- 64 GB+;
- GPUs de alta VRAM;
- múltiplos monitores;
- 10 GbE quando necessário.

## Critério de qualidade

Um jogo é considerado suportado quando:

1. instala;
2. inicia;
3. entra no menu;
4. joga por período de teste;
5. áudio e controle funcionam;
6. suspensão/retorno não quebram o sistema;
7. atualização do jogo não remove a configuração.
