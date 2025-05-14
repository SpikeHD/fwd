<div align="center">
  <h1>fwd</h1>
  <p>Expose localhost ports to your local network</p>
</div>

<div align="center">
  <img src="https://img.shields.io/github/actions/workflow/status/SpikeHD/fwd/build.yml" />
  <img src="https://img.shields.io/github/repo-size/SpikeHD/fwd" />
  <img src="https://img.shields.io/github/commit-activity/m/SpikeHD/fwd" />
</div>

<div align="center">
  <img src="https://img.shields.io/github/release-date/SpikeHD/fwd" />
  <img src="https://img.shields.io/github/stars/SpikeHD/fwd" />
</div>

# Features

* Multithreaded
* Native ARM support

# Table of Contents
* [Installation](#installation)
* [Usage](#usage)
* [Building](#building)
  * [Prerequisites](#prerequisites)
  * [Steps](#steps)
* [TODO](#todo)
* [Contributions](#contributions)

## Installation

### Windows

```powershell
# Run the install script
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/SpikeHD/fwd/refs/heads/main/install.ps1" -OutFile "$env:TEMP\install.ps1"; PowerShell -ExecutionPolicy Bypass -File "$env:TEMP\install.ps1"

# You can uninstall by deleting C:\Program Files\fwd
del "C:\Program Files\fwd"
```

### Linux

```shell
# Run the install script
curl -fsSL https://raw.githubusercontent.com/SpikeHD/fwd/refs/heads/main/install.sh | sudo bash

# You can uninstall by removing the binary from /usr/local/bin
rm /usr/local/bin/fwd
```

### macOS

```shell
# Run the install script
curl -fsSL https://raw.githubusercontent.com/SpikeHD/fwd/refs/heads/main/install.sh | sudo bash

# You can uninstall by removing the binary from /usr/local/bin
rm /usr/local/bin/fwd
```

# Usage

> [!NOTE]
> You may need to add a firewall rule to allow incoming connections on the port you are forwarding to first!

```shell
# Get all options
fwd -h

# Forward connections to port 8080 from port 8081 (fwd will automatically expose <dst + 1> to the local network if nothing is specified)
fwd 8080

# Forward connections to port 8080 from port 88001 on the local network
fwd 8080 -p 88001
```

## Building

### Prerequisites

* Rust

### Steps

1. Clone the repository
2. Run `cargo build --release`
3. The binary will be in `target/release/fwd`

## TODO

- [ ] Add support for UDP
- [ ] Multi-mapping (map multiple ports at once)

## Contributions

Issues, PRs, etc. are all welcome!
