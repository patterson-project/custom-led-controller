from fastapi import FastAPI

from src.routes.requests import router as CustomLedStripRequestRouter

app = FastAPI(
    title="Custom Led Strip API",
    description="API to make Custom Led Strip lighting requests",
    docs_url="/docs",
    openapi_url="/docs/openapi.json",
)

app.include_router(
    CustomLedStripRequestRouter, tags=["Custom Led Strip Requests"] 
)

@app.get("/healthz", tags=["Health"])
async def health():
    return "Healthy"