# Documentação e Governança

## Regra de branches

`main` = referência estável.

`projeto/base-movel` = integração e arquitetura geral.

`base-movel/servidores` = hardware e software dos quatro servidores.
`base-movel/casa` = área habitacional.
`base-movel/energia` = geração, armazenamento e distribuição elétrica.
`base-movel/rede` = conectividade e segurança de rede.
`base-movel/storage` = NAS, nuvem e backups.
`base-movel/notebooks` = notebooks.
`base-movel/veiculo` = caminhão/baú e integração física.
`base-movel/climatizacao` = refrigeração da infraestrutura.

## Fluxo recomendado

1. Alterar somente a branch do subsistema afetado.
2. Registrar decisões importantes em Markdown/ADR.
3. Quando houver dependência entre subsistemas, atualizar primeiro `projeto/base-movel`.
4. Integrar mudanças em `main` somente após revisão.
