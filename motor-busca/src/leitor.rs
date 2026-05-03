use std::fs;
use std::path::Path;

pub fn ler_arquivos(caminhos: Vec<String>) -> Vec<(String, String)> {
    let mut conteudos = Vec::new();
    
    for caminho in caminhos {
        let path = Path::new(&caminho);
        
        let extensao = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        
        if extensao != "txt" && extensao != "md" {
            continue;
        }
        
        match fs::read_to_string(&caminho) {
            Ok(conteudo) => {
                conteudos.push((caminho.clone(), conteudo));
            }
            Err(e) => {
                eprintln!("Erro ao ler {}: {}", caminho, e);
            }
        }
    }
    
    conteudos
}
