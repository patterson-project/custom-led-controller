FROM python:bullseye

WORKDIR /app

RUN apt-get update && apt-get install -y curl build-essential
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY ./requirements.txt /app/
RUN pip install -r requirements.txt

EXPOSE 8000

COPY ./src/ ./

ENTRYPOINT [ "python", "-u", "__main__.py" ]