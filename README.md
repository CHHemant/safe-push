# SafePush

<div align="center">

**The pre-commit guardian that blocks secrets and PII before you push.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Stars](https://img.shields.io/github/stars/CHHemant/safe-push?style=social)](https://github.com/CHHemant/safe-push)

</div>

> **Stop leaking API keys, passwords, and PII.** SafePush is a blazing-fast, zero-config tool written in Rust that scans your code at commit time and sanitizes diffs for public sharing.

## Why SafePush?

Most security tools scan *after* a leak happens. SafePush stops it **at the source**:
-  **Pre-Commit Blocking:** Prevents accidental pushes of secrets.
-  **Auto-Sanitization:** One command to generate a "safe diff" for GitHub Issues.
-  **Blazing Fast:** Built in Rust for instant scanning of large repositories.
-  **Zero Dependencies:** Single binary, no runtime installation required.

## Installation

### 1. Install via Cargo (Recommended)
```bash
cargo install safe-push
```

### 2. Manual Build
```bash
git clone https://github.com/CHHemant/safe-push.git
cd safe-push
cargo build --release
```

### 3. Enable the Pre-Commit Hook
Copy the hook to your local git hooks:
```bash
cp .githooks/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

## Usage

### Scan a Directory
Check your current project for leaks:
```bash
safe-push scan .
```

### Sanitize a File (For Public Sharing)
Need to paste code in a GitHub Issue but it has secrets?
```bash
safe-push sanitize config.json config-safe.json
```
Output: All secrets replaced with `[REDACTED]`.

### Block a Commit
Try to commit a file with an API key:
```bash
git add .
git commit -m "feat: update config"
# SafePush will block this and show the error!
```

## Detected Patterns

SafePush currently detects:

| Pattern | Example |
|---------|---------|
| AWS Access Keys & Secrets | `aws_access_key_id = ...` |
| GitHub Personal Access Tokens | `ghp_...` |
| Stripe Live Keys | `sk_live_...` |
| RSA/Private Keys | `-----BEGIN RSA PRIVATE KEY-----` |
| Email Addresses (PII) | `user@example.com` |

> **Note:** You can extend patterns by editing `src/main.rs`.

## Contributing

Contributions are welcome! Whether it's adding new regex patterns, improving the CLI, or fixing bugs, please open an issue or PR.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See [LICENSE](LICENSE) for more information.

## Author

**CHHemant** — [GitHub Profile](https://github.com/CHHemant)

Built with ❤️ and Rust to keep the internet secure.
