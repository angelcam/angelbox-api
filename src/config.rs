// Copyright 2021 Angelcam, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

use clap::{App, Arg};

/// Port validator.
fn port_validator(port: String) -> Result<(), String> {
    u16::from_str(&port)
        .map(|_| ())
        .map_err(|_| String::from("invalid port"))
}

/// Bind address validator.
fn bind_address_validator(addr: String) -> Result<(), String> {
    IpAddr::from_str(&addr)
        .map(|_| ())
        .map_err(|_| String::from("invalid bind address"))
}

/// Log output.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum LogOutput {
    Silent,
    Stderr,
    Syslog,
}

/// Application configuration.
pub struct Config {
    data: Arc<ConfigData>,
}

impl Config {
    /// Create a new configuration.
    pub fn new() -> Self {
        Self {
            data: Arc::new(ConfigData::new()),
        }
    }

    /// Get bind address.
    pub fn bind_address(&self) -> SocketAddr {
        self.data.bind_address
    }

    /// Get the log output.
    pub fn log_output(&self) -> LogOutput {
        self.data.log_output
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal configuration data.
struct ConfigData {
    bind_address: SocketAddr,
    log_output: LogOutput,
}

impl ConfigData {
    /// Create new configuration data by parsing command line arguments.
    fn new() -> Self {
        let matches = App::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .takes_value(true)
                    .value_name("PORT")
                    .default_value("80")
                    .validator(port_validator)
                    .help("Listening port"),
            )
            .arg(
                Arg::with_name("bind-address")
                    .short("b")
                    .long("bind-address")
                    .takes_value(true)
                    .value_name("ADDRESS")
                    .default_value("0.0.0.0")
                    .validator(bind_address_validator)
                    .help("Bind address"),
            )
            .arg(
                Arg::with_name("quiet")
                    .short("q")
                    .long("quiet")
                    .takes_value(false)
                    .help("Do not produce any log messages"),
            )
            .arg(
                Arg::with_name("log-stderr")
                    .long("log-stderr")
                    .takes_value(false)
                    .help("Log to the standard error output instead of syslog"),
            )
            .get_matches();

        let bind_address = matches
            .value_of("bind-address")
            .unwrap()
            .parse::<IpAddr>()
            .unwrap();

        let listening_port = matches.value_of("port").unwrap().parse::<u16>().unwrap();

        let log_output = if matches.is_present("quiet") {
            LogOutput::Silent
        } else if matches.is_present("log-stderr") {
            LogOutput::Stderr
        } else {
            LogOutput::Syslog
        };

        Self {
            bind_address: SocketAddr::from((bind_address, listening_port)),
            log_output,
        }
    }
}
