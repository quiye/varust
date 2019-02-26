# Varust

A simple CLI tool to extract environment variables from yaml file.

## Installing

```sh
cargo install --git https://github.com/quiye/varust.git
```

## Usage

[sample.yaml](sample.yaml) is a simple structured yaml file.

```sh
$ cat sample.yaml
# shared variables
shared:
  environment:
    REPLICATION: 3
    CONSISTENCY: ONE
# variables for production
prod:
  environment:
    URL: https://prod-env.com
    PORT: 443
    REPLICATION: 5
    CONSISTENCY: ALL
# variables for develop
dev:
  environment:
    URL: http://dev-env.com
    PORT: 80
    CONSISTENCY: TWO
```

From above file, we can extract productional environment variables by varust.

```sh
$ varust prod.environment sample.yaml
REPLICATION=5
CONSISTENCY=ALL
URL=https://prod-env.com
PORT=443
```

On the other hand, we can extract variables for develop environment.

```sh
$ varust dev.environment sample.yaml
PORT=80
URL=http://dev-env.com
CONSISTENCY=TWO
```

### Options

#### `-o (--on)`

By using this option, we can expand `dev.environment` on a shared setting `shared.environment`.

Look below !

```sh
$ varust dev.environment --on shared.environment sample.yaml
PORT=80
REPLICATION=3
CONSISTENCY=TWO
URL=http://dev-env.com
```