from fastapi import APIRouter
from app.servicos.servico_busca import buscar_no_indice

# Importado o APIRouter para organizar as rotas em arquivos separados
# Importado a função de lógica que foi criano no arquivo de serviço

# Criado o objeto 'router' que será registrado no arquivo principal (main.py)
router = APIRouter()

# Definido que, quando alguém acessar "/pesquisar" via método GET:
@router.get("/pesquisar")
def pesquisar(q: str):
    resultados = buscar_no_indice(q)
    return {"busca": q, "resultados": resultados, "total": len(resultados)}