# Actix web with Diesel using MySQL

## Getting Started
### Install MySQL (macOS)
```
$ brew install mysql
```

### Start Docker
```
$ docker-compose up
```

### Run Migration
```
$ cargo install diesel_cli --no-default-features --features mysql
$ diesel migration run
```

### Start Server
```
$ cargo run
```