from app.database import ler_dados_do_rust

def buscar_no_indice(termo: str):
    # Recebe o termo de busca, solicita os dados ao database e realiza o filtro por nome de arquivo.

    dados_brutos = ler_dados_do_rust()
    resultados = []
    
    # Normaliza a busca para minúsculo para evitar erros de digitação
    termo_busca = termo.lower()
    
    for item in dados_brutos:
        if termo_busca in item["arquivo"].lower():
            resultados.append(item)
            
    return resultados