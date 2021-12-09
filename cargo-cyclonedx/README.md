[![Build Status](https://github.com/CycloneDX/cyclonedx-rust-cargo/workflows/Rust%20CI/badge.svg)](https://github.com/CycloneDX/cyclonedx-rust-cargo/actions?workflow=Rust+CI)
[![Crates.io](https://img.shields.io/crates/v/cargo-cyclonedx.svg)](https://crates.io/crates/cargo-cyclonedx)
[![License](https://img.shields.io/badge/license-Apache%202.0-brightgreen.svg)][License]
[![Website](https://img.shields.io/badge/https://-cyclonedx.org-blue.svg)](https://cyclonedx.org/)
[![Slack Invite](https://img.shields.io/badge/Slack-Join-blue?logo=slack&labelColor=393939)](https://cyclonedx.org/slack/invite)
[![Group Discussion](https://img.shields.io/badge/discussion-groups.io-blue.svg)](https://groups.io/g/CycloneDX)
[![Twitter](https://img.shields.io/twitter/url/http/shields.io.svg?style=social&label=Follow)](https://twitter.com/CycloneDX_Spec)


# `cargo-cyclonedx`

The [CycloneDX](https://cyclonedx.org/) plugin for `cargo` creates a [custom `cargo` subcommand](https://doc.rust-lang.org/cargo/reference/external-tools.html#custom-subcommands) that generates a Software Bill-of-Materials (SBOM) file that describes the `cargo` project.

CycloneDX is a lightweight SBOM specification that is easily created, human and machine readable, and simple to parse.

## Usage

### Installing

``` bash
cargo install cargo-cyclonedx
```

### Executing from `cargo`

``` bash
cargo cyclonedx
```

This produces a `bom.xml` file adjacent to every `Cargo.toml` file that exists in the workspace.

#### Common command-line options

* `--format` (`xml` or `json`): Defaults to XML output
* `--all`: Include the transitive dependencies for the project rather than only the top-level dependencies
* `--manifest-path`: where to find the `Cargo.toml` file if other than the default `cargo` location of the current directory

## Copyright & License

CycloneDX Rust Cargo is Copyright (c) OWASP Foundation. All Rights Reserved.

Permission to modify and redistribute is granted under the terms of the Apache 2.0 license. See the [LICENSE] file for the full license.

[License]: https://github.com/CycloneDX/cyclonedx-rust-cargo/blob/master/LICENSE