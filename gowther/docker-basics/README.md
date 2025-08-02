- group the RUN commands can reduce the image size because each RUN command creates a layer

  ```diff
    FROM ubuntu
  - RUN apt update
  - RUN apt install -y python3 python3-pip
  - RUN pip install flask
  - RUN apt autoremove --purge -y python3-pip
  - RUN rm -f /var/lib/lists
  + RUN apt update && \
  +     apt install -y python3 python3-pip && \
  +     pip install flask && \
  +     apt autoremove --purge -y python3-pip && \
  +     rm -f /var/lib/lists
  ```

- when one layer is changes, it will invalidate the following layers

  ```diff
    FROM node
  - COPY . .
  + COPY package.json package-lock.json .
    RUN npm install
  + COPY src ./src
    EXPOSE 3000
    CMD ["node", "src/index.js"]
  ```

- using `--link` indicates a layer is completely independent of other layers (only works for COPY and ADD)

# Resource

- https://www.docker.com/learning-paths/learning-basics/
