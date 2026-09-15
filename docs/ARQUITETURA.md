# Arquitetura

## Camadas

```text
UEFI
  ↓
Secure Boot / measured boot
  ↓
Bootloader
  ↓
Linux kernel
  ↓
system services
  ↓
Graphics stack (Wayland)
  ↓
Desktop Shell
  ↓
System apps / sandboxed apps
```

## Componentes principais

### 1. Boot

- UEFI-only para hardware suportado.
- Secure Boot como modo recomendado e futuro requisito de segurança para instalações protegidas.
- TPM 2.0 para chaves, medição e atestado quando o hardware oferecer suporte.
- Bootloader assinado.

### 2. Kernel

Linux upstream como base, evitando forks profundos.

Objetivos:

- atualizar regularmente;
- reduzir patches próprios;
- manter drivers em caminhos upstream sempre que possível;
- hardening documentado;
- perfis de baixa latência opcionais.

### 3. Gráficos

Wayland como direção principal.

- compositor próprio ou integração com compositor existente;
- XWayland para compatibilidade;
- Vulkan como API gráfica preferencial para aplicações/games;
- OpenGL via camadas de compatibilidade quando necessário.

### 4. Desktop Shell

O shell é o diferencial do projeto.

Funções:

- barra de tarefas;
- menu de aplicativos;
- busca;
- central de notificações;
- bandeja de sistema;
- widgets opcionais;
- snapping;
- desktops virtuais;
- múltiplos monitores;
- atalhos;
- painel rápido;
- modo jogo;
- modo produtividade;
- modo técnico.

### 5. File Manager

Criar/usar uma base Linux com uma camada própria de UX.

Objetivos:

- abas;
- breadcrumbs;
- busca rápida;
- favoritos;
- múltiplas janelas;
- preview;
- permissões legíveis;
- operações em lote;
- histórico de operações;
- recuperação de arquivos quando possível.

### 6. Aplicações

Direção inicial: Flatpak/portal/sandbox para aplicações de desktop, com pacotes nativos quando necessário para componentes do sistema.

### 7. Serviços

systemd é a referência inicial por maturidade e integração, sujeito a decisão formal durante o protótipo.

### 8. Rede

NetworkManager como padrão inicial para desktop.

Suporte a:

- Ethernet;
- Wi-Fi;
- VPN;
- Bluetooth;
- hotspot;
- DNS seguro;
- perfis por rede.

### 9. Armazenamento

- ext4 como caminho conservador inicial;
- Btrfs para sistemas que usarem snapshots/rollback, após validação;
- opção futura de ZFS somente quando houver justificativa clara para o perfil de máquina.

## Imutabilidade

A direção de longo prazo é um sistema de base preferencialmente atômico/imutável, com atualizações transacionais e rollback. A experiência deve permitir tanto um modo simples de atualização quanto um modo técnico para desenvolvedores.

## Virtualização

A máquina não precisa usar VM para tudo.

Virtualização é reservada para:

- ambientes de teste;
- compatibilidade;
- laboratórios;
- segurança adicional quando justificada.

## Princípio de modularidade

Cada subsistema deve poder ser testado isoladamente:

```text
boot
kernel
drivers
network
security
graphics
desktop
apps
update
recovery
```

Nenhum componente experimental deve ser requisito para o boot do sistema até que tenha estabilidade comprovada.
