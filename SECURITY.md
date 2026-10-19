# Security policy

## Supported versions

Security reports are accepted for the current `0.1.x` development series. Older versions are not supported. This is early-stage software; no fixed response or patch timeline is promised.

## Private reporting

Please report vulnerabilities privately using the repository's [GitHub Security Advisories](https://github.com/taho140303/merlink-harness/security/advisories/new). Include reproduction steps, the affected version, and impact without sharing live credentials. If private reporting is unavailable, contact the [repository owner via GitHub](https://github.com/taho140303) to arrange a private channel. Do **not** open a public issue for an API key leak or publish an exploit before maintainers can investigate. If a key may be exposed, revoke or rotate it with the provider immediately.

## Scope

Merlink Harness is a local desktop app with a browser mock preview. Relevant issues include API-key exposure, path traversal affecting local files, and crossing the Tauri IPC boundary. No cloud service or telemetry backend is operated by this project.
