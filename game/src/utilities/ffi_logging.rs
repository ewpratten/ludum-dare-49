use std::{convert::TryInto, ffi::c_void};

use num_traits::FromPrimitive;
use printf::printf;

/// Direct mapping of Raylib's log levels
/// See: https://github.com/raysan5/raylib/blob/d875891a3c2621ab40733ca3569eca9e054a6506/parser/raylib_api.json#L985-L1026
#[derive(FromPrimitive)]
enum RaylibLogLevel {
    All = 0,
    Trace = 1,
    Debug = 2,
    Info = 3,
    Warning = 4,
    Error = 5,
    Fatal = 6,
    None = 7,
}

/// Callback that is called by the FFI code
#[allow(unsafe_code)]
#[no_mangle]
unsafe extern "C" fn raylib_log_callback(
    level: i32,
    message: *const i8,
    args: *mut raylib::ffi::__va_list_tag,
) {
    // Get the message as a string
    let formatted_message = printf(message, args as *mut c_void);

    // Handle the log level
    match RaylibLogLevel::from_u32(level.try_into().unwrap()) {
        Some(level) => match level {
            RaylibLogLevel::Trace => tracing::trace!("{}", formatted_message),
            RaylibLogLevel::Debug => tracing::debug!("{}", formatted_message),
            RaylibLogLevel::Warning => tracing::warn!("{}", formatted_message),
            RaylibLogLevel::Error => tracing::error!("{}", formatted_message),
            RaylibLogLevel::Fatal => tracing::error!("{}", formatted_message),
            _ => tracing::info!("{}", formatted_message),
        },
        None => {
            println!("{:?}", formatted_message)
        }
    }
}

/// Call this to replace raylib's logger with the rust logging system
pub fn hook_raylib_logging() {
    #[allow(unsafe_code)]
    unsafe {
        raylib::ffi::SetTraceLogCallback(Some(raylib_log_callback));
    }
}
