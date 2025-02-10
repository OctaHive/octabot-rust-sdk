extern crate alloc;
use core::fmt::Write;
use log::{Level, Log, Metadata, Record, SetLoggerError};

use crate::bindings::wit::wasi::logging::logging::{log, Level as LoggingLevel};

pub struct Logger {}

impl Logger {
  /// Sets the global logger to a `Logger` using [`log::set_logger`] and
  /// returns the installed `Logger`.
  ///
  /// This function may only be called once in the lifetime of a program. Any
  /// log events that occur before the call to [`log::set_logger`]
  /// completes will be ignored.
  ///
  /// # Errors
  ///
  /// An error is returned if a logger has already been set.
  pub fn install() -> Result<&'static Self, SetLoggerError> {
    static LOGGER: Logger = Logger {};

    log::set_logger(&LOGGER)?;

    Ok(&LOGGER)
  }
}

impl Log for Logger {
  #[inline]
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= log::max_level()
  }

  #[inline]
  fn log(&self, record: &Record) {
    if !self.enabled(record.metadata()) {
      return;
    }

    let level = match record.level() {
      Level::Error => LoggingLevel::Error,
      Level::Warn => LoggingLevel::Warn,
      Level::Info => LoggingLevel::Info,
      Level::Debug => LoggingLevel::Debug,
      Level::Trace => LoggingLevel::Trace,
    };

    if let Some(message) = record.args().as_str() {
      if record.module_path().is_none() && record.file().is_none() && record.line().is_none() && {
        #[cfg(feature = "kv")]
        {
          record.key_values().count() == 0
        }
        #[cfg(not(feature = "kv"))]
        {
          true
        }
      } {
        return log(level, record.target(), message);
      }
    }

    let mut message = alloc::string::String::new();

    if let Some(module_path) = record.module_path() {
      if module_path != record.target() {
        message.push_str(module_path);
      }
    }

    if let Some(file) = record.file() {
      if !message.is_empty() {
        message.push_str(" in ");
      }

      message.push_str(file);
    }

    if let Some(line) = record.line() {
      if !message.is_empty() {
        message.push(':');
      }

      #[allow(clippy::unwrap_used)]
      // formatting a u32 cannot fail
      message.write_fmt(format_args!("{line}")).unwrap();
    }

    if !message.is_empty() {
      message.push_str(": ");
    }

    #[allow(clippy::expect_used)]
    // failing to format the args is a bug and we cannot continue
    message
      .write_fmt(*record.args())
      .expect("formatting log::Record::args() returned an error");

    #[cfg(feature = "kv")]
    {
      if record.key_values().count() > 0 {
        if !message.is_empty() {
          message.push(' ');
        }

        #[allow(clippy::expect_used)]
        // failing to format the key-value pairs is a bug and we cannot continue
        message
          .write_fmt(format_args!("{:?}", KeyValues(record.key_values())))
          .expect("debug-formatting log::Record::key_values() returned an error");
      }
    }

    log(level, record.target(), &message);
  }

  #[inline]
  fn flush(&self) {}
}

#[cfg(feature = "kv")]
struct KeyValues<'a>(&'a dyn log::kv::Source);

#[cfg(feature = "kv")]
impl<'a> core::fmt::Debug for KeyValues<'a> {
  fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
    struct Display<T: core::fmt::Display>(T);

    impl<T: core::fmt::Display> core::fmt::Debug for Display<T> {
      fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.0.fmt(fmt)
      }
    }

    struct KeyValueMapFormatter<'a, 'b>(core::fmt::DebugMap<'a, 'b>);

    impl<'a, 'b, 'kvs> log::kv::VisitSource<'kvs> for KeyValueMapFormatter<'a, 'b> {
      fn visit_pair(&mut self, key: log::kv::Key<'kvs>, value: log::kv::Value<'kvs>) -> Result<(), log::kv::Error> {
        self.0.entry(&Display(key), &value);
        Ok(())
      }
    }

    // adapted from log v0.4.21
    // released under the MIT or Apache 2.0 License
    // https://docs.rs/log/0.4.21/src/log/lib.rs.html#730-745
    let mut visitor = KeyValueMapFormatter(fmt.debug_map());
    self.0.visit(&mut visitor).map_err(|_| core::fmt::Error)?;
    visitor.0.finish()
  }
}
