FROM ubuntu:24.04

# Install build tools
RUN apt-get update && \
    apt-get install -y cmake g++ make && \
    rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /workspace
