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

fn is_subsection(path: &Path) -> bool {
    let name = path.file_stem().and_then(|x| x.to_str()).unwrap_or_default();
    name.contains('-') &&
        !name.ends_with("-00") &&
        !name.ends_with("-01") &&
        !name.ends_with("-02") &&
        !name.ends_with("-03") &&
        !name.ends_with("-04") &&
        !name.ends_with("-05") &&
        !name.ends_with("-06") &&
        !name.ends_with("-07") &&
        !name.ends_with("-08") &&
        !name.ends_with("-09") &&
        !name.ends_with("-10")
}

fn main() {
    let root = env::args().nth(1).unwrap_or_else(|| "levels".into());
    let mut files = Vec::new();
    walk(Path::new(&root), &mut files);
    files.sort();

    let mut main_count = 0usize;
    let mut subsection_count = 0usize;
    let mut failing = 0usize;

    println!("# Backrooms inventory report");
    println!();
    println!("| Arquivo | Tipo | Caracteres | Seções ausentes | Marcadores unresolved |");
    println!("|---|---|---:|---:|---:|");

    for path in files {
        let text = match fs::read_to_string(&path) {
            Ok(v) => v,
            Err(_) => {
                println!("| {} | erro de leitura | 0 | 10 | 1 |", path.display());
                failing += 1;
                continue;
            }
        };

        let missing = REQUIRED.iter().filter(|h| !text.contains(**h)).count();
        let unresolved = text.matches("TODO").count()
            + text.matches("TBD").count()
            + text.matches("[ ]").count();

        let subsection = is_subsection(&path);
        if subsection {
            subsection_count += 1;
        } else {
            main_count += 1;
        }

        if missing > 0 || unresolved > 0 {
            failing += 1;
        }

        println!(
            "| {} | {} | {} | {} | {} |",
            path.display(),
            if subsection { "subseção/especial" } else { "nível" },
            text.len(),
            missing,
            unresolved
        );
    }

    println!();
    println!("Níveis/especiais: {}", main_count);
    println!("Subseções/especiais detectadas por nome: {}", subsection_count);
    println!("Arquivos com falha estrutural: {}", failing);
}
