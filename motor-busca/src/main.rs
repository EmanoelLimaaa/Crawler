mod varredura;
mod leitor;
mod indexador;
mod exportador;
mod busca;
use std::env;
use std::path::PathBuf;
use varredura::listar_arquivos;
use leitor::ler_arquivos;
use indexador::criar_indice;

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
        
        println!("\n--- Lendo arquivos ---");
        let conteudos = ler_arquivos(arquivos);
        
        if conteudos.is_empty() {
            println!("Nenhum arquivo .txt ou .md encontrado.");
        } else {
            let indice = criar_indice(conteudos.clone());

            if let Err(e) = exportador::exportar_indice(&indice) {
                eprintln!("Erro ao exportar índice: {}", e);
            } else {
                println!("Índice exportado para ../compartilhado/saida/index.json");
            }

            println!("Arquivos lidos: {}\n", conteudos.len());
            for (arquivo, conteudo) in &conteudos {
                println!("📄 {}", arquivo);
                println!("{}", "─".repeat(60));
                println!("{}\n", conteudo);
            }

            println!("--- Índice invertido ---");
            let mut palavras: Vec<_> = indice.keys().collect();
            palavras.sort();
            for palavra in palavras {
                let arquivos = &indice[palavra];
                println!("{} => {}", palavra, arquivos.join(", "));
            }

            println!("\n--- Teste de Busca ---");
            let resultados = busca::buscar("teste", "../compartilhado/saida/index.json");
            println!("Busca 'teste': {:?}", resultados);
        }
    }
}
