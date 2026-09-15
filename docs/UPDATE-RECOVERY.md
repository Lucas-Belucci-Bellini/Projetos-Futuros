# Atualização e Recuperação

## Objetivo

Uma atualização não pode transformar uma máquina funcional em um sistema quebrado sem uma rota de retorno.

## Modelo

```text
versão A
  ↓
verificação
  ↓
stage da versão B
  ↓
verificação de assinatura
  ↓
aplicar atomicamente
  ↓
boot B
  ↓
health check
  ├── OK → confirmar B
  └── FAIL → rollback A
```

## Regras

- downloads verificados;
- metadados assinados;
- pacotes/artefatos com hashes;
- proteção contra downgrade não autorizado;
- estado de boot registrado;
- ponto anterior preservado até a nova versão ser validada.

## Rollback

O usuário deve conseguir retornar à versão anterior pelo menu de recuperação sem depender do sistema gráfico principal.

## Recuperação

Ter um ambiente de recuperação mínimo com:

- diagnóstico de boot;
- verificação de disco;
- restauração de snapshot;
- reinstalação sem apagar dados quando possível;
- terminal administrativo;
- exportação de logs.

## Driver quebrado

Se uma atualização de driver gráfico falhar, deve existir fallback para um driver seguro/mesa de diagnóstico quando possível.

## Atualização offline

O sistema deve permitir preparar uma atualização em outra máquina e transferi-la para instalação offline, desde que a cadeia de assinatura seja válida.

## Backups

Snapshot local e backup externo são conceitos diferentes. O OS deve explicar essa diferença claramente.

## Testes obrigatórios

- atualização normal;
- atualização interrompida;
- falta de energia durante update;
- rollback;
- driver quebrado;
- disco cheio;
- pacote inválido;
- assinatura inválida;
- recovery offline.
