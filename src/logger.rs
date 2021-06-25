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

use std::{ffi::CString, io, ptr, sync::Once};

use flexi_logger::{writers::LogWriter, DeferredNow, Level, LevelFilter, Logger, Record};

use crate::config::{Config, LogOutput};

static SYSLOG_INIT: Once = Once::new();

/// Log writer using the POSIX log interface in libc.
struct SyslogWriter {
    fmt: CString,
}

impl SyslogWriter {
    /// Create a new syslog writer.
    fn new() -> Self {
        SYSLOG_INIT.call_once(|| unsafe {
            libc::openlog(ptr::null(), libc::LOG_CONS | libc::LOG_PID, libc::LOG_USER);
        });

        let fmt = CString::new("%s").unwrap();

        Self { fmt }
    }
}

impl LogWriter for SyslogWriter {
    fn write(&self, _: &mut DeferredNow, record: &Record) -> io::Result<()> {
        let msg = CString::new(format!("{}", &record.args())).unwrap();

        let level = match record.level() {
            Level::Debug => libc::LOG_DEBUG,
            Level::Info => libc::LOG_INFO,
            Level::Warn => libc::LOG_WARNING,
            Level::Error => libc::LOG_ERR,
            _ => return Ok(()),
        };

        unsafe {
            libc::syslog(level, self.fmt.as_ptr(), msg.as_ptr());
        }

        Ok(())
    }

    fn flush(&self) -> io::Result<()> {
        Ok(())
    }

    fn max_log_level(&self) -> LevelFilter {
        LevelFilter::Debug
    }
}

/// Format a given log record.
fn stderr_log_format(
    w: &mut dyn io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "{} {} {}",
        now.now().format("%Y-%m-%d %H:%M:%S%.3f"),
        record.level(),
        &record.args()
    )
}

/// Initialize the logger.
pub fn init(config: &Config) {
    match config.log_output() {
        LogOutput::Stderr => {
            Logger::try_with_str("info")
                .unwrap()
                .log_to_stderr()
                .format(stderr_log_format)
                .start()
                .unwrap();
        }
        LogOutput::Syslog => {
            Logger::try_with_str("info")
                .unwrap()
                .log_to_writer(Box::new(SyslogWriter::new()))
                .start()
                .unwrap();
        }
        _ => (),
    }
}
