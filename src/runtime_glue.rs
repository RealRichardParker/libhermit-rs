// Copyright (c) 2017 Stefan Lankes, RWTH Aachen University
//               2018 Colin Finck, RWTH Aachen University
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Minor functions that Rust really expects to be defined by the compiler,
//! but which we need to provide manually because we're on bare metal.

use alloc::alloc::Layout;
use arch;
use core::panic::PanicInfo;

// see https://users.rust-lang.org/t/psa-breaking-change-panic-fmt-language-item-removed-in-favor-of-panic-implementation/17875
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	print!("[{}][!!!PANIC!!!] ", arch::percore::core_id());

	if let Some(location) = info.location() {
		print!("{}:{}: ", location.file(), location.line());
	}

	if let Some(message) = info.message() {
		print!("{}", message);
	}

	println!("");

	loop {
		arch::processor::halt();
	}
}

#[cfg(not(test))]
#[lang = "oom"]
#[no_mangle]
pub fn rust_oom(layout: Layout) -> ! {
	println!(
		"[{}][!!!OOM!!!] Memory allocation of {} bytes failed",
		arch::percore::core_id(),
		layout.size()
	);
	loop {
		arch::processor::halt();
	}
}
