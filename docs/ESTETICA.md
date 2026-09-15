# Estética — Industrial / Militar Futurista

## Direção

A identidade visual deve transmitir:

- robustez;
- organização;
- tecnologia;
- operação técnica;
- confiança;
- legibilidade.

A estética é **militar/industrial fictícia**, sem reproduzir interfaces governamentais ou de forças reais e sem usar símbolos oficiais.

## Paleta conceitual

- fundo escuro;
- cinza grafite;
- verde de diagnóstico;
- âmbar para atenção;
- vermelho somente para falha/crítico;
- branco/cinza claro para texto principal.

## Tipografia

Priorizar fontes open source, legíveis e disponíveis no Linux.

- títulos compactos;
- números monoespaçados em painéis técnicos;
- texto comum sem excesso de condensação.

## Elementos

- cartões de estado;
- indicadores de saúde;
- linhas técnicas discretas;
- grids;
- códigos de sistema;
- ícones simples;
- animações curtas e opcionais.

Evitar:

- excesso de HUD;
- aparência de jogo em toda a interface;
- elementos piscando continuamente;
- informação importante dependente apenas de cor.

## Identidade

Nome de trabalho: **Projeto Sentinel OS**.

Codinome interno de desenvolvimento pode mudar antes do primeiro release.

## Estados visuais

```text
GREEN   = operação normal
AMBER   = atenção / alteração pendente
RED     = falha crítica
BLUE    = informação
GRAY    = indisponível / desativado
```

## Tela de boot

Conceito:

```text
SENTINEL OS
SYSTEM INTEGRITY CHECK

BOOT      [OK]
KERNEL    [OK]
STORAGE   [OK]
NETWORK   [READY]
SECURITY  [OK]

Pressione Esc para diagnóstico
```

Não usar textos alarmistas que confundam o usuário.

## Desktop

O desktop deve ser elegante e funcional. A estética forte fica nos detalhes: ícones, estados, painéis, telas de diagnóstico, terminal e configurações.

## Tema claro

Também existirão temas claros para acessibilidade e ambientes muito iluminados. A identidade deve sobreviver ao tema claro.
