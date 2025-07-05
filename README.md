<a name="readme-top"></a>

<div align="center">

<a href="https://github.com/vistormu/cahier" target="_blank" title="go to the repo"><img width="196px" alt="cahier logo" src="/docs/logo.png"></a>


# cahier<br>a humble and cute ssh manager

_cahier_ is a notebook that will help you manage and handle your ssh connections

<br>

[![go version][go_version_img]][go_dev_url]
[![License][repo_license_img]][repo_license_url]
<!-- [![Go report][go_report_img]][go_report_url] -->

<br>

</div>

> [!WARNING]
> this project is functional but still in development, so expect some bugs and missing features

## ✨ features

- store all your ssh connections in a single hidden file so you never have to type them again
- use a simple and intuitive command line interface to manage your connections

## ⚡️ quick start

add a new ssh connection

```bash
cahier add nickname user@host:port
```

then, connect to it with

```bash
cahier connect nickname
```

## 📖 documentation

| command | description |
| --- | --- |
| `add` | add a new ssh connection |
| `bring` | bring (copy) a file or directory from the remote server to your local machine |
| `clear` | clear the cahier file, removing all ssh connections |
| `config` | configure the cahier file |
| `connect` | connect to an ssh connection |
| `delete` | delete an existing ssh connection |
| `exec` | execute a command on a remote server |
| `help` | show help for a specific command |
| `keygen` | generate a new ssh key pair |
| `list` | list all ssh connections |
| `ping` | ping a remote server to check if it's reachable |
| `remove` | remove an existing ssh connection |
| `send` | send (copy) a file or directory from your local machine to the remote server |
| `sendkey` | send an ssh public key to a remote server |
| `sftp` | connect to a remote server using sftp |
| `transfer` | transfer a file or directory from a remote server to another |
| `version` | show the current version of cahier |

## 🚀 installation

### homebrew

if you have [homebrew](https://brew.sh/) installed, you can tap and install the formula

```bash
brew install vistormu/cahier/cahier
```

### from releases

check the [releases](https://github.com/vistormu/cahier/releases) page and download the latest version for your operating system.

unzip the file (optional) and move the binary to a directory in your `PATH`, for example:

```bash
mv cahier /usr/local/bin/
```

### using go

if you have `go` installed, you can install _cahier_ with the following command:

```bash
go install github.com/vistormu/cahier@latest
```

this will install the binary in your `$GOPATH/bin` directory, so make sure to add it to your `PATH` if it's not already there.

### from source

clone the repo:

```bash
git clone https://github.com/vistormu/cahier.git
```

then build the project:

```bash
cd cahier
go build -o cahier
```

(optional) move the binary to a directory in your `PATH`:

```bash
mv cahier /usr/local/bin/
```

## 🌟 stargazers

[![Stargazers over time](https://starchart.cc/vistormu/cahier.svg?variant=adaptive)](https://starchart.cc/vistormu/cahier)

<div align="right">

[&nwarr; Back to top](#readme-top)

</div>


<!-- links -->
[go_version_img]: https://img.shields.io/badge/Go-1.24+-00ADD8?style=for-the-badge&logo=go
[go_dev_url]: https://go.dev/
[go_report_img]: https://goreportcard.com/badge/github.com/vistormu/cahier
[go_report_url]: https://goreportcard.com/report/github.com/vistormu/cahier
[repo_license_img]: https://img.shields.io/github/license/vistormu/cahier?style=for-the-badge
[repo_license_url]: https://github.com/vistormu/cahier/blob/main/LICENSE
