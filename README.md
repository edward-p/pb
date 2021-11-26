# pb

# Build

``` bash
$ cargo build --release
```

# Environment Variables

- `PB_DATA`: /some/path (Default: `./pb_data`)
- `PB_SITE`: Url of your site. (Default: `http://localhost:8000`)
- `ROCKET_PORT`: listening port (Default: `8000`)
- `ROCKET_TEMPLATE_DIR`: dir to lookup templates (Default: `./templates`)

see also https://rocket.rs/v0.5-rc/guide/configuration/

# Run

``` bash
$ PB_DATA="/some/path" PB_SITE="https://pb.example.com" ROCKET_TEMPLATE_DIR=./templates ./pb
```
