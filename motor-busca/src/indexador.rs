use std::collections::{HashMap, HashSet};

pub fn criar_indice(arquivos: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut indice: HashMap<String, Vec<String>> = HashMap::new();

    for (arquivo, conteudo) in arquivos {
        let mut palavras_vistas: HashSet<String> = HashSet::new();

        for palavra in conteudo.split_whitespace() {
            let palavra_limpa = palavra
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_lowercase();

            if palavra_limpa.is_empty() {
                continue;
            }

            if palavras_vistas.contains(&palavra_limpa) {
                continue;
            }

            palavras_vistas.insert(palavra_limpa.clone());
            let arquivos_list = indice.entry(palavra_limpa).or_default();
            if !arquivos_list.contains(&arquivo) {
                arquivos_list.push(arquivo.clone());
            }
        }
    }

    indice
}
