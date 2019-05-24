#[cfg(feature = "tokio")]
#[doc(hidden)]
pub use tokio::spawn as __spawn;
#[cfg(all(feature = "tokio-executor", not(feature = "tokio")))]
#[doc(hidden)]
pub use tokio_executor::spawn as __spawn;

#[cfg(any(feature = "tokio", feature = "tokio-executor"))]
#[doc(hidden)]
pub use tokio_trace::{span as __span, Level as __Level};

#[cfg(any(feature = "tokio", feature = "tokio-executor"))]
#[macro_export(inner_local_macros)]
macro_rules! spawn {
    (level: $lvl:expr, target: $tgt:expr, name: $name:expr, $fut:expr, $($field:tt)*) => {{
        use $crate::macros::__spawn;
        use $crate::Instrument;
        let fut = Box::new($fut.instrument($crate::macros::__span!($lvl, target: $tgt, $name, $($field)*)));
        __spawn(fut)
    }};
    (level: $lvl:expr, name: $name:expr, $fut:expr, $($field:tt)*) => {
        spawn!(
            level: $lvl,
            target: __tokio_trace_futures_module_path!(),
            name: $name,
            $fut,
            $($field)*
        )
    };
    (level: $lvl:expr, target: $tgt:expr, $fut:expr, $($field:tt)*) => {
        spawn!(
            level: $lvl,
            target: $tgt,
            name: __tokio_trace_futures_stringify!($fut),
            $fut,
            $($field)*
        )
    };
    (target: $tgt:expr, name: $name:expr, $fut:expr, $($field:tt)*) => {
        spawn!(
            level: $crate::macros::__Level::TRACE,
            target: $tgt,
            name: $name,
            $fut,
            $($field)*
        )
    };
    (target: $tgt:expr, $fut:expr, $($field:tt)*) => {
        spawn!(
            level: $crate::macros::__Level::TRACE,
            target: $tgt,
            $fut,
            $($field)*
        )
    };
    (name: $name:expr, $fut:expr, $($field:tt)*) => {
        spawn!(
            target: __tokio_trace_futures_module_path!(),
            name: $name,
            $fut,
            $($field)*
        )
    };
    ($fut:expr, $($field:tt)*) => {
        spawn!(name: __tokio_trace_futures_stringify!($fut), $fut, $($field)*)
    };
    (level: $lvl:expr, target: $tgt:expr, name: $name:expr, $fut:expr) => {
        spawn!(level: $lvl, target: $tgt, name: $name, $fut,)
    };
    (level: $lvl:expr, name: $name:expr, $fut:expr) => {
        spawn!(level: $lvl, name: $name, $fut,)
    };
    (level: $lvl:expr, target: $tgt:expr, $fut:expr) => {
        spawn!(level: $lvl, target: $tgt, $fut,)
    };
    (target: $tgt:expr, name: $name:expr, $fut:expr) => {
        spawn!(target: $tgt, name: $name, $fut,)
    };
    (target: $tgt:expr, $fut:expr) => {
        spawn!(target: $tgt, $fut,)
    };
    (name: $name:expr, $fut:expr) => {
        spawn!(name: $name, $fut,)
    };
    ($fut:expr) => {
        spawn!($fut,)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __tokio_trace_futures_module_path {
    () => {
        module_path!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __tokio_trace_futures_stringify {
    ($ex:expr) => {
        stringify!($ex)
    };
}
