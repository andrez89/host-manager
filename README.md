# Host Manager

A small GTK4 desktop app for blocking websites through your system hosts file.
It manages each domain and its `www` address, saves your list locally, and applies
changes only when you choose **Apply changes**. The interface uses Italian for an
Italian system locale and English otherwise.

## Features

- Add domains and block or unblock them individually or in bulk.
- Lock entries to protect them from accidental changes.
- Import existing `127.0.0.1` entries for managed domains.
- Request administrator authorization only when applying changes.

## Build

Install Rust, GTK4 development files, and `pkg-config`, then run:

```sh
cargo build --release
```

Platform notes:

- **Linux:** Debian/Ubuntu packages: `libgtk-4-dev pkg-config`. Applying changes uses Polkit (`pkexec`).
- **macOS:** Install GTK4 and `pkg-config` with `brew install gtk4 pkg-config`. Applying changes requests administrator authorization.
- **Windows:** Install the GTK4 runtime and make its DLLs available on `PATH`. Applying changes uses a UAC prompt.

Release archives include the executable, this README, and the MIT license. They
do not bundle GTK, sign binaries, or include installers. Windows and macOS need
the GTK runtime installed separately.

## Releases

Update `version` in `Cargo.toml`, commit and push the change, then push a matching
version tag, for example:

```sh
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions checks the tag against the Cargo version, builds and tests on
Linux, Windows, and macOS, then publishes archives to GitHub Releases. macOS
archives are available for Intel and Apple Silicon. Branch pushes and pull
requests run formatting, check, and test jobs.

## License

MIT. See [LICENSE](LICENSE). Copyright © 2026 Andrez <andre@amovez.it>.
