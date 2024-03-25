FROM python:3.13.0a4-slim-bullseye

WORKDIR /app

# RUN apk add --update --no-cache python3 && ln -sf python3 /usr/bin/python
# RUN apk add py3-pip 
# RUN pip3 install --upgrade pip --break-system-packages

# RUN python3 -m ensurepip

# RUN apt-get update && apt-get install -y python3.9 python3-pip

# Install dependencies
RUN apt-get update && apt-get install -y curl build-essential

# Install Rust
# RUN curl --proto '=https' https://sh.rustup.rs -sSf | sh -s -- -y
ENV RUSTUP_HOME = "/usr/local/rustup"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . "$HOME/.cargo/env"
ENV PATH="/root/.cargo/bin:${PATH}"


COPY ./requirements.txt /app/
# RUN pip install rpi-ws281x==1.1.3 --break-system-packages
RUN pip3 install -r requirements.txt 
# RUN pip3.9 install -r requirements.txt

EXPOSE 8000

COPY ./src/ ./

ENTRYPOINT [ "python", "-u", "__main__.py" ]