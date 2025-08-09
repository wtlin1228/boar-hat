# Image deep dive and build

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

# Docker compose

## Docker CLI

- The CLI binary is called `docker`
- The CLI uses a plugin architecture
  - `compose` is a plugin
- Others can be installed
  - `$ docker plugin ls` shows all 3rd party plugins to docker
  - `$ ls -al ~/.docker/cli-plugins` shows all the plugins shipped with docker

## Convert run to compose

```dockerfile
docker run -d -p 3306:3306 \
  -e MYSQL_ROOT_PASSWORD=superSecret \
  -e MYSQL_DATABASE=memes \
  -v mysql-data:/var/lib/mysql \
  mysql:8.2.0
```

```dockerfile
services:
  database:
    image: mysql:8.2.0
    ports:
      - target: 3306
        published: 3306
    volumes:
      - mysql-data:/var/lib/mysql
    environment:
      MYSQL_ROOT_PASSWORD: superSecret
      MYSQL_DATABASE: memes
volumes:
  mysql-data:
```

Note that volume can be shared between different services.

## Advanced networking

- You can build more sophisticated networks to control what containers can talk to each other
- Aliases provide the ability to customize the DNS names

```

  ┌─────────────────────────┐       ┌─────────────────────────┐
  │                         │       │                         │
  │       ┌─────────┐       │       │                         │
  │       │         │   ┌───────────────┐                     │
  │       │  proxy  │   │               │                     │
  │       │         │   │      api      │                     │
  │       └─────────┘   │               │                     │
  │       ┌─────────┐   └───────────────┘   ┌───────────┐     │
  │       │         │       │       │       │           │     │
  │       │   app   │       │       │       │    db     │     │
  │       │         │       │       │       │           │     │
  │       └─────────┘       │       │       └───────────┘     │
  │                         │       │                         │
  │      proxy network      │       │      backend network    │
  │                         │       │                         │
  └─────────────────────────┘       └─────────────────────────┘
```

```dockerfile
services:
  proxy:
    ...
    networks:
      - proxy
  app:
    ...
    networks:
      - proxy
  api:
    ...
    networks:
      proxy:
      backend:
  db:
    ...
    networks:
      backend:
        aliases:
          - database
networks:
  proxy:
  backend:
```

## Adding a dev service

- Build a custom image and use it for the service using the `build` field
  - If you're using multi-stage builds, you can target a specific stage with `build.target`
- Mount in source code using a bind mount
  - As you make changes to the files on your host, they'll be updated in your host

```dockerfile
services:
  python:
    build:
      context: ./
      target: dev
    ports:
      8000:5000
    volumes:
      - ./app:/usr/local/app
  mysql:
    image: mysql:8.2.0
    ...
  phpmyadmin:
    image: phpmyadmin:5.2
    ...
```

## Adding environment variables

- Hard-coding parameters is a bad idea
- Let's replace them with environment variables

  - We can bring them in with a .env (or different) file

    ```
    DBNAME=memes
    ```

  - We can pass them on the command line

    ```
    docker compose run -e DBNAME=memes
    ```

  - We can get them from the shell

    ```
    export DBNAME="memes"
    ```

- Use `docker compose config` to validate

```dockerfile
services:
  mysql:
    environment:
      MYSQL_DATABASE: ${DBNAME}
```

## Adding secrets

- Using Environment Variables for secrets can also be a bad idea
  - Available to all processes
  - Can show up in logs
- Let's use secrets files instead

```dockerfile
services:
  mysql:
    image: mysql:8.2.0
    environment:
      MYSQL_ROOT_PASSWORD_FILE: /run/secrets/db_root_password
      MYSQL_PASSWORD_FILE: /run/secrets/db_password
      MYSQL_DATABASE: ${DBNAME}
    secrets:
      - db_root_password
      - db_password
secrets:
  db_password:
    file: db_password.txt
  db_root_password:
    file: db_root_password.txt
```

## Compose watch

- Watch for file changes and perform actions
- Sync files from host to container
- Auto rebuild images and restart the containers
- Available with the new `docker compose watch` command

```dockerfile
services:
  ...
  client:
    ...
    develop:
      watch:
        - path: ./client/src
          action: sync
          target: /usr/src/app/src
        - path: ./client/yarn.lock
          action: rebuild
...
```

## Compose merge

Docker compose can merge a set of compose files to create a composite file

Example:

```
docker compose -f compose.yml -f overlay.yml up
```

- Compose merges all the files in the order they are specified
- All paths are relative to the base file (the first one appears after -f)

## Compose include

- Include Compose files from other locations for composable apps
  - Reference local files
  - Pull from remote git repos
- Each compose file can reference their own relative paths

```dockerfile
include:
  - another-compose.yml
  - git@github.com:namespace/repo.git
services:
  ...
volumes:
  ...
```

## Compose debugging

Debugging advanced compose functionality can be tricky

Use the `config` option to "parse, resolve and render" the final compose file

Example:

```
docker compose -f compose.yml -f overlay.yml config
```

This will output the final YAML file that compose would try to run

# Resource

- https://www.docker.com/learning-paths/learning-basics/
