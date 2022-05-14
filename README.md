# pb

# Build

```bash
$ cargo build --release
```

# Environment Variables

## PB Custom

- `PB_DATA`: /some/path (Default: `./pb_data`)
- `PB_SITE`: Url of your site. (Default: `http://localhost:8000`)
- `STATIC_FILE_PATH`: dir to static resources (where `index.html` is)

## Rocket defined:

- `ROCKET_PORT`: listening port (Default: `8000`)
- `ROCKET_TEMPLATE_DIR`: dir to lookup templates (Default: `./templates`)

see also https://rocket.rs/v0.5-rc/guide/configuration/

# Rocket.toml

Configs can also be applied using `Rocket.toml` but environment variables have higher priority.
The path of `Rocket.toml` can be changed using `ROCKET_CONFIG` environment variable

# Run

```bash
$ PB_DATA="/some/path" PB_SITE="https://pb.example.com" ROCKET_TEMPLATE_DIR=./templates STATIC_FILE_PATH="./static" ./pb
```
