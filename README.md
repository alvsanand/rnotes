# rnotes

Simple Notes service written in Rust

## Development

### Requisites

- Rust nightly

        ``` bash
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        rustup override set nightly-2020-03-01
        rustup component add rls rust-analysis rust-src
        ```

- Cargo
- Docker & Docker Compose

### Use local PostgreSQL

- Start

``` bash
docker-compose up -d
```

- Delete

``` bash
docker-compose down -v
```

## Usage

### Rest API

- Login

``` bash
TOKEN=$(curl -Ss -d '{
    "email": "user_a@email.com",
    "password": "1464acd6765f91fccd3f5bf4f14ebb7ca69f53af91b0a5790c2bba9d8819417b"
}' http://127.0.0.1:8080/auth/login | jq -r '.jwt_token')
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
