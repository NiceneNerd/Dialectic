# Ghost Coding Challenge Application

A simple comment interface for an imaginary web article.

## Requirements

- MySQL 8
- A recent Node version
- A recent Rust version

## Build and Run

To run as-is, first start a MySQL server at `root:root@127.0.0.1:3307` and
create a database named `dialectic`. You can do so using the following Docker
compose file:

```yaml
version: '3'
services:
    mysql:
        image: mysql:latest
        container_name: mysql8
        volumes:
            - ./data:/var/lib/mysql
            - ./config/my.cnf:/etc/mysql/my.cnf
            - ./logs/mysql.log:/var/log/mysql.log
        ports:
            - "3307:3306"
        environment:
            MYSQL_ROOT_USER: root
            MYSQL_ROOT_PASSWORD: root
            MYSQL_DATABASE: dialectic
```

Build and run with `cargo run`. A server will start at `localhost:8000`.