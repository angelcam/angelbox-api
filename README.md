# AngelBox API

REST API with additional features for AngelBox. Currently, the only supported
additional feature is direct audio playback.

* [API documentation](docs/api.md)

## Building the application

The application is written in Rust, so make sure that you have Rust installed
on your system. See https://www.rust-lang.org/tools/install for more info. To
build the application, simply run the following command:
```bash
cargo build
```

For production builds use:
```bash
cargo build --release
```

## Requirements

The application itself does not have any special requirements. However, in
order to use the audio playback feature, `aplay` (part of ALSA) must be present
on the host system.

## Application usage

You can start the application easily without any configuration or configuration
or command line arguments:
```bash
target/release/angelbox-api
```

By default the application will listen on all available network interfaces on
port 80. All logs will go to syslog. You can change this using command line
arguments:
```text
angelbox-api 0.1.0

USAGE:
    angelbox-api [FLAGS] [OPTIONS]

FLAGS:
    -h, --help          Prints help information
        --log-stderr    Log to the standard error output instead of syslog
    -q, --quiet         Do not produce any log messages
    -V, --version       Prints version information

OPTIONS:
    -b, --bind-address <ADDRESS>    Bind address [default: 0.0.0.0]
    -p, --port <PORT>               Listening port [default: 80]
```
