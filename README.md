# Hacker News Stories

A command line interface to browse and open Hacker News stories from the terminal

[![Crates.io](https://img.shields.io/crates/v/hn-stories?style=flat)](https://crates.io/crates/hn-stories)
[![Crates.io](https://img.shields.io/crates/d/hn-stories?style=flat)](https://crates.io/crates/hn-stories)
![CI](https://img.shields.io/github/actions/workflow/status/brysonbw/hn-stories/ci.yml?branch=main&style=flat&logo=github&label=CI)

## Install

```bash
cargo install hn-stories
```

## Usage

```bash
hn-stories <OPTIONS>
```

```text
Options:
  -s, --story <STORY>  Story type (top, new, best, ask, show, job) [default: t] [possible values: n, t, b, a, s, j]
  -l, --limit <LIMIT>  The number of stories to fetch and display in the terminal UI [default: 30]
  -h, --help           Print help
  -V, --version        Print version
```

## Contributing

If you have suggestions for how this project could be improved, or want to report a bug, feel free to open an issue! We welcome all contributions.

Likewise, before contributing please read and complete the [contribution guide](CONTRIBUTING.md).

## Resources

- [Changelog](CHANGELOG.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Contributing](CONTRIBUTING.md)
- [Security](SECURITY.md)

## License

[MIT](LICENSE)
