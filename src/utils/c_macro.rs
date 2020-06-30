//自定义日志宏

#[macro_export]
macro_rules! c_debug {
    ($target:expr, $($arg:tt)*) => (
        debug!(concat!("[D]===> ", $target), $($arg)*)
    );
    ($($arg:tt)*) => (
        debug!("[D]===> {:?}", $($arg)*)
    )
}

#[macro_export]
macro_rules! c_info {
    ($target:expr, $($arg:tt)*) => (
        info!(concat!("[F]===> ", $target), $($arg)*)
    );
    ($($arg:tt)*) => (
        info!("[F]===> {:?}", $($arg)*)
    )
}

#[macro_export]
macro_rules! c_important {
    ($target:expr, $($arg:tt)*) => (
        warn!(concat!("[I]===> ", $target), $($arg)*)
    );
    ($($arg:tt)*) => (
        warn!("[I]===> {:?}", $($arg)*)
    )
}

#[macro_export]
macro_rules! c_v_important {
    ($target:expr, $($arg:tt)*) => (
        warn!(concat!("[V]===> ", $target), $($arg)*)
    );
    ($($arg:tt)*) => (
        warn!("[V]===> {:?}", $($arg)*)
    )
}

#[macro_export]
macro_rules! c_error {
    ($target:expr, $($arg:tt)*) => (
        error!(concat!("[E]===> ", $target), $($arg)*)
    );
    ($($arg:tt)*) => (
        error!("[E]===> {:?}", $($arg)*)
    )
}
