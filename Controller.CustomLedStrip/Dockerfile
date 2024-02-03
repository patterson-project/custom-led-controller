FROM rust:1.54 as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM python:bullseye

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/app /usr/local/bin
COPY ./requirements.txt /app/
RUN pip install -r requirements.txt

EXPOSE 8000

COPY ./src/ ./

ENTRYPOINT [ "python", "-u", "__main__.py" ]