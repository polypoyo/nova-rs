//! Nova's logging abstraction
//!
//! _Or maybe this will be Nova's implementation of a popular logging abstraction_
//!
//! Nova logs need to go _somewhere_. While Nova itself could maintain a log file, we decided instead that Nova should
//! use the client application's logger. When Nova is initialized by the client, the client passes in callbacks for
//! logging. All of Nova's logs go through these callbacks
//!
//! Thanks to the logging crate we can simply log from everywhere in Nova's source. We however provide a very basic
//! logger for tests and in case the application doesn't set one

/// Very basic logger struct, containing info if debug and trace level logs are enabled
///
/// # Examples
/// Example of using this logger:
/// ```edition2018
/// use log::{LevelFilter, error, warn, info, debug, trace};
/// use nova_rs::logging::BasicLogger;
///
/// // Enabled trace and debug
/// let logger = Box::new(BasicLogger::new(true, true));
/// // We also need to enable Trace here
/// log::set_boxed_logger(logger).map(|()| log::set_max_level(LevelFilter::Trace)).expect("Failed to set logger");
///
/// // Now we can use the macros provided by the log crate
/// error!("An error occurred: {}", "Something went wrong...");
/// warn!("Failed to load optional dependency {}, some functionality will not be available!", "Vectors");
/// info!("Program started successfully.");
/// debug!("x = {}", 5 * 6);
/// trace!("Reading char...");
/// ```
pub struct BasicLogger {
    /// If debug level logs are enabled
    debug: bool,

    /// If trace level logs are enabled
    trace: bool,
}

/// Implementation of the `BasicLogger` logging to the standard output streams
impl BasicLogger {
    /// Constructs a new `BasicLogger`
    ///
    /// Here the debug and trace levels can be enabled, all other levels are always enabled
    /// with the `BasicLogger`
    pub fn new(debug: bool, trace: bool) -> BasicLogger {
        BasicLogger { debug, trace }
    }
}

impl log::Log for BasicLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        match metadata.level() {
            log::Level::Debug => self.debug,
            log::Level::Trace => self.trace,
            _ => true,
        }
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            if record.metadata().level() <= log::Level::Warn {
                eprintln!("[{}]: {}", record.level(), record.args());
            } else {
                println!("[{}]: {}", record.level(), record.args());
            }
        }
    }

    fn flush(&self) {
        // Flushing is not required when using the std output streams
    }
}
