FROM rust:1.49-buster

# Add dev user
RUN useradd -ms /bin/bash developer

# Preparing to install tools
RUN apt-get update && apt-get install -y apt-transport-https
