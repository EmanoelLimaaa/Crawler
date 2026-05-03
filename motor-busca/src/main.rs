mod varredura;
use std::env;
use std::path::PathBuf;
use varredura::listar_arquivos;

#[tokio::main]
async fn main() {
    let default_path = "../compartilhado/dados";
    let arg_path = env::args().nth(1);
    let path = arg_path.as_deref().unwrap_or(default_path);
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let absolute_path = cwd.join(path);

    println!("Motor de busca iniciado!");
    println!("Diretório de trabalho: {}", cwd.display());
    println!("Caminho usado: {}", path);
    println!("Caminho absoluto: {}", absolute_path.display());

    let arquivos = listar_arquivos(path);
    if arquivos.is_empty() {
        println!("Nenhum arquivo encontrado em {}.", path);
        if !PathBuf::from(path).exists() {
            println!("O caminho não existe.");
        }
    } else {
        println!("Arquivos em {} ({}):", path, arquivos.len());
        for arquivo in &arquivos {
            println!("  - {}", arquivo);
        }
    }
}
