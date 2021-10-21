# ANICAP
Small project to keep track of the chapters and seasons of a TV series or anime


## Requirements

### Dev

- sqlite3-devel (openSuSe)


## Config file

**Anicap** config file must be called `anicap.yml` and be placed on the same file path as *anicap* binary, this config file should be as follows:

```yml
# Ip address of Anicap backend 0.0.0.0 means can accept connections from everywhere
ip_address: 0.0.0.0
# Port where Anicap backend will listen
server_port: 8085
# Log type and level could be something like:
# ERROR, WARN, INFO, DEBUG, TRACE
# or have specified a library like:
# actix_web=DEBUG, actix_web=INFO
log_type: DEBUG
# Database config
database:
  # URL of database for Anicap, should contain:
  # user, pass, ip address, port and db schema
  # - Mysql mysql://user:pass@ip:port/db_name
  # - PostgreSQl mysql://user:pass@ip:port/db_name
  # - SQLite ./anicap.db
  db_url: ./anicap.db
  # Connection pool allow maximum number of connections managed by the pool
  pool_size: 6
# Token config
token:
  # Super secret key for encoding tokens
  jwt_secret: anicap-super-secret-key
  # Duration in minutes
  duration: 60
```

You can find a sample version called `anicap-sample.yml` in this repo.

## Run

Run Anicap Backend

```bash
cargo run
```

## Build

Build Anicap Backend with **debug** compatibility

```bash
cargo build
```

## Endpoints

This has a sample of the current endpoints and how they work

#### register

Request

```
path: /register
method: POST
```

Body

```json
{
  "first_name": "Jon",
  "last_name": "Doe",
  "username": "jd",
  "email": "jd@test.rs",
  "password": "1234abcd"
}
```

#### login

Request

```
path: /login
method: POST
```
Body

```json
{
  "username": "jd",
  "password": "1234abcd"
}
```

#### add_serie

Request

```
path: /serie
method: POST
```

Body

```json
{
	"name": "The Outpost",
	"season": 2,
	"chapter": 1,
	"score": 8.0
}
```

#### get_series

Request

```
path: /serie
method: GET
headers: Bearer token
```

Response

```json
[
  {
    "id": 1,
    "user_id": 1,
    "name": "The Outpost",
    "season": 2,
    "chapter": 0,
    "score": 8.0
  }
]
```

#### update_serie

Request

```
path: /serie/{serie_id}
method: PUT
headers: Bearer token
```

Body

```json
{
	"name": "The Outpost",
	"season": 2,
	"chapter": 2,
	"score": 8.5
}
```
