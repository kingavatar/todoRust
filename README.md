# todoRust

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/kingavatar/todoRust/Publish?logo=rust)
![GitHub Workflow Status](https://github.com/kingavatar/todoRust/workflows/CI/badge.svg)
![GitHub Workflow Status](https://github.com/kingavatar/todoRust/workflows/Publish/badge.svg)
[![Coverage Status](https://coveralls.io/repos/github/kingavatar/todoRust/badge.svg?branch=main)](https://coveralls.io/github/kingavatar/todoRust?branch=main)
![Libraries.io dependency status for todoRust](https://img.shields.io/librariesio/github/kingavatar/todoRust?logo=rust)

[![forthebadge made-with-rust](https://forthebadge.com/images/badges/made-with-rust.svg)](https://www.rust-lang.org/)

## About the Project

This is my first attempt to learn and implement Rust language. This application is a sort of extension to my other project [lmsScraperGo](https://github.com/kingavatar/lmsScraperGo). This application connects to the concurrent daemon socket server created by the scraper and consumes the events list. Then it extracts the date and time as `chrono` crate `datetime` variable and calculates the remaining time for each event(task).

## Getting Started

For now this has to be used with [lmsScraperGo](https://github.com/kingavatar/lmsScraperGo) for it to display output.

### Installation

Latest Releases for linux, macos and windows is available in the Releases Section [here](https://github.com/kingavatar/todoRust/releases).

### Building from source
1. Make sure you have installed the dependencies:
    - Better way to install `rustup` and run `rustup install stable`.
    - Make sure `cargo` is installed.

2. The source is also available in releases(recommended) or you can clone the repo:

```sh
git clone https://github.com/kingavatar/todoRust.git
cd todoRust
```

3. you can directly install package:

```sh
cargo install --path .
```
Make sure `cargo/bin` is in the path.

4. To build normally:

```sh
cargo build --release
```

### Usage

For color output in terminal use `term` argument:

```sh
todo term
```

For usage and color output in conky use `conky` argument:

```conky
execpi 600 todo conky
```