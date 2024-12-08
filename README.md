# Custom-Led-Controller

A custom LED controller written for the WS281x family of controllable LED strips. Uses a fast-api to process and unpack lighting requests

## Requirements:

- Docker
- Docker Compose
- Python
- Raspberry Pi

## Installation

### Docker:

- Please refer to the installation method [here](https://docs.docker.com/engine/install/raspberry-pi-os/#next-steps) **32-bit**
- You can confirm your installation with `systemctl status docker` which should state that docker is active and runnign

### Repository:

- Clone this repository and `cd custom-led-controller`
- Run `docker compose up`
  - If it's the first time running this, it will take a while to install all the neccessary components

## Usage

### Starting the container:

- Run `docker compose up` in `custom-led-controller`

### Reinitializing the container:

- Run `docker compose build` in `custom-led-controller`

### Stopping the container:

- Ctrl-C or `docker compose stop` to stop the service

### Debugging / Status of container:

- `docker compose ps` to see the status of your container
