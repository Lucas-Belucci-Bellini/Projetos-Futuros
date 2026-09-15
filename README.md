# Sistema Operacional — Projeto V2

Branch: `sistema-operacional-linux`

Projeto para criar um sistema operacional desktop baseado em Linux, com experiência moderna, segurança forte, gaming, desenvolvimento, IA local, operação offline e identidade visual militar/industrial futurista.

## Objetivo V2

Não é um clone do Windows. A proposta é construir um produto próprio combinando usabilidade de desktop moderna, ferramentas avançadas sem exigir conhecimento de Linux, segurança por defesa em profundidade, atualizações atômicas e rollback, sandbox, gaming, desenvolvimento, IA local, privacidade e recuperação simples.

## Documentação principal

- `docs/OBJETIVOS-V2.md` — visão, escopo, diferenciais e limites.
- `docs/ARQUITETURA-DE-IMPLEMENTACAO.md` — camadas, tecnologias, modularidade e observabilidade.
- `docs/SEGURANCA-DEFESA-EM-PROFUNDIDADE.md` — cadeia de confiança, isolamento, identidade, updates e resposta.
- `docs/EXPERIENCIA-DESKTOP.md` — shell, arquivos, configurações, perfis e acessibilidade.
- `docs/GAMING-E-COMPATIBILIDADE.md` — Steam, Proton, drivers, Vulkan e política de anti-cheat.
- `docs/GERENCIADOR-DE-APLICATIVOS.md` — instalação, sandbox, repositórios e permissões.
- `docs/ATUALIZACAO-E-RECUPERACAO.md` — update transacional, rollback e recovery offline.
- `docs/IA-LOCAL-E-AUTOMACAO.md` — IA local, Hermes e integração com servidores.
- `docs/DESEMPENHO-E-ENERGIA.md` — perfis de energia, hardware awareness e benchmarks.
- `docs/PRIVACIDADE-E-DADOS.md` — telemetria, contas, criptografia e coleta mínima.
- `docs/ESTETICA-E-TEMA.md` — sistema visual militar/industrial e identidade própria.

## Segurança

A meta de "nível Steam/VAC" significa alto rigor de integridade, não equivalência técnica ao VAC. O projeto não copia mecanismos proprietários e não promete segurança invulnerável.

A defesa deve usar, quando disponível, Secure Boot, TPM 2.0, measured boot, privilégios mínimos, sandboxing, assinatura de software, atualizações assinadas, proteção de downgrade, auditoria local e recovery confiável.

## Arquitetura inicial

- Kernel Linux.
- UEFI + Secure Boot.
- systemd como referência inicial.
- Wayland como protocolo gráfico principal.
- Portais/sandbox modernos para aplicações.
- Componentes críticos novos preferencialmente em Rust quando adequado.
- C/C++ apenas quando exigido por componentes existentes ou interfaces do sistema.
- Suporte prioritário a AMD, NVIDIA e Intel.

## Perfis de uso

`Eco` · `Balanced` · `Performance` · `Gaming` · `AI`

## Requisitos para o futuro Claude Code

Antes de implementar, ler todo este README e todos os arquivos em `docs/`. Não começar pelo visual isolado. Primeiro transformar requisitos em ADRs, arquitetura, protótipo mínimo bootável e testes automatizados.

A implementação deve manter módulos independentes, documentação atualizada, testes e critérios de aceitação por fase.

## Roadmap resumido

1. Especificação e threat model.
2. Boot mínimo e imagem ISO.
3. Base Linux + hardware detection.
4. Desktop Shell.
5. Apps e sandbox.
6. Updates/recovery.
7. Gaming.
8. Desenvolvimento.
9. IA local.
10. Hardening e auditoria.
11. Beta público.
