
# Smathome web-server example

API for storing location information in DB

## Preparation

create posgtres container:

```sh
just postgres-run
```

or

```sh
docker run -d --name postgres -p 5432:5432 -e POSTGRES_PASSWORD=password postgres:latest

```

setup db using cargo-sqlx:

```sh
sqlx database setup
```

request examples in `request.http` file
