# IA Local e Automação

## Objetivo
Tratar IA como recurso opcional do sistema, mantendo o usuário no controle dos dados.

## Recursos
- Assistente local opcional.
- Descoberta de modelos instalados.
- Seleção de GPU/CPU/RAM conforme carga.
- Containers isolados para runtimes de IA.
- API local para integração com aplicações autorizadas.
- Integração futura com Hermes.

## Segurança
- Modelos e ferramentas de terceiros executados em sandbox quando possível.
- Permissões explícitas para acessar arquivos, microfone, câmera e rede.
- Sem envio obrigatório de dados para a nuvem.

## Hardware awareness
Detectar VRAM, RAM, CPU, armazenamento e temperatura para sugerir se uma tarefa deve executar localmente ou ser encaminhada a um servidor.

## Integração com Base Móvel
O SO deve poder detectar serviços autorizados no SERVER-02 e usar a rede interna sem expor serviços de administração para a internet pública.
