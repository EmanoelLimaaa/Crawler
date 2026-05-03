# Importando a função que foi criado no database.py
from app.database import ler_dados_do_rust

def buscar_no_indice(termo: str):

    #Pega os dados brutos do database e filtra de acordo com o que o usuário digitou na API.

    # Buscar a "lista bruta" que veio do arquivo do Rust
    dados_brutos = ler_dados_do_rust()
    # Criar uma lista para os resultados filtrados
    resultados = []
    
    for item in dados_brutos:
        # Se o termo que o usuário digitou estiver no nome do arquivo
        if termo.lower() in item["arquivo"].lower():
            resultados.append(item)
            
    return resultados