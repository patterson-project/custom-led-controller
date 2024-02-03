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
- Clone this repository and `cd custom-led-controller/Controller.CustomLedStrip` into it
- Run `docker compose up`
  - If it's the first time running this, it will take a while to install all the neccessary components

## Usage
### Starting the container:
- Run `docker compose up` in `custom-led-controller/Controller.CustomLedStrip`

### Stopping the container:

### Debugging / Status of container:
