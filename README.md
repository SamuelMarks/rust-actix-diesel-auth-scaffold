rust-actix-diesel-auth-scaffold
===============================
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT%20OR%20CC0--1.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Simple baseline scaffold to get you started using actix + diesel with a custom OAuth2 implementation.

For example runnable server, see child repository: https://github.com/SamuelMarks/serve-actix-diesel-auth-scaffold

## Usage

Install Rust, `git`, and ensure you have your PostgreSQL and Redis/Valkey services setup.

### Environment setup

Add an `.env` file or otherwise add these environment variables; replacing connection strings with what you use:

    DATABASE_URL=postgres://rest_user:rest_pass@localhost/rest_db
    REDIS_URL=redis://127.0.0.1/

### Main entrypoint

In your own project, add dependencies to your Cargo.toml with `cargo add --git https://github.com/SamuelMarks/rust-actix-diesel-auth-scaffold` or by manually editing your `Cargo.toml` like so:
```toml
[dependencies]
rust-actix-diesel-auth-scaffold = { git = "https://github.com/SamuelMarks/rust-actix-diesel-auth-scaffold", version = "0.0.1" }
```

Then in your `main.rs` add:
```rs
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(rust_actix_diesel_auth_scaffold::routes::token::token)
            .service(rust_actix_diesel_auth_scaffold::routes::authorisation::authorise)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
```

## Contribution guide
Ensure all tests are passing [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html) and [`rustfmt`](https://github.com/rust-lang/rustfmt) has been run. This can be with [`cargo make`](https://github.com/sagiegurari/cargo-make); installable with:

```sh
$ cargo install --force cargo-make
```

Then run:
```sh
$ cargo make
```

Finally, we recommend [feature-branches](https://martinfowler.com/bliki/FeatureBranch.html) with an accompanying [pull-request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests).
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
