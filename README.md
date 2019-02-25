# varust

A simple CLI tool to extract environment variables from yaml file.

## install

```sh
cargo install --git https://github.com/quiye/varust.git
```

## usage

[sample.yaml](sample.yaml) is a simple structured yaml file.

```sh
$ cat sample.yaml
prod:
  environment:
    USER: prod
    URL: https://prod-env.com
    PORT: 443
dev:
  environment:
    USER: dev
    URL: http://dev-env.com
    PORT: 80
```

From above file, we can extract productional environment variables by varust.

```sh
$ varust prod.environment sample.yaml
USER=prod
URL=https://prod-env.com
PORT=443
```

On the other hand, we can extract variables for develop environment.

```sh
$ varust dev.environment sample.yaml
URL=http://dev-env.com
PORT=80
USER=dev
```
