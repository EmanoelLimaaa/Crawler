## Definimos uma função que recebe um 'termo' (o que o usuário digitou)
def buscar_no_indice(termo: str):
    # ---Esta função simula a busca em um banco de dados. No futuro, ela vai abrir o arquivo gerado pelo Rust.---

    # Crindo uma lista de dicionários para simular os arquivos encontrados no Rust

    banco_fake = [
        {"arquivo": "projeto_rust.rs", "local": "/home/docs", "tags": "codigo, engine"},
        {"arquivo": "precos_gpu.csv", "local": "/home/downloads", "tags": "crawler, precos"},
    ]



    # Filtra os resultados que contém o termo pesquisado
    resultados = [item for item in banco_fake if termo.lower() in item["arquivo"].lower()]
    # Devolvemos a lista preenchida (ou vazia, se nada for encontrado)
    return resultados
