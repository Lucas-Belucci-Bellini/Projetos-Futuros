#!/usr/bin/env python3
"""Converte os orçamentos em Markdown para PDF.

Sem dependência de pandoc/LaTeX: usa fpdf2 com DejaVu Sans (Unicode).
Suporta: títulos, parágrafos, listas, tabelas, blocos de código, citações,
regras horizontais, negrito e links inline.

Uso:
    python3 scripts/md2pdf.py saida.pdf "Título" "Subtítulo" arquivo1.md [...]
"""
import re
import sys
from pathlib import Path

from fpdf import FPDF
from fpdf.enums import XPos, YPos

FONT_DIR = Path("/usr/share/fonts/truetype/dejavu")
PAGE_W, MARGIN = 210, 16
CONTENT_W = PAGE_W - 2 * MARGIN

INK = (24, 26, 30)
MUTED = (110, 116, 126)
ACCENT = (150, 110, 20)
RULE = (208, 212, 218)
TABLE_HEAD = (238, 240, 243)
CODE_BG = (243, 244, 246)
LINK = (24, 78, 168)

# Emoji -> equivalente que a DejaVu desenha. Sem isso o glifo sai como caixa.
GLYPHS = {
    "⭐": "*", "✅": "✓", "❌": "✗", "⚠️": "!", "⚠": "!",
    "🔴": "[!]", "🟡": "[~]", "🟢": "[ok]", "🟠": "[+]", "💡": "*",
    "🔗": "\u00bb", "👉": "->", "📌": "*", "🖥️": "", "🖥": "", "🏠": "",
    "🔫": "", "⚡": "", "❄️": "", "❄": "", "🚪": "", "🛡️": "", "🛡": "",
    "🧭": "", "📐": "", "📊": "", "🤖": "", "🚧": "", "🔒": "[trancado]",
    "️": "", "️": "",
}
# Preserva U+2713/U+2717 (check e cruz), que sao o destino do mapa acima.
EMOJI_RE = re.compile(
    "[\U0001F000-\U0001FAFF\U0001F1E6-\U0001F1FF"
    "\u2600-\u2712\u2714-\u2716\u2718-\u27BF]+"
)


def clean(text):
    for bad, good in GLYPHS.items():
        text = text.replace(bad, good)
    return EMOJI_RE.sub("", text)


def inline(text):
    """Achata markdown inline que o fpdf2 não renderiza, preservando negrito."""
    text = re.sub(r"\[([^\]]+)\]\(([^)]+)\)", r"\1", text)   # links -> rótulo
    text = text.replace("`", "")
    text = re.sub(r"(?<!\*)\*(?!\*)([^*]+)\*(?!\*)", r"\1", text)  # itálico
    return clean(text).strip()


class Doc(FPDF):
    def __init__(self, title, subtitle):
        super().__init__(format="A4")
        self.doc_title, self.doc_subtitle = title, subtitle
        self.cover_page = 1
        self.set_margins(MARGIN, 18, MARGIN)
        self.set_auto_page_break(True, margin=18)
        self.add_font("dv", "", str(FONT_DIR / "DejaVuSans.ttf"))
        self.add_font("dv", "B", str(FONT_DIR / "DejaVuSans-Bold.ttf"))
        # A DejaVu deste sistema nao traz o oblique; a Liberation cobre.
        # Italico tambem em DejaVu: e a unica familia deste sistema com os
        # simbolos que o texto usa (U+2713 / U+2717). Citacao se distingue
        # por cor e barra lateral, nao por inclinacao.
        self.add_font("dv", "I", str(FONT_DIR / "DejaVuSans.ttf"))
        self.add_font("dv", "BI", str(FONT_DIR / "DejaVuSans-Bold.ttf"))
        self.add_font("dvm", "", str(FONT_DIR / "DejaVuSansMono.ttf"))
        self.set_text_color(*INK)

    def header(self):
        if self.page_no() == self.cover_page:
            return
        self.set_font("dv", "", 7.5)
        self.set_text_color(*MUTED)
        self.cell(0, 5, self.doc_title, align="R",
                  new_x=XPos.LMARGIN, new_y=YPos.NEXT)
        self.set_draw_color(*RULE)
        self.set_line_width(0.2)
        self.line(MARGIN, self.get_y(), PAGE_W - MARGIN, self.get_y())
        self.ln(3)
        self.set_text_color(*INK)

    def footer(self):
        if self.page_no() == self.cover_page:
            return
        self.set_y(-14)
        self.set_font("dv", "", 7.5)
        self.set_text_color(*MUTED)
        self.cell(0, 5, f"{self.page_no()}", align="C")
        self.set_text_color(*INK)

    def make_cover(self):
        self.add_page()
        self.ln(75)
        self.set_font("dv", "B", 26)
        self.multi_cell(CONTENT_W, 11, clean(self.doc_title), align="L")
        self.ln(3)
        self.set_draw_color(*ACCENT)
        self.set_line_width(1.1)
        self.line(MARGIN, self.get_y(), MARGIN + 52, self.get_y())
        self.ln(7)
        self.set_font("dv", "", 12)
        self.set_text_color(*MUTED)
        self.multi_cell(CONTENT_W, 6.5, clean(self.doc_subtitle), align="L")
        self.ln(14)
        self.set_font("dv", "", 9)
        self.multi_cell(
            CONTENT_W, 5,
            "Cotação de 15 de setembro de 2026 — preços em USD, mercado dos EUA.\n"
            "Preços de hardware mudam diariamente: confira o link da fonte "
            "antes de comprar.\n\nProjetos-Futuros / base-movel",
            align="L")
        self.set_text_color(*INK)
        self.cover_page = self.page_no()

    # ---------- blocos ----------

    def heading(self, level, text):
        # O titulo inteiro ja e desenhado em negrito: os ** do fonte seriam
        # impressos literalmente, porque heading nao passa por markdown=True.
        text = inline(text).replace("**", "")
        sizes = {1: 17, 2: 13, 3: 11, 4: 9.8}
        space_before = {1: 7, 2: 6, 3: 4.5, 4: 3.5}
        if level == 1 and self.get_y() > 60:
            self.add_page()
        else:
            self.ln(space_before[level])
        if self.get_y() > 245:
            self.add_page()
        self.set_font("dv", "B", sizes[level])
        self.set_text_color(*(ACCENT if level <= 2 else INK))
        self.multi_cell(CONTENT_W, sizes[level] * 0.46, text, align="L")
        self.set_text_color(*INK)
        if level <= 2:
            self.set_draw_color(*RULE)
            self.set_line_width(0.3 if level == 1 else 0.2)
            y = self.get_y() + 1.2
            self.line(MARGIN, y, PAGE_W - MARGIN, y)
            self.ln(3)
        else:
            self.ln(1.5)

    def para(self, text):
        self.set_font("dv", "", 9.2)
        self.multi_cell(CONTENT_W, 4.9, inline(text), align="L", markdown=True)
        self.ln(1.8)

    def bullet(self, text, depth=0):
        indent = 4 + depth * 5
        self.set_font("dv", "", 9.2)
        x0 = self.get_x()
        self.set_x(MARGIN + indent)
        self.cell(3.4, 4.9, "•")
        self.multi_cell(CONTENT_W - indent - 3.4, 4.9, inline(text),
                        align="L", markdown=True)
        self.set_x(x0)

    def numbered(self, num, text, depth=0):
        indent = 4 + depth * 5
        self.set_font("dv", "", 9.2)
        x0 = self.get_x()
        self.set_x(MARGIN + indent)
        self.cell(5.5, 4.9, f"{num}.")
        self.multi_cell(CONTENT_W - indent - 5.5, 4.9, inline(text),
                        align="L", markdown=True)
        self.set_x(x0)

    def quote(self, lines):
        text = inline(" ".join(lines))
        self.ln(1)
        y0 = self.get_y()
        self.set_font("dv", "I", 8.8)
        self.set_text_color(*MUTED)
        self.set_x(MARGIN + 4)
        self.multi_cell(CONTENT_W - 6, 4.6, text, align="L", markdown=True)
        self.set_draw_color(*ACCENT)
        self.set_line_width(0.7)
        if self.get_y() > y0:
            self.line(MARGIN + 1, y0, MARGIN + 1, self.get_y() - 1)
        self.set_text_color(*INK)
        self.set_x(MARGIN)
        self.ln(2)

    @staticmethod
    def code_height(lines):
        size = 6.4 if max((len(l) for l in lines), default=0) > 74 else 7.4
        return size * 0.52 * len(lines) + 4

    def code(self, lines):
        size = 6.4 if max((len(l) for l in lines), default=0) > 74 else 7.4
        lh = size * 0.52
        self.ln(1.5)
        if self.get_y() + lh * len(lines) > 268 and lh * len(lines) < 230:
            self.add_page()
        self.set_font("dvm", "", size)
        self.set_fill_color(*CODE_BG)
        for line in lines:
            if self.get_y() > 268:
                self.add_page()
                self.set_font("dvm", "", size)
            self.cell(CONTENT_W, lh, "  " + clean(line.replace("\t", "    ")),
                      fill=True, new_x=XPos.LMARGIN, new_y=YPos.NEXT)
        self.ln(2.5)

    def table(self, rows):
        cols = max(len(r) for r in rows)
        rows = [r + [""] * (cols - len(r)) for r in rows]
        rows = [[inline(c) for c in r] for r in rows]

        weights = [max(len(r[i]) for r in rows) ** 0.62 for i in range(cols)]
        total = sum(weights) or 1
        widths = [max(11.0, CONTENT_W * w / total) for w in weights]
        widths = [w * CONTENT_W / sum(widths) for w in widths]

        size = 7.6 if cols <= 5 else (7.0 if cols <= 7 else 6.3)
        self.ln(1.5)
        for idx, row in enumerate(rows):
            head = idx == 0
            self.set_font("dv", "B" if head else "", size)
            heights = []
            for i, cell in enumerate(row):
                # Mede na MESMA largura em que desenha (widths[i] - 2),
                # senao a altura da linha fica curta e o texto e cortado.
                n = len(self.multi_cell(widths[i] - 2, size * 0.54, cell,
                                        dry_run=True, output="LINES",
                                        align="L", markdown=True))
                heights.append(max(1, n))
            h = max(heights) * size * 0.54 + 1.6
            if self.get_y() + h > 272:
                self.add_page()
                self.set_font("dv", "B" if head else "", size)
            y0, x = self.get_y(), MARGIN
            self.set_fill_color(*(TABLE_HEAD if head else (255, 255, 255)))
            self.set_draw_color(*RULE)
            self.set_line_width(0.15)
            for i, cell in enumerate(row):
                self.set_xy(x, y0)
                self.rect(x, y0, widths[i], h, style="DF")
                self.set_xy(x + 1, y0 + 0.8)
                self.multi_cell(widths[i] - 2, size * 0.54, cell,
                                align="L", markdown=True)
                x += widths[i]
            self.set_xy(MARGIN, y0 + h)
        self.ln(3)

    def rule(self):
        self.ln(2.5)
        self.set_draw_color(*RULE)
        self.set_line_width(0.25)
        self.line(MARGIN + 40, self.get_y(), PAGE_W - MARGIN - 40, self.get_y())
        self.ln(3.5)


def gather_item(lines, i):
    """Junta as linhas de continuacao (indentadas) de um item de lista.

    Sem isso, um item quebrado em duas linhas no fonte perde a segunda metade
    para um paragrafo solto — e marcacao inline aberta numa linha e fechada na
    outra vaza como texto literal.
    """
    parts, j = [], i + 1
    while j < len(lines):
        nxt = lines[j]
        if not nxt.strip():
            break
        if not nxt[:1].isspace():
            break
        if re.match(r"^\s*([-*+]|\d+[.)])\s+", nxt):
            break
        if nxt.strip().startswith(("```", "|", "#")):
            break
        parts.append(nxt.strip())
        j += 1
    return " ".join(parts), j


def render(pdf, md):
    lines = md.split("\n")
    i, para_buf, quote_buf = 0, [], []

    def flush():
        nonlocal para_buf, quote_buf
        if para_buf:
            pdf.para(" ".join(para_buf))
            para_buf = []
        if quote_buf:
            pdf.quote(quote_buf)
            quote_buf = []

    while i < len(lines):
        line = lines[i]
        stripped = line.strip()

        if stripped.startswith("```"):
            flush()
            i += 1
            block = []
            while i < len(lines) and not lines[i].strip().startswith("```"):
                block.append(lines[i])
                i += 1
            pdf.code(block)
            i += 1
            continue

        if not stripped:
            flush()
            i += 1
            continue

        m = re.match(r"^(#{1,4})\s+(.*)", stripped)
        if m:
            flush()
            # Se o proximo bloco for um diagrama alto que nao cabe no resto da
            # pagina, quebra antes do titulo para os dois irem juntos.
            j = i + 1
            while j < len(lines) and not lines[j].strip():
                j += 1
            if j < len(lines) and lines[j].strip().startswith("```"):
                k, blk = j + 1, []
                while k < len(lines) and not lines[k].strip().startswith("```"):
                    blk.append(lines[k])
                    k += 1
                height = pdf.code_height(blk)
                if height < 225 and pdf.get_y() + 16 + height > 268:
                    pdf.add_page()
            pdf.heading(len(m.group(1)), m.group(2))
            i += 1
            continue

        if re.match(r"^(---+|\*\*\*+|___+)$", stripped):
            flush()
            pdf.rule()
            i += 1
            continue

        if stripped.startswith(">"):
            if para_buf:
                pdf.para(" ".join(para_buf))
                para_buf = []
            quote_buf.append(stripped.lstrip("> ").rstrip())
            i += 1
            continue

        # tabela: linha com | seguida de separador |---|
        if stripped.startswith("|") and i + 1 < len(lines) and \
                re.match(r"^\s*\|[\s:|-]+\|\s*$", lines[i + 1]):
            flush()
            rows = []
            while i < len(lines) and lines[i].strip().startswith("|"):
                raw = lines[i].strip()
                if not re.match(r"^\|[\s:|-]+\|$", raw):
                    rows.append([c.strip() for c in raw.strip("|").split("|")])
                i += 1
            if rows:
                pdf.table(rows)
            continue

        m = re.match(r"^(\s*)[-*+]\s+(.*)", line)
        if m:
            flush()
            extra, i = gather_item(lines, i)
            texto = (m.group(2) + " " + extra).strip() if extra else m.group(2)
            pdf.bullet(texto, depth=min(len(m.group(1)) // 2, 2))
            continue

        m = re.match(r"^(\s*)(\d+)[.)]\s+(.*)", line)
        if m:
            flush()
            extra, i = gather_item(lines, i)
            texto = (m.group(3) + " " + extra).strip() if extra else m.group(3)
            pdf.numbered(m.group(2), texto,
                         depth=min(len(m.group(1)) // 2, 2))
            continue

        para_buf.append(stripped)
        i += 1

    flush()


def main():
    out, title, subtitle, *sources = sys.argv[1:]
    pdf = Doc(title, subtitle)
    pdf.make_cover()
    for n, src in enumerate(sources):
        text = Path(src).read_text(encoding="utf-8")
        if n:
            pdf.add_page()
        render(pdf, text)
    Path(out).parent.mkdir(parents=True, exist_ok=True)
    pdf.output(out)
    kb = Path(out).stat().st_size / 1024
    print(f"{out}: {pdf.page_no()} paginas, {kb:.0f} KB")


if __name__ == "__main__":
    main()
