# SHAAT Stack (SQL, HTML, Actix, Askama, Tailwind) 
A web service without _any_ client-side JavaScript, taking it back to the good ol' days of HTML templates, but with all the typesafety of Rust and DX Tailwind! 🦀

# Running
## Local
A `.env` should be created, `.env.template` can be used as a template: `cp .env.template .env`.

The rust crate requires a postgres database to be running and named in the `.env` file (`DATABASE_URL`).

`./scripts/backend_dev.sh` can be executed to start a local postgres docker image with parameters suitable for the default `DATABASE_URL` found in `.env.template` file.

This script also requires the sqlx cli to be installed to run the migrations on the database. sqlx cli can be installed with `cargo install sqlx-cli`

With the backend running `./scripts/frontend_dev.sh` can be used to build the tailwind and begin the actix server. The web service should now be viewable on `localhost:3000`.

Note that due to the rust webservice utilizing sqlx macros, it will not compile unless `DATABASE_URL` is defined in the env, and the database is running and migrated (i.e. as done by executing `./scripts/backend_dev.sh`).

## Containerized
Docker compose can also be used to deploy the full stack by running `docker compose up`. Note that the `.env` file is still required for this, where secrets for JWT and Hash should be placed.
