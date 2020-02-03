## Migrations

You can run migrations via refinery. Install it first:

```
cargo install refinery_cli
```

Then you can setup it to your own database:

```
refinery setup
```

It outputs to `refinery.toml` file which is unfortunately not the default one, so rename it to uppercase `Refinery.toml` and run:

```
refinery migrate
```

# Contributing

##Running tests

Running tests is easy:

```$xslt
cargo test
```

##Static analysis

To run static analysis on the code, you can use clippy:

```$xslt
cargo clippy
```

To automatically fix the problems (be careful, as it might lead to code deletion):

```$xslt
cargo fix -Z unstable-options --clippy
```