# <img src="https://camweb-www.website-eu-central-1.linodeobjects.com/assets/camweb.svg" style="width:40px;margin-bottom:-10px;display: inline-block;"/> WIP: CamWeb 

Simple web app to view and manage images from an ESP cam 

This is a simple project to play a bit with ESP32 cam embbeded rust and shuttle.rs

## Dependencies 

### External resources
- S3 Buckets:
  - One bucket with named: `<x>` 
  - One bucket as webServer with named: `<x>-www`
  - Write access to this 2 bucket 
- Shuttle.rs account

### Commands

- `s3cmd`


## Getting started (UI)

Set values in  `server/Secrets.toml` 

```sh
cd ui
cargo install trunk
trunk serve
bash ./deploy.sh
```

## Getting started (API server)

> depends on UI

```sh
cd server
cargo install cargo-shuttle
cargo shuttle login
cargo shuttle project new
# Comment Secrets.toml in .gitignore (issue with shuttle atm)
cargo shuttle deploy --allow-dirty
# Uncomment Secrets.toml in .gitignore
```