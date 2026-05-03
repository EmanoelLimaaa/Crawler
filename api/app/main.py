from fastapi import FastAPI
from app.rotas import busca

app = FastAPI(title="Motor de Busca V1")

# Registrando as rotas do arquivo busca.py no aplicativo principal
# Isso mantém o código limpo: rotas de busca num lugar, rotas de usuário em outro, etc.
app.include_router(busca.router)

# Criando uma rota raiz apenas para testar se o servidor está ligado
@app.get("/")
def home():
    return {"mensagem": "API de Busca Online. Use /docs para testar."}