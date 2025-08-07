# top-secret-stuff

Monorepo of top secret stuff developed in an n-sided polygonal building.

### Getting Started

#### Digester

#### Excretor

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Install [Docker](https://www.docker.com/get-started/) and [Docker Compose](https://docs.docker.com/compose/install/).
3. Make sure to add your user to the `docker` group: `sudo usermod -aG docker $USER`
3. Make sure [GNU Make](https://www.gnu.org/software/make/) is installed.
4. Install [`cargo-watch`](https://github.com/watchexec/cargo-watch) to compile on change: `cargo install cargo-watch`.
5. Install [`sqlx-cli`](https://lib.rs/crates/sqlx-cli) for compile-time checking of SQL queries: `cargo install sqlx-cli`.
6. Download a slack archive `.zip` file from your required workspace. [How-to](https://slack.com/intl/en-in/help/articles/201658943-Export-your-workspace-data)
7. Create `.env` file in project root from `.env.template`
8. Run `make digest FILE="/path/to/file.zip"`
9. Run `make dev`.
10. Make sure you have node js installed, then run `npm install` in the `garnisher` directory.
11. Run `npm run dev` in the `garnisher` directory to start the frontend server.
12. Enjoy
