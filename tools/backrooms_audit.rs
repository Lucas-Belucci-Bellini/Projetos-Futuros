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

fn main() {
    let root = env::args().nth(1).unwrap_or_else(|| "levels".to_string());
    let mut files = Vec::new();
    walk(Path::new(&root), &mut files);

    let mut failures = 0usize;
    for path in files {
        let text = match fs::read_to_string(&path) {
            Ok(v) => v,
            Err(err) => {
                eprintln!("[ERROR] {}: {}", path.display(), err);
                failures += 1;
                continue;
            }
        };

        let missing: Vec<&str> = REQUIRED
            .iter()
            .copied()
            .filter(|heading| !text.contains(heading))
            .collect();

        let unresolved = text.matches("a verificar").count()
            + text.matches("A verificar").count()
            + text.matches("não documentad").count()
            + text.matches("Não documentad").count()
            + text.matches("[ ]").count();

        if !missing.is_empty() || unresolved > 0 {
            println!("{}", path.display());
            if !missing.is_empty() {
                println!("  missing headings: {}", missing.join(", "));
            }
            if unresolved > 0 {
                println!("  unresolved markers: {}", unresolved);
            }
            failures += 1;
        }
    }

    println!();
    println!("Audited Markdown files: {}", files.len());
    println!("Files requiring second-pass review: {}", failures);

    if failures > 0 {
        std::process::exit(2);
    }
}
