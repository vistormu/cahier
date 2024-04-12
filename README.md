# Cahier: a humble and cute SSH manager

<p align="center">
    <img style="width: 20%" src="assets/logo.png">
</p>

Meet Cahier! Cahier is a single-page notebook that will help you manage and handle your SSH connections. Despite having a single page, Cahier will efficiently manage all your SSH connections.

## Usage

To setup Cahier for the first time, run the following command:

```bash
cahier setup
```

Then, you can use Cahier to manage your SSH connections with the following commands:

- `add`      Add a new host to the Cahier configuration file.
- `bring`    Bring a file or directory from a host to the local machine.
- `clear`    Clear the Cahier configuration file.
- `config`   Open the Cahier configuration file in the default editor.
- `connect`  Connect to a host in the Cahier configuration file.
- `delete`   Delete a host from the Cahier configuration file.
- `help`     Display the help message.
- `list`     List all hosts in the Cahier configuration file.
- `ping`     Ping a host in the Cahier configuration file.
- `send`     Send a file or directory from the local machine to a host.
- `setup`    Setup Cahier for the first time.
- `version`  Display the current version of Cahier.

## Installation

> Cahier is only available for macOS (for now).

Download the Cahier binary for your machine from the [Releases Page](https://github.com/vistormu/cahier/releases).

Then, rename it to `cahier`:

```bash
mv cahier-<ARCH> cahier
```

Give it execution privileges:

```bash
chmod +x cahier
```

And move it to the system's binaries:

```bash
sudo mv cahier /usr/local/bin/
```
