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

- we can have multi-stage build

  ```dockerfile
  From <image1> AS stage1
  COPY ...
  RUN ...
  ...
  FROM <image2> AS stage2
  COPY --from=stage1 /path/in/stage1 /path/in/stage2
  ```

- using mounts with RUN

  - mounts allow you to modify the filesystem when building
  - supports several types
    - bind - share a host directory into the build
    - cache - temporary directory for compile private files
    - ssh - allow build container to access SSH key via SSH agents

  ```dockerfile
  FROM node
  COPY package.json yarn.lock ./
  RUN --mount=type=secret,id=npm-creds,target=/root/.npmrc \
      --mount=type=cache,id=yarn,target=/root/.yarn \
      yarn install
  ```

- HEREDOC support

  ```dockerfile
  FROM debian
  RUN <<EOT bash
    set -ex
    apt-get update
    apt-get install -y vim
  EOT
  ```

- multi-arch features

  ```dockerfile
  FROM --platform=$BUILDPLATFORM node AS build
  WORKDIR /usr/local/app
  COPY package.json yarn.lock ./
  RUN yarn install
  COPY public ./public
  COPY src ./src
  RUN yarn build

  FROM --platform=$TARGETPLATFORM nginx AS final
  COPY --from=build /usr/local/app/build /usr/share/nginx/html
  ```

  ```bash
  $ docker buildx build \
      -t react-app \
      --platform=linux/amd64,linux/arm64 .
  ```

# Resource

- https://www.docker.com/learning-paths/learning-basics/
