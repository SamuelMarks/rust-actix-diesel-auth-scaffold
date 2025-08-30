rust-actix-diesel-auth-scaffold
===============================
[![.github/workflows/test.yml](https://github.com/SamuelMarks/rust-actix-diesel-auth-scaffold/actions/workflows/test.yml/badge.svg)](https://github.com/SamuelMarks/rust-actix-diesel-auth-scaffold/actions/workflows/test.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT%20OR%20CC0--1.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Simple baseline scaffold to get you started using actix + diesel with a custom OAuth2 implementation.

For example runnable server, see repository: https://github.com/SamuelMarks/serve-actix-diesel-auth-scaffold

## OAuth2 supported flows

| Grant Flow                            | Supported |
|---------------------------------------|:---------:|
| Resource Owner Password Credentials   | ✅        |
| Authorization Code                    | ⚠️*      |
| Client Credentials                    | ❌        |
| Refresh Token                         | ✅        |
| Implicit                              | ❌        |

## Roadmap

- [ ] Make redis optional hidden behind feature (make it default on)
- [ ] Add more tests
- [ ] Hide each grant flow behind features
- [ ] Add grant flow Authorization Code
- [ ] Add grant flow Client Credentials
- [ ] Add grant flow Implicit

## Path Table

| Method                                                                 | Path                        | Description                        |
|------------------------------------------------------------------------|-----------------------------|------------------------------------|
| POST                                                                   | [/api/token](#postapitoken) | Generate a token for a grant flow. 
 Implements https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.3 |

## Reference Table

| Name         | Path                                                                        | Description |
|--------------|-----------------------------------------------------------------------------|-------------|
| GrantType    | [#/components/schemas/GrantType](#componentsschemasgranttype)               |             |
| TokenRequest | [#/components/schemas/TokenRequest](#componentsschemastokenrequest)         |             |
| password     | [#/components/securitySchemes/password](#componentssecurityschemespassword) |             |

## Path Details

***

### [POST]/api/token

- Summary  
  Generate a token for a grant flow.  
  Implements https://datatracker.ietf.org/doc/html/rfc6749#section-4.1.3

#### RequestBody

- application/x-www-form-urlencoded

```ts
{
  // optional client ID (as used, for example, in RFC6749's non password non refresh grant flow)
  client_id?: string | null
  // optional client secret (as used, e.g., in RFC6749's non (password|refresh) grant flow)
  client_secret?: string | null
  grant_type: enum[password, authorization_code, client_credentials, refresh_token, invalid]
  // optional password (as used, for example, in RFC6749's password grant flow)
  password?: string | null
  // optional refresh token (as used, for example, in RFC6749's refresh grant flow)
  refresh_token?: string | null
  // optional username (as used, for example, in RFC6749's password grant flow)
  username?: string | null
}
```

#### Responses

- 200 Token created

- 400 Unauthorized User

- 404 Not Found User

- 500 Bad Request

## References

### #/components/schemas/GrantType

```ts
{
  "type": "string",
  "enum": [
    "password",
    "authorization_code",
    "client_credentials",
    "refresh_token",
    "invalid"
  ]
}
```

### #/components/schemas/TokenRequest

```ts
{
  // optional client ID (as used, for example, in RFC6749's non password non refresh grant flow)
  client_id?: string | null
  // optional client secret (as used, e.g., in RFC6749's non (password|refresh) grant flow)
  client_secret?: string | null
  grant_type: enum[password, authorization_code, client_credentials, refresh_token, invalid]
  // optional password (as used, for example, in RFC6749's password grant flow)
  password?: string | null
  // optional refresh token (as used, for example, in RFC6749's refresh grant flow)
  refresh_token?: string | null
  // optional username (as used, for example, in RFC6749's password grant flow)
  username?: string | null
}
```

### #/components/securitySchemes/password

```ts
{
  "type": "oauth2",
  "flows": {
    "password": {
      "tokenUrl": "/api/token",
      "scopes": {}
    }
  }
}
```

---

## Docker usage

Install Docker, and then run the following, which will run `cargo test` with Valkey and PostgreSQL from Docker:

```sh
$ docker compose up
````

NOTE: You may need to configure this for your architecture first, for example:

```sh
$ docker compose build --build-arg ARCH_VARIANT='amd64' \
                       --build-arg ARCH='x86_64'
$ docker compose up
```

Or to work with just one image and provide your own database and redis:

```sh
$ docker build -f 'debian.Dockerfile' -t "${PWD##*/}"':latest' .
$ docker run -e DATABASE_URL="$RDBMS_URI" \
             -e REDIS_URL='localhost:6379' \
             -p '3000:3000' \
             --name 'cargo_test_lib' \
             "${PWD##*/}"
```

## Native Usage

Install Rust, `git`, and ensure you have your PostgreSQL and Redis/Valkey services setup.

### PostgreSQL

One way to install PostgreSQL is with my cross-platform https://github.com/SamuelMarks/libscript:

```sh
$ [ -d /tmp/libscript ] || git clone --depth=1 --single-branch https://github.com/SamuelMarks/libscript /tmp/libscript
$ env -i HOME="$HOME" \
         PATH="$PATH" \
         POSTGRES_USER='rest_user' \
         POSTGRES_SERVICE_PASSWORD='addGoodPasswordhere' \
         POSTGRES_PASSWORD='rest_pass' \
         POSTGRES_HOST='localhost' \
         POSTGRES_DB='rest_db' \
         '/tmp/libscript/_lib/_storage/postgres/setup.sh'
```

(on Windows use `set` and `libscript\_lib\_storage\postgres\setup.cmd`)

### Valkey (Redis-compatible)

One way to install the Redis-compatible Valkey is with my cross-platform https://github.com/SamuelMarks/libscript:

```sh
$ [ -d libscript ] || git clone --depth=1 --single-branch https://github.com/SamuelMarks/libscript
$ env -i HOME="$HOME" \
         PATH="$PATH" \
         "$(pwd)"'/libscript/_lib/_storage/valkey/setup.sh'
```

(on Windows use Garnet: https://github.com/microsoft/garnet)

### Environment setup

Add an `.env` file or otherwise add these environment variables; replacing connection strings with what you use:

    DATABASE_URL=postgres://rest_user:rest_pass@localhost/rest_db
    REDIS_URL=redis://127.0.0.1/

### Main entrypoint

In your own project, add dependencies to your Cargo.toml with
`cargo add --git https://github.com/SamuelMarks/rust-actix-diesel-auth-scaffold` or by manually editing your
`Cargo.toml` like so:

```toml
[dependencies]
rust-actix-diesel-auth-scaffold = { git = "https://github.com/SamuelMarks/rust-actix-diesel-auth-scaffold", version = "0.0.1" }
```

Then write a `main.rs` like: https://github.com/SamuelMarks/serve-actix-diesel-auth-scaffold/blob/master/src/main.rs

### Test
```sh
$ cargo test
```

## Contribution guide

Ensure all tests are passing [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html) and [
`rustfmt`](https://github.com/rust-lang/rustfmt) has been run. This can be with [
`cargo make`](https://github.com/sagiegurari/cargo-make); installable with:

```sh
$ cargo install --force cargo-make
```

Then run:

```sh
$ cargo make
```

Finally, we recommend [feature-branches](https://martinfowler.com/bliki/FeatureBranch.html) with an
accompanying [pull-request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests).
</small>

<hr/>

## License

Licensed under any of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
- CC0 license ([LICENSE-CC0](LICENSE-CC0) or <https://creativecommons.org/publicdomain/zero/1.0/legalcode>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.
