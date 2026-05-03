use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn exportar_indice(indice: &HashMap<String, Vec<String>>) -> Result<(), std::io::Error> {
    let json_str = serde_json::to_string_pretty(indice)?;
    
    let caminho_saida = Path::new("../compartilhado/saida");
    fs::create_dir_all(caminho_saida)?;
    
    let caminho_arquivo = caminho_saida.join("index.json");
    fs::write(&caminho_arquivo, json_str)?;
    
    Ok(())
}

