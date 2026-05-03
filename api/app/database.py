import os

# Definindo o nome do arquivo que o Rust vai criar
ARQUIVO_INDICE = "indice.txt"

def ler_dados_do_rust():
    
    #Esta função abre o arquivo gerado pelo motor em Rust, lê cada linha e transforma em uma lista de dicionários que o Python consegue entender.
    
    lista_arquivos = []

    # Proteção: Se o arquivo não existir, a API não trava
    if not os.path.exists(ARQUIVO_INDICE):
        return [{"arquivo": "Aviso", "local": "Aguardando motor Rust...", "tags": ""}]

    # Leitura eficiente linha por linha
    with open(ARQUIVO_INDICE, "r", encoding="utf-8") as arquivo:
        for linha in arquivo:
            caminho_completo = linha.strip()
            
            if caminho_completo:
                # Transforma o caminho bruto em um objeto estruturado
                item = {
                    "arquivo": os.path.basename(caminho_completo),
                    "local": caminho_completo,
                    "tags": "rust_engine"
                }
                lista_arquivos.append(item)
                
    return lista_arquivos