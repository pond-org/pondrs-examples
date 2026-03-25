# 🤔 pondrs-examples

[![CI](https://github.com/pond-org/pondrs-examples/actions/workflows/ci.yml/badge.svg)](https://github.com/pond-org/pondrs-examples/actions/workflows/ci.yml)

Example projects using the [pondrs](https://github.com/pond-org/pondrs) pipeline library.

## Examples

- [pondrs-spaceflights](pondrs-spaceflights/) — A port of Kedro's [spaceflights-pandas](https://github.com/kedro-org/kedro-starters/tree/main/spaceflights-pandas) starter to pondrs.

## Usage

All examples follow the same workflow. From within an example directory:

```sh
cargo build
cargo run -- run
```

To use the interactive pipeline visualization, start the viz server first:

```sh
cargo run -- viz
```

Then, in a separate terminal, run the pipeline:

```sh
cargo run -- run
```

The visualization is available at [localhost:8080](http://localhost:8080) and will display execution information as the pipeline runs.

## Example Data

Some examples include copied example data from their upstream projects. License files are provided in the relevant directories.

## AI Disclosure

This library was designed and architected by humans. Implementation was carried out by an AI coding agent under close human supervision and review.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
