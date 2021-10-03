# Everhour CLI

[![dependency status](https://deps.rs/repo/github/V1RE/everhour/status.svg)](https://deps.rs/repo/github/V1RE/everhour)
![CI status](https://github.com/V1RE/everhour/actions/workflows/ci.yml/badge.svg)
[![Coverage Status](https://coveralls.io/repos/github/V1RE/everhour/badge.svg)](https://coveralls.io/github/V1RE/everhour)

A CLI tool built in Rust to interact with the [Everhour API](https://everhour.docs.apiary.io/)

## Features

- [x] Display current timer status
- [x] Stop current timer
- [ ] Start timer by ID
- [ ] Find tasks (with [fuzzyfinder](https://github.com/lotabout/skim)?)
- [ ] Display reported time in [table](https://github.com/Nukesor/comfy-table)

## Installation

Clone the repository and build the binary.

```bash
  git clone https://github.com/V1RE/everhour.git
  cd everhour
  cargo build --release
```

This will create a binary in `target/release`, which you can copy to a location in your `PATH`.

## Authors

- [Niels Mentink](https://www.github.com/V1RE)
