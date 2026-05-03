use std::fs;
use walkdir::WalkDir;

pub fn listar_arquivos(path: &str) -> Vec<String> {
    let mut arquivos = Vec::new();
    if fs::metadata(path).is_ok() {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Some(p) = entry.path().to_str() {
                    arquivos.push(p.to_string());
                }
            }
        }
    }
    arquivos
}
