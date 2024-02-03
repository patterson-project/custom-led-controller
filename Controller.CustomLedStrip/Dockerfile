FROM python:3.9-buster

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y curl build-essential

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY ./requirements.txt /app/
RUN pip install -r requirements.txt

EXPOSE 8000

COPY ./src/ ./

ENTRYPOINT [ "python", "-u", "__main__.py" ]