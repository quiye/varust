# varust

## install

```sh
cargo install --git https://github.com/quiye/varust.git
```

## usage

```sh
$ varust prod.environment sample.yaml
USER=prod
URL=http://prod-env.com
$ varust dev.environment sample.yaml
USER=dev
URL=http://dev-env.com
```
