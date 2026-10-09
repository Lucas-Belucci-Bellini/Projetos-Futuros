# 16. Diretório de lojas e profissionais

## Objetivo funcional
Adicionar ao catálogo uma área separada para encontrar lojas de pisos e revestimentos e profissionais/empresas que executem instalação e acabamento em Londrina/PR.

Documento detalhado de referência: [PROFISSIONAIS_E_LOJAS_DE_PISOS.md](../../PROFISSIONAIS_E_LOJAS_DE_PISOS.md).

## Escopo do MVP
- Listagem separada de fornecedores e prestadores.
- Filtro por especialidade e região atendida.
- Página de detalhes com contatos comerciais públicos e fonte.
- Data da última verificação.
- Estado de cadastro para não publicar resultados não confirmados.
- Formulário para sugerir correções, inclusão ou remoção.

## Requisitos funcionais
1. O visitante consegue alternar entre lojas e profissionais.
2. Cada perfil mostra somente campos verificados e autorizados para publicação.
3. O visitante pode filtrar por categoria de material ou serviço.
4. Links e contatos levam a canais oficiais conhecidos.
5. O sistema mostra que preço, disponibilidade e serviços precisam ser confirmados diretamente.
6. Dados pendentes, suspensos ou removidos não aparecem no diretório público.
7. Alterações administrativas geram histórico de auditoria.

## Modelo de domínio proposto
- Business: empresa ou profissional.
- BusinessLocation: filial/local comercial, quando aplicável.
- BusinessCategory: especialidade, material ou serviço.
- BusinessContact: canal comercial público.
- BusinessSource: evidência de origem e verificação.
- BusinessVerificationEvent: registro de verificação, alteração ou suspensão.

Não colocar informações pessoais privadas de autônomos. Para quem trabalha em domicílio, preferir região atendida e contato comercial autorizado em vez de endereço residencial.

## Estados de cadastro
- discovered: encontrado, não avaliado.
- pending: aguardando confirmação.
- verified: confirmado e aprovado para exibição.
- suspended: temporariamente oculto por dúvida, reclamação ou dado desatualizado.
- removed: retirado do diretório.

Somente verified pode aparecer publicamente, e apenas com os campos aprovados.

## Critérios de aceite
- Lojas e profissionais são distinguíveis visualmente e nos dados.
- Filtros não mostram especialidades não confirmadas.
- Toda página pública contém fonte e data de verificação.
- Não há nomes, endereços, contatos, avaliações ou certificações inventados.
- Existe procedimento de correção e remoção.
- Testes cobrem estados pendente/verificado/suspenso e campos opcionais.
- A implementação não sugere parceria comercial inexistente.

## Sequência de implementação
1. Verificar a lista inicial de candidatos.
2. Fechar schema e migração do banco.
3. Adicionar endpoints de listagem e detalhe.
4. Criar páginas e filtros no frontend.
5. Criar revisão administrativa com autenticação.
6. Publicar somente registros verificados.
