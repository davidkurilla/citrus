# Contributing to Citrus

First off, thank you for considering contributing to Citrus! Your help is greatly appreciated.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [How Can I Contribute?](#how-can-i-contribute)
    - [Reporting Bugs](#reporting-bugs)
    - [Suggesting Enhancements](#suggesting-enhancements)
    - [Pull Requests](#pull-requests)
3. [Style Guides](#style-guides)
    - [Git Commit Messages](#git-commit-messages)
    - [Code Style](#code-style)
4. [Getting Started](#getting-started)

## Code of Conduct

This project and everyone participating in it is governed by the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you are expected to uphold this code. Please report unacceptable behavior to [davidkurilla03@.com].

## How Can I Contribute?

### Reporting Bugs

If you find a bug, please open an issue [here](https://github.com/davidkurilla/citrus/issues) and include the following information:

- A clear and descriptive title.
- A description of the problem, including steps to reproduce the issue.
- Any relevant screenshots or logs.
- Your environment (OS, browser, Node.js version, etc.).

### Suggesting Enhancements

We welcome suggestions to improve Citrus! To suggest an enhancement, please:

1. Check if the enhancement has already been suggested by searching the issues.
2. If not, open a new issue with the following details:
    - A clear and descriptive title.
    - A detailed description of the suggested enhancement.
    - Any relevant examples or screenshots.

### Pull Requests

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/YourFeature`).
3. Make your changes.
4. Ensure all tests pass.
5. Commit your changes (`git commit -m 'Add some feature'`).
6. Push to the branch (`git push origin feature/YourFeature`).
7. Open a pull request describing your changes.

## Style Guides

### Git Commit Messages

- Use the present tense ("Add feature" not "Added feature").
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...").
- Limit the first line to 72 characters or less.
- Reference issues and pull requests liberally.

### Code Style

- Follow the coding style conventions used in the project.
- Ensure your code is properly linted.
- Write meaningful tests for your changes.

## Getting Started

To get a local copy up and running, follow these simple steps:

1. Clone the repo:

```shell
git clone https://github.com/davidkurilla/citrus
```

2. Use `cargo run` to run and test the application. Put command line arguments after the ``--`` of the line
```shell
cargo run -- args
```

Thank you for contributing to Citrus!
