# Contributing to Shielded Contracts

Thank you for your interest in contributing to Shielded Protocol!

## Development Setup

1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Add WASM target: `rustup target add wasm32-unknown-unknown`
3. Install Stellar CLI: `cargo install --locked stellar-cli --features opt`
4. Clone the repo and run tests: `cargo test --all`

## Code Style

- Run `cargo fmt` before committing
- Ensure `cargo clippy --all-targets -- -D warnings` passes
- Write doc comments for all public items
- Add tests for all new functionality

## Pull Request Process

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/your-feature`
3. Make your changes with tests
4. Ensure CI passes locally
5. Submit a PR using the template

## Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` — new feature
- `fix:` — bug fix
- `docs:` — documentation changes
- `test:` — adding or updating tests
- `refactor:` — code refactoring
- `chore:` — maintenance tasks

## Security

If you discover a security vulnerability, please follow our [Security Policy](./SECURITY.md).
Do NOT open a public issue for security vulnerabilities.
