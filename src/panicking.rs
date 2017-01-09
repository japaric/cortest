//! Implementation of the `panic!` macro

use core::fmt::Write as _0;
use core::fmt;

use io::Write as _1;
use {io, process};

#[lang = "panic_fmt"]
pub extern "C" fn rust_begin_panic(msg: fmt::Arguments,
                                   file: &'static str,
                                   line: u32)
                                   -> ! {
    let mut stderr = io::stderr();
    stderr.write_all(b"panicked at '")
        .ok()
        .and_then(|_| stderr.write_fmt(msg).ok())
        .and_then(|_| writeln!(stderr, "', {}:{}", file, line).ok());

    process::exit(101)
}
