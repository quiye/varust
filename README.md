# varust

## install

```sh
cargo build
```

## usage

```sh
$ ./target/debug/varust prod.environment sample.yaml
USER=prod
URL=http://prod-env.com
$ ./target/debug/varust dev.environment sample.yaml
USER=dev
URL=http://dev-env.com
```