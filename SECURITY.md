# Security Policy — CrawRAD

We take the security of CrawRAD seriously. This document describes how to report a
vulnerability privately, our scope, supported versions, and our Software Bill of
Materials (SBOM) practices.

## Supported Versions

During the beta phase, security updates are provided only for the latest Beta release.

| Version | Supported |
|---|---|
| Latest Beta release | :white_check_mark: Supported |
| Older releases | :x: Please update |

## Reporting a Vulnerability

**Please do NOT open a public issue, pull request, or discussion for a security
vulnerability.** Public disclosure before a fix is available puts users at risk.

Report privately through **GitHub Private Vulnerability Reporting**:

1. Go to the CrawRAD repository: [github.com/isaquepinheiro/CrawRAD](https://github.com/isaquepinheiro/CrawRAD).
2. Click on the **Security** tab.
3. Select **Report a vulnerability** to open an advisory draft.

> **Note:** We do not use an email address for security reports. GitHub Private Vulnerability Reporting is the single official channel.

### What to Include

- A description of the issue and its potential security impact.
- Steps to reproduce, or a minimal proof of concept.
- Affected version, operating system, and architecture.
- Any relevant logs — **with secrets, credentials, tokens, and personal data redacted**.

## What to Expect

When you submit a report through GitHub Private Vulnerability Reporting:

- **Acknowledgement:** We will acknowledge receipt of your report.
- **No fixed SLAs:** As an open-source project, we do not commit to fixed SLA promises or guaranteed resolution timelines, but we triage and prioritize reports by severity.
- **Coordinated disclosure:** We follow coordinated disclosure. We ask that you give us reasonable time to investigate and remediate the issue before any public disclosure, and we will keep you informed. With your permission, we will gladly credit you once the fix ships.

## Scope

### In Scope
- The CrawRAD IDE application and binary (`crawrad-ide`).
- Shipped runtime crates and helper binaries (`crawrad-runtime`, `crawrad-runtime-makepad`, `crawrad-runtime-egui`, `crawrad-dbprobe`, `crawrad-mcp`).
- Shipped installation packages and artifacts (Windows Inno Setup installer, bundle zip archive).

### Out of Scope
- Code, components, and data in your own projects built with CrawRAD.
- Third-party compilers and toolchains (`rustc`, `cargo`, system linkers).
- External tools and language servers (such as `rust-analyzer`).
- Operating system runtimes and system services.

## Software Bill of Materials (SBOM)

To enable supply-chain audits and artifact integrity verification:

- Each CrawRAD release attaches a machine-readable Software Bill of Materials in **CycloneDX 1.5** JSON format (`.cdx.json`) for every shipped binary crate (`crawrad-ide`, `crawrad-dbprobe`, and `crawrad-mcp`).
- Each release attaches a SHA-256 checksum file (`CrawRAD-<version>-SHA256SUMS.txt`) covering all files in the release bundle, allowing cryptographic verification of file integrity.
