use std::{env, fs, path::{Path, PathBuf}};

const REQUIRED: &[&str] = &[
    "## Identidade",
    "## Aparência",
    "## Estrutura",
    "## Entidades",
    "## Recursos",
    "## Bases",
    "## Entradas",
    "## Saídas",
    "## Mídia",
    "## Auditoria",
];

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                walk(&p, out);
            } else if p.extension().and_then(|x| x.to_str()) == Some("md") {
                out.push(p);
            }
        }
    }
}

fn body<'a>(text: &'a str, heading: &str) -> &'a str {
    let Some(start) = text.find(heading).map(|p| p + heading.len()) else {
        return "";
    };
    let tail = &text[start..];
    let end = tail.find("
## ").unwrap_or(tail.len());
    tail[..end].trim()
}

fn unknowns(text: &str) -> usize {
    ["Não documentado", "Não confirmado", "pendente", "não consolidado", "desconhecido"]
        .iter()
        .map(|marker| text.matches(marker).count())
        .sum()
}

fn main() {
    let root = env::args().nth(1).unwrap_or_else(|| "levels".into());
    let mut files = Vec::new();
    walk(Path::new(&root), &mut files);
    files.sort();

    println!("# Editorial quality report");
    println!();
    println!("| Arquivo | Seções | Auditoria marcada | Incertas/pendências | Mídia tem licença/status |");
    println!("|---|---:|---:|---:|---|");

    for path in files {
        let text = match fs::read_to_string(&path) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let present = REQUIRED.iter().filter(|heading| text.contains(**heading)).count();
        let audit = body(&text, "## Auditoria");
        let checked = audit.matches("[x]").count() + audit.matches("[X]").count();
        let uncertain = unknowns(&text);
        let media = body(&text, "## Mídia");
        let media_status = media.contains("Licença")
            || media.contains("licença")
            || media.contains("pendente")
            || media.contains("CC ");

        println!(
            "| {} | {}/10 | {}/10 | {} | {} |",
            path.display(),
            present,
            checked.min(10),
            uncertain,
            if media_status { "sim" } else { "não" }
        );
    }
}
