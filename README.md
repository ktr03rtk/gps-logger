# Overview

This application communicates with the gpsd server via TCP protocol to obtain location information and save it to a file.

# Usage

- Run the `docker-compose up -d` command to start the container. It build the Rust code and automatically execute the generated binary.

- The file with the location information is generated in the `data/` directory.

- Run the `docker-compose down` to stop collecting the data.

# Supported OS

Only tested with Raspberry Pi OS on Raspberry Pi 4B
