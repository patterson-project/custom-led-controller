FROM tiangolo/uvicorn-gunicorn-fastapi:python3.9

COPY ./requirements.txt /app/

RUN pip install --no-cache-dir --upgrade -r requirements.txt

EXPOSE 8000

COPY ./src ./

ENTRYPOINT [ "python", "-u", "__main__.py" ]