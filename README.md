# rnotes

RNotes is a simple notes service written in Rust that has been as a "graduation" project for my learning of the fantastic [Rust language](https://rust-lang.org/).

It is consist of three modules:

- rnotes_core: which contains de DB and API models.
- rnotes_server: the API server that process the request of the notes services.
- rnotes_cli: a command line client for the rnotes services.

## Development

### Requisites

- Rust nightly

    ``` bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    rustup toolchain install nightly
    rustup override set nightly
    rustup component add rls rust-analysis rust-src
    ```

- Cargo
- Docker & Docker Compose

### Building

- Local build

    ``` bash
    make build
    ```

- Docker build

    ``` bash
    make docker-build
    ```

### Using local PostgreSQL

- Start

    ``` bash
    docker_compose_up
    ```

- Delete

    ``` bash
    docker_compose_down
    ```

## Usage

### Launch rnotes_server

1. Docker build

    ``` bash
    docker-compose up -d
    ```

1. Start servers

    ``` bash
    docker-compose up -d
    ```

### Launch requests

#### Run rnotes_cli

``` bash
docker exec -it $(docker ps -aq -f name=rnotes) /rnotes_cli
```

#### Rest API

- Login

    ``` bash
    TOKEN=$(curl -Ss -d '{
        "email": "user_a@email.com",
        "password": "1464ACD6765F91FCCD3F5BF4F14EBB7CA69F53AF91B0A5790C2BBA9D8819417B"
    }'  --header "Content-Type: application/json" http://127.0.0.1:8080/auth/login | jq -r '.jwt_token')
    ```

- Get all notes of the user

    ``` bash
    curl -vSs -H "Authorization: Bearer ${TOKEN}" http://127.0.0.1:8080/notes
    ```

- Create a note

    ``` bash
    curl -Ss -v -H "Authorization: Bearer ${TOKEN}" --header "Content-Type: application/json" -d '{
    "category_id": null,
    "title": "note_XXX_user_a",
    "data": "some_text_note_XXX_user_a"
    }' http://127.0.0.1:8080/notes | jq
    ```

- Get a note

    ``` bash
    curl -Ss -v -H "Authorization: Bearer ${TOKEN}" http://127.0.0.1:8080/notes/{NOTE_ID} | jq
    ```

- Update a note

    ``` bash
    curl -Ss -v -X PUT -H "Authorization: Bearer ${TOKEN}" --header "Content-Type: application/json" -d '{
    "category_id": null,
    "title": "note_XXX_user_a_other",
    "data": "some_text_note_XXX_user_a_other"
    }' http://127.0.0.1:8080/notes/{NOTE_ID} | jq
    ```
