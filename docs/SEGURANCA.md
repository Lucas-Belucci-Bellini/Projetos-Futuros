# Segurança

## Objetivo

A meta de segurança é criar uma plataforma com **defesa em profundidade**, tomando como inspiração o rigor de plataformas de jogos e os mecanismos modernos de hardware/OS, sem copiar componentes proprietários.

O VAC da Valve é anti-cheat e não equivale a uma arquitetura completa de segurança de SO. A documentação oficial descreve detecção automatizada de cheats, servidores protegidos e aplicação de banimentos. Este projeto usa isso como referência de seriedade, não como promessa de equivalência. citeturn570816search0turn570816search2

## Modelo de ameaça

Considerar:

- malware comum;
- ransomware;
- aplicativo malicioso;
- extensão de navegador comprometida;
- ataque por mídia removível;
- roubo físico do dispositivo;
- adulteração do boot;
- exploração de vulnerabilidade do kernel;
- credenciais roubadas;
- supply chain comprometida;
- driver malicioso ou vulnerável;
- erro do próprio usuário.

## Raiz de confiança

A referência é um hardware moderno com:

- UEFI;
- Secure Boot;
- TPM 2.0;
- suporte a measured boot;
- virtualização de hardware.

O Windows 11 utiliza TPM 2.0, Secure Boot, BitLocker e VBS/HVCI como partes de sua arquitetura de segurança; essas ideias servem de referência para requisitos do projeto. citeturn320009search1turn320009search0turn320009search4

## Boot seguro

Fluxo desejado:

```text
Firmware
  ↓
Secure Boot
  ↓
bootloader assinado
  ↓
kernel assinado/verificado
  ↓
init/system manager
  ↓
serviços confiáveis
```

O sistema deve detectar alterações inesperadas e fornecer uma rota clara de recuperação.

## TPM

Utilizar TPM 2.0 quando disponível para:

- proteção de chaves;
- measured boot;
- desbloqueio condicionado à integridade;
- proteção de credenciais;
- atestado local/futuro remoto.

O TPM não deve ser tratado como substituto de criptografia, autenticação ou atualizações seguras.

## Criptografia

- criptografia de disco como opção recomendada e, para perfis protegidos, padrão;
- chaves protegidas por hardware quando possível;
- recuperação documentada;
- nenhuma chave secreta hard-coded em componentes públicos.

## Privilégios

Princípio de menor privilégio.

- usuário normal sem acesso root;
- elevação explícita para operações administrativas;
- serviços com usuários dedicados;
- capabilities reduzidas quando apropriado;
- isolamento de serviços.

## Aplicações

Direção para desktop:

- sandbox;
- portals;
- permissões por aplicação;
- acesso controlado a arquivos, microfone, câmera, rede e dispositivos;
- confirmação para ações sensíveis.

## Integridade de aplicações

O sistema deve ter um mecanismo de assinatura e origem de pacotes:

```text
aplicativo
   ↓
manifesto
   ↓
assinatura
   ↓
fonte confiável
   ↓
verificação
   ↓
instalação
```

Pacotes modificados ou assinaturas inválidas devem falhar de maneira segura.

## Atualizações

- pacotes assinados;
- cadeia de confiança validada;
- atualização atômica;
- rollback;
- proteção contra downgrade indevido;
- registro da versão instalada;
- recuperação offline.

## Kernel e drivers

Priorizar:

- código upstream;
- patches mínimos;
- atualização rápida de vulnerabilidades;
- isolamento de drivers quando tecnicamente possível;
- monitoramento de vulnerabilidades.

## Telemetria

Telemetria deve ser:

- mínima;
- documentada;
- desligável quando possível;
- sem conteúdo pessoal desnecessário;
- agregada por padrão;
- sem captura de documentos, senhas ou conteúdo privado.

## Segurança de conta

- passkeys/FIDO2 quando disponíveis;
- MFA;
- sessões revogáveis;
- chaves de recuperação;
- proteção contra phishing;
- bloqueio progressivo contra tentativas automatizadas.

## Modo Jogo

O modo jogo pode reduzir processos não essenciais, mas nunca deve desligar silenciosamente controles críticos de segurança. O usuário deverá saber o que está sendo alterado.

## Anti-cheat

O sistema pode oferecer APIs para jogos verificarem integridade do ambiente, mas não deve prometer que qualquer anti-cheat de terceiros funcionará. O objetivo do OS é fornecer uma base verificável e estável; o jogo e sua infraestrutura continuam responsáveis pelas decisões específicas do anti-cheat.

## Testes de segurança

Antes de uma versão estável:

- boot integrity test;
- update/rollback test;
- privilege boundary test;
- sandbox escape test;
- fuzzing de componentes críticos;
- análise estática;
- dependency audit;
- SBOM;
- reprodução de builds quando viável;
- teste de recuperação offline.

## Meta de confiança

A expressão interna **SECURITY-TIER-HIGH** representa a meta arquitetural. Não significa certificação nem garantia absoluta contra comprometimento.
