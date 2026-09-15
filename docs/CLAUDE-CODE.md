# Instruções para Claude Code

## Objetivo

Quando esta branch for usada para iniciar a implementação, trate a documentação como a especificação do produto. Não pule diretamente para uma UI sem validar arquitetura, segurança e compatibilidade.

## Ordem de leitura obrigatória

1. `README.md`
2. `docs/VISAO-E-REQUISITOS.md`
3. `docs/ARQUITETURA.md`
4. `docs/SEGURANCA.md`
5. `docs/UX-WINDOWS11.md`
6. `docs/ESTETICA.md`
7. `docs/COMPATIBILIDADE-E-JOGOS.md`
8. `docs/PACOTE-HARDWARE.md`
9. `docs/UPDATE-RECOVERY.md`
10. `docs/PRIVACIDADE.md`
11. `docs/ROADMAP.md`

## Regras de implementação

- Não assumir decisões que a documentação marcou como sujeitas a ADR.
- Não substituir componentes centrais do Linux sem justificativa.
- Não implementar criptografia própria quando biblioteca/algoritmo padrão e auditado atender ao requisito.
- Não criar um sistema de autenticação caseiro para substituir padrões estabelecidos.
- Não desabilitar Secure Boot, sandbox ou controles críticos apenas para tornar um protótipo mais simples.
- Prototipar primeiro em VM e hardware de teste antes de declarar compatibilidade.
- Toda dependência nova deve ser registrada com licença, versão e motivo.
- Toda alteração arquitetural significativa deve gerar uma ADR.

## Primeiro protótipo

O primeiro objetivo não é criar o SO inteiro. Criar:

```text
ISO bootável
  ↓
Linux kernel
  ↓
Wayland
  ↓
Desktop shell inicial
  ↓
File manager inicial
  ↓
Settings inicial
  ↓
Terminal
```

Depois adicionar segurança, atualizações, recuperação e gaming em fases controladas.

## Estrutura sugerida do código

```text
os/
├── build/
├── boot/
├── kernel-config/
├── system/
├── shell/
├── file-manager/
├── settings/
├── security/
├── updater/
├── recovery/
├── gaming/
├── installer/
├── packages/
├── tests/
└── docs/
```

## Qualidade

Antes de marcar uma fase como concluída:

- build reproduzível quando possível;
- testes automatizados;
- boot test;
- update/rollback test;
- security test;
- hardware test;
- documentação atualizada.

## Política de trabalho

Não copiar o Windows 11 ou tecnologias proprietárias da Valve. A inspiração deve ser traduzida em requisitos próprios e implementada usando software livre, padrões abertos e componentes com licenças compatíveis.

## Resultado esperado

No final de cada fase, produzir:

- código;
- testes;
- documentação;
- riscos conhecidos;
- decisão de continuar, revisar ou reverter.
