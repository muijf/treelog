# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Which versions are eligible for receiving such patches depends on the CVSS v3.0 Rating:

| Version | Supported          |
| ------- | ------------------ |
| 0.0.x   | Yes                |

## Reporting a Vulnerability

Please report (suspected) security vulnerabilities to the maintainer privately.

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them via one of the following methods:
- Email the maintainer directly
- Open a private security advisory on GitHub (if you have access)

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the requested information listed below (as much as you can provide) to help us better understand the nature and scope of the possible issue:

- Type of issue (e.g. buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit the issue

This information will help us triage your report more quickly.

## Security Considerations

TreeLog is a library for rendering tree structures and does not:
- Process untrusted input from network sources
- Execute system commands
- Access the file system beyond reading configuration
- Make network requests

However, if you discover a security vulnerability, please report it responsibly.
