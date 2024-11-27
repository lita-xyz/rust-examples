pub use core::sync::atomic::{
    AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32,
    AtomicU64, AtomicU8, AtomicUsize, Ordering,
};

// Macro for testing basic store/load operations
#[macro_export]
macro_rules! test_store_load {
    ($func_name:ident, $atomic_type:ty, $value:expr) => {
        fn $func_name() -> bool {
            let atomic = <$atomic_type>::new(false as _);
            atomic.store($value, Ordering::SeqCst);
            atomic.load(Ordering::SeqCst) == $value
        }
    };
}

// Macro for testing fetch_add operations
#[macro_export]
macro_rules! test_fetch_add {
    ($func_name:ident, $atomic_type:ty, $initial:expr, $add:expr) => {
        fn $func_name() -> bool {
            let atomic = <$atomic_type>::new($initial);
            let previous = atomic.fetch_add($add, Ordering::SeqCst);
            previous == $initial && atomic.load(Ordering::SeqCst) == $initial + $add
        }
    };
}

// Macro for testing fetch_sub operations
#[macro_export]
macro_rules! test_fetch_sub {
    ($func_name:ident, $atomic_type:ty, $initial:expr, $sub:expr) => {
        fn $func_name() -> bool {
            let atomic = <$atomic_type>::new($initial);
            let previous = atomic.fetch_sub($sub, Ordering::SeqCst);
            previous == $initial && atomic.load(Ordering::SeqCst) == $initial - $sub
        }
    };
}

// Macro for testing fetch_or operations
#[macro_export]
macro_rules! test_fetch_or {
    ($func_name:ident, $atomic_type:ty, $initial:expr, $or:expr) => {
        fn $func_name() -> bool {
            let atomic = <$atomic_type>::new($initial);
            let previous = atomic.fetch_or($or, Ordering::SeqCst);
            previous == $initial && atomic.load(Ordering::SeqCst) == ($initial | $or)
        }
    };
}