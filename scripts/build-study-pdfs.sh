#!/usr/bin/env bash
set -euo pipefail

ROOT="estudos"

is_revision_file() {
  case "$1" in
    */REVISAO-*.md|*/V2-MEMORIA-*.md|*/V3-REVISAO-*.md|*/V4-APRENDENDO-*.md|*/V5-GUIA-*.md)
      return 0 ;;
    *)
      return 1 ;;
  esac
}

mapfile -t REVISION_FILES < <(
  find "$ROOT" -type f -name "*.md" -print0 |
    while IFS= read -r -d '' file; do
      if is_revision_file "$file"; then
        printf '%s\n' "$file"
      fi
    done |
    sort
)

if [[ ${#REVISION_FILES[@]} -eq 0 ]]; then
  echo "Nenhuma revisão encontrada."
  exit 1
fi

echo "Encontradas ${#REVISION_FILES[@]} revisões."

for md in "${REVISION_FILES[@]}"; do
  pdf="${md%.md}.pdf"
  echo "Gerando: $pdf"
  pandoc "$md" \
    --from markdown+tex_math_dollars+tex_math_single_backslash \
    --standalone \
    --pdf-engine=xelatex \
    --variable=geometry:margin=1.8cm \
    --variable=fontsize=11pt \
    --variable=lang=pt-BR \
    --variable=colorlinks=true \
    --output="$pdf"
done

MASTER="$ROOT/ESTUDOS-REVISOES-COMPLETAS.pdf"
echo "Gerando pacote completo: $MASTER"
pandoc "${REVISION_FILES[@]}" \
  --from markdown+tex_math_dollars+tex_math_single_backslash \
  --standalone \
  --pdf-engine=xelatex \
  --variable=geometry:margin=1.8cm \
  --variable=fontsize=11pt \
  --variable=lang=pt-BR \
  --variable=colorlinks=true \
  --output="$MASTER"

INDEX="$ROOT/PDF-INDEX.md"
{
  echo "# PDFs das revisões"
  echo
  echo "PDFs gerados automaticamente a partir das revisões Markdown desta pasta."
  echo
  echo "[Baixar/abrir o pacote completo](ESTUDOS-REVISOES-COMPLETAS.pdf)"
  echo
  declare -A seen
  for md in "${REVISION_FILES[@]}"; do
    dir="$(dirname "$md")"
    base="$(basename "$md" .md)"
    pdf="$base.pdf"
    if [[ -z "${seen[$dir]+x}" ]]; then
      echo "## ${dir#estudos/}"
      seen["$dir"]=1
    fi
    echo "- [$base]($dir/$pdf)"
  done
} > "$INDEX"

echo "Índice criado em $INDEX"
