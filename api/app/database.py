import os
import json

# Caminho exato onde o Rust salvou o arquivo[cite: 1]
ARQUIVO_INDICE = "../compartilhado/saida/index.json"

def ler_dados_do_rust():
    if not os.path.exists(ARQUIVO_INDICE):
        return {} # Retorna dicionário vazio se não achar o arquivo

    with open(ARQUIVO_INDICE, "r", encoding="utf-8") as f:
        return json.load(f) # Transforma o JSON em um dicionário do Python