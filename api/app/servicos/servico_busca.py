from app.database import ler_dados_do_rust

def buscar_no_indice(termo: str):
    indice_completo = ler_dados_do_rust()
    termo_busca = termo.lower() # O Rust salva tudo em minúsculo[cite: 2]
    
    resultados = []

    # Se a palavra existe no índice que o Rust criou
    if termo_busca in indice_completo:
        caminhos = indice_completo[termo_busca]
        
        for caminho in caminhos:
            import os
            resultados.append({
                "arquivo": os.path.basename(caminho),
                "local": caminho,
                "tags": "indexado_pelo_rust"
            })
            
    return resultados