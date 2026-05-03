use serde_json;
use std::collections::HashMap;
use std::fs;

pub fn buscar(palavra: &str, caminho_indice: &str) -> Vec<String> {
    if let Ok(conteudo) = fs::read_to_string(caminho_indice) {
        match serde_json::from_str::<HashMap<String, Vec<String>>>(&conteudo) {
            Ok(indice) => match indice.get(palavra.to_lowercase().as_str()) {
                Some(arquivos) => arquivos.clone(),
                None => vec![],
            },
            Err(_) => {
                eprintln!("Erro ao parsear índice JSON");
                vec![]
            }
        }
    } else {
        eprintln!("Erro ao ler índice: {}", caminho_indice);
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_busca_existe() {
        let resultados = buscar("teste", "../compartilhado/saida/index.json");
        assert!(!resultados.is_empty());
    }

    #[test]
    fn test_busca_inexistente() {
        let resultados = buscar("xyz123", "../compartilhado/saida/index.json");
        assert_eq!(resultados.len(), 0);
    }
}

