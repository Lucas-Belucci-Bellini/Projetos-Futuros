use regex::Regex;
use std::{fs, path::Path};

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

fn main() {
    let levels_dir = Path::new("levels");
    let main_file_re = Regex::new(r"^level-(\d{2})\.md$").expect("regex válida");
    let source_re = Regex::new(r"https://backrooms-wiki\.wikidot\.com/level-[^\s)]+")
        .expect("regex válida");

    let mut found = [false; 100];
    let mut errors = Vec::new();

    let entries = fs::read_dir(levels_dir).expect("diretório levels/ não encontrado");

    for entry in entries.flatten() {
        let file_name = entry.file_name().to_string_lossy().to_string();

        let Some(captures) = main_file_re.captures(&file_name) else {
            continue;
        };

        let number: usize = captures[1].parse().expect("número válido");
        found[number] = true;

        let text = match fs::read_to_string(entry.path()) {
            Ok(v) => v,
            Err(error) => {
                errors.push(format!("{file_name}: falha ao ler Markdown: {error}"));
                continue;
            }
        };

        if !source_re.is_match(&text) {
            errors.push(format!("{file_name}: página oficial ausente"));
        }

        for heading in REQUIRED {
            if !text.contains(heading) {
                errors.push(format!("{file_name}: seção obrigatória ausente: {heading}"));
            }
        }

        if text.contains("TODO") || text.contains("TBD") {
            errors.push(format!("{file_name}: marcador TODO/TBD encontrado"));
        }
    }

    for (level, exists) in found.iter().enumerate() {
        if !exists {
            errors.push(format!("Level {level}: arquivo principal ausente"));
        }
    }

    if errors.is_empty() {
        println!("OK: Levels 0–99 presentes e compatíveis com o schema documental.");
    } else {
        eprintln!("Falhas encontradas:");
        for error in errors {
            eprintln!("- {error}");
        }
        std::process::exit(1);
    }
}
