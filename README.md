# metran

(iOS) Media Transfer utility written in Rust.

## Table of Contents

- [Description](#description)
- [Installation](#installation)
- [Usage](#usage)
    - [Firewall Warning](#firewall-warning)
- [Links](#links)

## Description

I got really tired of transfering photos from iPhone to Arch + Hyprland. Even
though there are existing apps, such as KDE connect, they sometimes just don't
save the sent files for some reason. It also (as far as I know), doesn't
support deleting files after transfer.

## Installation

You have to compile it yourself, but that shouldn't be a problem. Only thing
you need is `cargo`.

You can install it globally to `~/.cargo/bin`:

```bash
cargo install --path .
```

Alternatively, you can just build the release binary locally (binary will be
in `./target/release`):

```bash
cargo build -r
```

## Usage

Run metran to start the server, which listens for files.

```bash
metran
```

Then on your iPhone, get Shortcuts app and add
[metran shortcut](https://www.icloud.com/shortcuts/6f27d108ab1242e88ff5385be8ef1fb3).
You can also use the
[zip variant](https://www.icloud.com/shortcuts/210d9ab0de3c4e2a9e7918f9d4244810)
of the shortcut, which sends ZIP archive instead of individual files. This aims
to improve transfer speeds, especially for bulk sending.

When adding the shortcut, it asks you for the server address. You need to know
your PC's local IP address. You can find it by running:

```bash
ip a show
```

Look for interfaces such as `wlan0` or `eno1` and then find the `inet` field.
The IP address should look something like `192.168.1.50`.

You can then run the shortcut. It will prompt you to select the photos to send,
and after sending them, it will prompt if you want to delete the original
photos from your phone.

---

When starting the server, you can also adjust the configuration, such as the
server address and port.

> Note: To allow connections from other devices on your network, you should
> use `0.0.0.0` address.

```bash
./metran -a 0.0.0.0 -p 1234
```

You can see all the usage in the program help.

```bash
./metran -h
```

### Firewall Warning

Ensure your firewall allows incoming TCP connections on the port metran is
running on (default i `8080`).

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [metran](https://github.com/Martan03/metran)
- **Author website:** [martan03.github.io](https://martan03.github.io)
- **Apple shortcut:**
    - [metran shortcut](https://www.icloud.com/shortcuts/9027245d8ecb4de78071dd6062f5dece)
    - [metran ZIP shortcut](https://www.icloud.com/shortcuts/210d9ab0de3c4e2a9e7918f9d4244810)
