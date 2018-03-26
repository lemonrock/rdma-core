// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#![allow(non_snake_case)]


extern crate cc;


use ::cc::Build;
use ::std::env;
use ::std::process::Command;


fn main()
{
	let cargo_manifest_folder_path = variable("CARGO_MANIFEST_DIR");
	let cargo_build_folder_path = variable("OUT_DIR");
	
	// We deliberately run as much as possible outside of cargo as it makes it far easier to debug a long, complex build which has little to do with Rust.
	// Of course, this script, being shell, won't run under Windows.
	println!("{}", tool(&cargo_manifest_folder_path, "bindgen-wrapper/build-under-cargo"));
	compile_embedded_c_code(&cargo_manifest_folder_path, &cargo_build_folder_path);
}

fn compile_embedded_c_code(cargo_manifest_folder_path: &str, cargo_build_folder_path: &str)
{
	if env::var("CROSS_COMPILE").is_err()
	{
		panic!("Please specify CROSS_COMPILE=x86_64-linux-musl- cargo build --target=x86_64-unknown-linux-musl as the 'cc' crate incorrectly looks for musl-gcc")
	}
	
	let files_path = format!("{}/src/bindgen/c", cargo_manifest_folder_path.to_owned());
	let headers_path = format!("{}/root/DESTDIR/usr/include", cargo_build_folder_path.to_owned());
	
	// We use .flag(-isystem) rather than .include() to stop warnings that occur in system headers.
	Build::new()
	.file(format!("{}/infiniband-verbs-static-inline.c", files_path))
	.file(format!("{}/rdma-cma-static-inline.c", files_path))
	.file(format!("{}/rdma-verbs-static-inline.c", files_path))
	.flag("-Werror")
	.flag(&format!("-isystem{}", headers_path))
	.include(files_path)
	.define("_GNU_SOURCE", None)
	.define("_BSD_SOURCE", None)
	.compile("rdma_core_sys_c.a");
}

fn variable(environment_variable_name: &str) -> String
{
	env::var(environment_variable_name).unwrap()
}

fn tool(cargo_manifest_folder_path: &str, program_name: &'static str) -> String
{
	let full_path = format!("{}/tools/{}", cargo_manifest_folder_path.to_owned(), program_name.to_owned());
	panic_if_process_not_successful(program_name, cargo_manifest_folder_path, Command::new(full_path))
}

fn panic_if_process_not_successful(program_name: &'static str, cargo_manifest_folder_path: &str, mut command: Command) -> String
{
	let output = command.output().unwrap_or_else(|error|
	{
		panic!("Failed to execute '{}' in '{}' error was '{}'", program_name, cargo_manifest_folder_path, error);
	});
	
	let code = output.status.code().unwrap_or_else(||
	{
		panic!("Failed to retrieve exit status from command - was it killed by a signal?");
	});

	let standard_out = String::from_utf8_lossy(&output.stdout);
	if code == 0
	{
		return standard_out.into_owned();
	}
	
	let standard_error = String::from_utf8_lossy(&output.stderr);
	panic!("Command '{}' failed with exit code '{}' (standard out was '{}'; standard error was '{}')", program_name, code, standard_out.into_owned(), standard_error.into_owned());
}
