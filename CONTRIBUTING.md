# Contributing to functional-programming-jargon.rs

All sort of contributions are welcome and there are no complicated rules with it.
We appreciate:

- New features
- Bug fixes
- Suggestions
- Ideas

## Issues

Feel free to submit issues, ideas, suggestions and enhancement requests.

## Contributing

Please refer to each project's style guidelines and guidelines for submitting patches and additions.
In general, we follow the "fork-and-pull" Git workflow.

1.  **Fork** the repo on GitHub
2.  **Clone** the project to your own machine
3.  **Commit** changes to your own branch
4.  **Push** your work back up to your fork
5.  Submit a **Pull request** so that we can review your changes

NOTE: Be sure to merge the latest from "upstream" before making a pull request!

## Development Environments

| type | version                |
| ---- | ---------------------- |
| OS   | Linux, Windows and Mac |

You will need to install Rust to work on this project. Installation instruction can be found at https://www.rust-lang.org/tools/install.

### Requirements

- **Rust stable** (1.70.0 or later recommended)
- **No nightly features required** - this project uses only stable Rust features

The project automatically uses the correct Rust version via `rust-toolchain.toml`.

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific package
cargo test -p fp-core

# Run with output
cargo test -- --nocapture
```

### Code Quality Checks

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Run clippy lints
cargo clippy --all-targets --all-features

# Fix clippy warnings automatically (where possible)
cargo clippy --all-targets --all-features --fix
```

## Copyright and Licensing

 is an open source project licensed under the MIT license.

functional-programming-jargon.rs does not require you to assign the copyright of your contributions, you retain the copyright.
functional-programming-jargon.rs does require that you make your contributions available under the MIT license in order to be
included in the main repo.

If appropriate, include the MIT license summary at the top of each file along with the copyright info.
If you are adding a new file that you wrote, include your name in the copyright notice in the license
summary at the top of the file.

## License Summary

You can copy and paste the MIT license summary from below.

```
MIT License

Copyright (c) 2026 Jason Shin

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
```
