</div>

<!-- PROJECT LOGO -->
<br />
<!-- UPDATE -->
<div align="center">
  <a href="https://github.com/kossiitkgp/opsa">
     <img width="140" alt="image" src="https://raw.githubusercontent.com/metakgp/design/main/logos/black-large.jpg">
  </a>

<h3 align="center">Our Precious Slack Archives</h3>

  <p align="center">
  <!-- UPDATE -->
    <i>Monorepo of top secret stuff developed in an n-sided polygonal building to checkmate capitalism

</i>
    <br />
    <a href="https://UPDATE.org">Website</a>
    ·
    <a href="https://github.com/kossiitkgp/opsa/issues">Request Feature / Report Bug</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
<summary>Table of Contents</summary>

- [About The Project](#about-the-project)
- [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
- [Usage](#usage)
- [Contact](#contact)
    - [Maintainer(s)](#maintainers)
    - [creators(s)](#creators)
- [Additional documentation](#additional-documentation)

</details>

## About The Project
<!-- UPDATE -->
<div align="center">
  <a href="https://github.com/metakgp/PROJECT_NAME">
    <img width="80%" alt="image" src="https://user-images.githubusercontent.com/86282911/206632547-a3b34b47-e7ae-4186-a1e6-ecda7ddb38e6.png">
  </a>
</div>
Our Precious Slack Archives (OPSA) is a utility to show Slack archives. 
Using an archive `.zip` file exported from your slack workspace, discover the ancient
gospels preached by your ancestors.
<p align="right">(<a href="#top">back to top</a>)</p>

# Our Precious Slack Archives
Monorepo of top secret stuff developed in an n-sided polygonal building.
## Getting Started
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
#### Digester
#### Excretor
