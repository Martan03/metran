# metran

(iOS) Media Transfer utility written in Rust.

## Table of Contents

- [Description](#description)
- [Installation](#installation)
- [Usage](#usage)
- [Links](#links)

## Description

I got really tired of transfering photos from iPhone to Arch + Hyprland. Even
though there are existing apps, such as KDE connect, they sometimes just don't
save the sent files for some reason. It also (as far as I know), doesn't
support deleting files after transfer.

## Installation

You have to compile it yourself, but that shouldn't be a problem. Only thing
you need is `cargo`:

```bash
cargo build -r
```

After its done compiling, you can start it in `./target/release/metran`

## Usage

Run metran to start the server, which listens for files.

```bash
./metran
```

Then on your iPhone, get Shortcuts app and add
[metran shortcut](https://www.icloud.com/shortcuts/9027245d8ecb4de78071dd6062f5dece).
When adding the shortcut, it asks you for the server address. You need to know
your PC IP address. You can find it by running:

```bash
ip a show
```

You should look for devices such as `wlan` or `eno` and then find the `inet`
field. The IP address should look something like `192.168.1.50`.

You can then run the shortcut. It will prompt you to select the photos to send
and after sending them, it will prompt if you want to delete the sent photos.

You can also adjust the configuration, such as the server address and port.

```bash
./metran -a 127.0.0.1 -p 1234
```

You can see all the usage in the program help.

```bash
./metran -h
```

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [metran](https://github.com/Martan03/metran)
- **Author website:** [martan03.github.io](https://martan03.github.io)
