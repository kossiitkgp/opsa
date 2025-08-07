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

[//]: # (- [Usage]&#40;#usage&#41;)

[//]: # (- [Contact]&#40;#contact&#41;)

[//]: # (    - [Maintainer&#40;s&#41;]&#40;#maintainers&#41;)

[//]: # (    - [creators&#40;s&#41;]&#40;#creators&#41;)

[//]: # (- [Additional documentation]&#40;#additional-documentation&#41;)

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

## Getting Started

### Prerequisites
The following dependencies are required to be installed for the project to function properly:
- [Rust](https://www.rust-lang.org/tools/install)
- [GoLang](https://go.dev/doc/install)
- [Docker](https://www.docker.com/get-started/) and [Docker Compose](https://docs.docker.com/compose/install/)
- [GNU Make](https://www.gnu.org/software/make/)
- [cargo-watch](https://github.com/watchexec/cargo-watch): `cargo install cargo-watch`
- [sqlx-cli](https://lib.rs/crates/sqlx-cli): `cargo install sqlx-cli`
- `Node JS` and `npm`

To set up a local instance of the application, follow the steps below.
1. Download a slack archive `.zip` file from your required workspace. [How-to](https://slack.com/intl/en-in/help/articles/201658943-Export-your-workspace-data)
2. Create `.env` file in project root from `.env.template`
3. Run `make digest FILE="/path/to/file.zip"`
4. Run `make dev`.
5. Make sure you have node js installed, then run `npm install` in the `garnisher` directory. 
6. Run `npm run dev` in the `garnisher` directory to start the frontend server. 
7. Enjoy.

<p align="right">(<a href="#top">back to top</a>)</p>
