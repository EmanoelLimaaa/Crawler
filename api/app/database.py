import os

# Definindo o nome do arquivo que o Rust vai criar
ARQUIVO_INDICE = "indice.txt"

def ler_dados_do_rust():
    
    #Esta função abre o arquivo gerado pelo motor em Rust, lê cada linha e transforma em uma lista de dicionários que o Python consegue entender.
    
    lista_arquivos = []

    # Verificr se o arquivo do Rust já existe para não dar erro
    if not os.path.exists(ARQUIVO_INDICE):
        return [{"arquivo": "Aviso", "local": "O motor em Rust ainda não gerou o índice.", "tags": ""}]

    # Abrir o arquivo para leitura ('r' de read)
    with open(ARQUIVO_INDICE, "r", encoding="utf-8") as arquivo:
        for linha in arquivo:

            linha_limpa = linha.strip()
            
            # Se a linha não estiver vazia, adicionar ao dados
            if linha_limpa:
                # Criando um formato de "objeto" para cada linha encontrada
                item = {
                    "arquivo": os.path.basename(linha_limpa), # Pega só o nome do arquivo
                    "local": linha_limpa,                     # O caminho completo
                    "tags": "encontrado pelo rust"            # Uma etiqueta simples
                }
                lista_arquivos.append(item)
                
    return lista_arquivos