from fastapi import APIRouter
from app.servicos.servico_busca import buscar_no_indice

router = APIRouter()

@router.get("/pesquisar")
def pesquisar(q: str):
    #Endpoint principal de busca. Exemplo de uso: /pesquisar?q=meu_arquivo

    dados_encontrados = buscar_no_indice(q)
    return {
        "termo_buscado": q,
        "resultados": dados_encontrados,
        "quantidade": len(dados_encontrados)
    }