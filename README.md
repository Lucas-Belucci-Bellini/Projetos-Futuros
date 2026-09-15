# Sistema Operacional Linux — Projeto

Branch: `sistema-operacional-linux`

Projeto conceitual para criar um sistema operacional desktop baseado em Linux, com foco em:

- experiência de uso inspirada nos melhores aspectos do Windows 11;
- compatibilidade com jogos, desenvolvimento e IA;
- segurança por camadas com raiz de confiança em hardware;
- atualização confiável e rollback;
- isolamento de aplicações;
- estética militar/industrial futurista;
- funcionamento offline e administração local;
- documentação suficiente para que o Claude Code possa iniciar a implementação futuramente.

## Princípio central

Não será um clone do Windows. O objetivo é combinar **usabilidade de desktop moderna + fundamentos Linux + segurança forte + identidade própria**.

## Segurança alvo

A meta "nível Steam/VAC" é tratada como **referência de rigor**, não como uma alegação de equivalência técnica. O sistema deverá ter uma arquitetura de defesa em profundidade, incluindo Secure Boot, TPM 2.0, measured boot, criptografia, sandboxing, controle de aplicações, atualização assinada, redução de privilégios, telemetria opcional e recuperação segura.

O VAC da Valve é um mecanismo de anti-cheat específico para jogos e não uma certificação geral de segurança de sistema operacional. A documentação oficial descreve detecção de cheats e penalidades em servidores protegidos; este projeto busca aplicar o mesmo espírito de proteção e integridade a um SO inteiro, sem copiar tecnologia proprietária da Valve.

## Documentação

- `docs/VISAO-E-REQUISITOS.md` — requisitos funcionais e não funcionais.
- `docs/ARQUITETURA.md` — arquitetura de alto nível.
- `docs/SEGURANCA.md` — modelo de ameaça e controles de segurança.
- `docs/UX-WINDOWS11.md` — recursos de UX a preservar/reinterpretar.
- `docs/ESTETICA.md` — identidade visual militar/industrial.
- `docs/COMPATIBILIDADE-E-JOGOS.md` — Steam, Proton, anti-cheat e drivers.
- `docs/PACOTE-HARDWARE.md` — requisitos mínimos, recomendados e high-end.
- `docs/UPDATE-RECOVERY.md` — atualizações atômicas, rollback e recuperação.
- `docs/PRIVACIDADE.md` — telemetria e proteção de dados.
- `docs/ROADMAP.md` — fases do projeto.
- `docs/CLAUDE-CODE.md` — instruções para futura construção com Claude Code.

## Base técnica inicial

- Kernel Linux.
- Boot UEFI + Secure Boot.
- systemd como referência inicial, sujeito a ADR futura.
- Wayland como compositor/protocolo gráfico de referência.
- Portais e sandbox de aplicações baseados em tecnologias Linux modernas.
- Pacotes imutáveis/atômicos como direção arquitetural, a validar durante prototipagem.
- Suporte a NVIDIA e AMD como prioridade.

## Regra de projeto

Toda decisão arquitetural deve virar ADR antes de ser considerada definitiva. O sistema deve evitar copiar código, marcas ou componentes proprietários sem licença compatível.
