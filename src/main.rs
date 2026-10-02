use regex::Regex;
use std::{fs, path::Path};

fn main() {
    let levels_dir = Path::new("levels");
    let filename_re = Regex::new(r"^level-(\d{2})\.md$").expect("regex válida");
    let source_re = Regex::new(r"https://backrooms-wiki\.wikidot\.com/level-(\d+)").expect("regex válida");

    let mut found = [false; 100];
    let mut errors = Vec::new();

    let entries = fs::read_dir(levels_dir).expect("diretório levels/ não encontrado");

    for entry in entries.flatten() {
        let file_name = entry.file_name().to_string_lossy().to_string();

        let Some(captures) = filename_re.captures(&file_name) else {
            continue;
        };

        let number: usize = captures[1].parse().expect("número válido");
        if number > 99 {
            continue;
        }

        found[number] = true;

        let text = fs::read_to_string(entry.path()).expect("Markdown legível");

        match source_re.captures(&text) {
            Some(source) if source[1].parse::<usize>().ok() == Some(number) => {}
            _ => errors.push(format!(
                "{}: página oficial ausente ou não corresponde ao número do arquivo",
                file_name
            )),
        }

        if !text.contains("## Metadados") {
            errors.push(format!("{}: seção de metadados ausente", file_name));
        }

        if !text.contains("## Atribuição") {
            errors.push(format!("{}: seção de atribuição ausente", file_name));
        }
    }

    for (level, exists) in found.iter().enumerate() {
        if !exists {
            errors.push(format!("Level {}: arquivo ausente", level));
        }
    }

    if errors.is_empty() {
        println!("OK: Levels 0–99 presentes, com URLs e estrutura mínima válidas.");
    } else {
        eprintln!("Falhas encontradas:");
        for error in errors {
            eprintln!("- {}", error);
        }
        std::process::exit(1);
    }
}
