use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

#[rustversion::since(1.65.0)]
use std::panic::PanicHookInfo as StdPanicHookInfo;

use std::any::Any;
#[rustversion::before(1.81.0)]
use std::panic::PanicInfo as StdPanicHookInfo;

fn lock_global_panic_info() -> MutexGuard<'static, GlobalPanicContext> {
    /// A lazily initialized global panic log. It aggregates the panics from all threads.
    /// This is used to find the info about the panic after the `catch_unwind` call
    /// to observe the context of the panic that happened.
    static GLOBAL: Mutex<GlobalPanicContext> = Mutex::new(GlobalPanicContext {
        last_panic: None,
        initialized: false,
    });

    GLOBAL.lock().unwrap_or_else(PoisonError::into_inner)
}

struct GlobalPanicContext {
    last_panic: Option<PanicContext>,
    initialized: bool,
}

/// This struct without any fields exists to make sure that [`PanicListener::register()`]
/// is called first before the code even attempts to get the last panic information.
#[derive(Default)]
pub(super) struct PanicListener {
    /// Required to make sure struct is not constructable via a struct literal
    /// in the code outside of this module.
    _private: (),
}

impl PanicListener {
    pub(super) fn register() -> Self {
        let mut global = lock_global_panic_info();

        if global.initialized {
            return Self { _private: () };
        }

        let prev_panic_hook = std::panic::take_hook();

        std::panic::set_hook(Box::new(move |panic_info| {
            {
                // Make sure the lock is released before the other panic hook is called
                // to prevent potential reentrancy. Therefore this code is in a block.
                let mut global = lock_global_panic_info();
                let panic_number = global.last_panic.as_ref().map(|p| p.0.panic_number);
                let panic_number = panic_number.unwrap_or(0) + 1;

                global.last_panic = Some(PanicContext::from_std(panic_info, panic_number));
            }

            prev_panic_hook(panic_info);
        }));

        global.initialized = true;
        drop(global);

        Self { _private: () }
    }

    /// Returns the last panic that happened since the [`PanicListener::register()`] call.
    // `self` is required to make sure this code runs only after we initialized
    // the global panic listener in the `register` method.
    #[allow(clippy::unused_self)]
    pub(super) fn get_last_panic(&self) -> Option<PanicContext> {
        lock_global_panic_info().last_panic.clone()
    }
}

/// Contains all the necessary bits of information about the occurred panic.
#[derive(Clone)]
pub(super) struct PanicContext(Arc<PanicContextShared>);

struct PanicContextShared {
    backtrace: backtrace::Backtrace,

    location: Option<PanicLocation>,
    thread_name: Option<String>,

    /// Defines the number of panics that happened before this one. Each panic
    /// increments this counter. This is useful to know how many panics happened
    /// before the current one.
    panic_number: usize,
}

impl PanicContext {
    fn from_std(std_panic_info: &StdPanicHookInfo<'_>, panic_number: usize) -> Self {
        let location = std_panic_info.location();
        Self(Arc::new(PanicContextShared {
            backtrace: backtrace::Backtrace::capture(),
            location: location.map(PanicLocation::from_std),
            thread_name: std::thread::current().name().map(String::from),
            panic_number,
        }))
    }
}

impl fmt::Debug for PanicContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for PanicContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let PanicContextShared {
            location,
            backtrace,
            thread_name,
            panic_number,
        } = &*self.0;

        write!(f, "panic occurred")?;

        if let Some(location) = location {
            write!(f, " at {location}")?;
        }

        if let Some(thread_name) = thread_name {
            write!(f, " in thread '{thread_name}'")?;
        }

        if *panic_number > 1 {
            write!(f, " (total panics observed: {panic_number})")?;
        }

        // #[rustversion::attr(before(1.65.0), allow(clippy::irrefutable_let_patterns))]
        #[allow(clippy::incompatible_msrv)]
        if backtrace.status() == backtrace::BacktraceStatus::Captured {
            write!(f, "\nbacktrace:\n{backtrace}")?;
        }

        Ok(())
    }
}

/// Extract the message of a panic.
pub(super) fn message_from_panic_payload(payload: &dyn Any) -> Option<String> {
    if let Some(str_slice) = payload.downcast_ref::<&str>() {
        return Some((*str_slice).to_owned());
    }
    if let Some(owned_string) = payload.downcast_ref::<String>() {
        return Some(owned_string.clone());
    }

    None
}

/// Location of the panic call site.
#[derive(Clone)]
struct PanicLocation {
    file: String,
    line: u32,
    col: u32,
}

impl PanicLocation {
    fn from_std(loc: &std::panic::Location<'_>) -> Self {
        Self {
            file: loc.file().to_owned(),
            line: loc.line(),
            col: loc.column(),
        }
    }
}

impl fmt::Display for PanicLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.col)
    }
}

#[rustversion::since(1.65.0)]
#[allow(clippy::module_name_repetitions)]
mod backtrace {
    pub(super) use std::backtrace::{Backtrace, BacktraceStatus};
}

#[rustversion::before(1.65.0)]
mod backtrace {
    #[derive(PartialEq)]
    pub(super) enum BacktraceStatus {
        Captured,
    }

    pub(super) struct Backtrace;

    impl Backtrace {
        pub(super) fn capture() -> Self {
            Self
        }
        pub(super) fn status(&self) -> BacktraceStatus {
            BacktraceStatus::Captured
        }
    }

    impl std::fmt::Display for Backtrace {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("{update your Rust compiler to >=1.65.0 to see the backtrace}")
        }
    }
}
