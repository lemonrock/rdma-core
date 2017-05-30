// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#![allow(non_snake_case)]


use ::std::env;
use ::std::process::Command;


fn main()
{
	let absoluteHomeFolderPath = env::var("CARGO_MANIFEST_DIR").unwrap();
	
	// We deliberately run as much as possible outside of cargo as it makes it far easier to debug a long, complex build which has little to do with Rust.
	// Of course, this script, being shell, won't run under Windows.
	tool(&absoluteHomeFolderPath, "build-under-cargo");
}

fn tool(absoluteHomeFolderPath: &str, programName: &'static str) -> String
{
	let fullPath = format!("{}/tools/{}", absoluteHomeFolderPath.to_owned(), programName.to_owned());
	panicIfProcessNotSuccessful(programName, absoluteHomeFolderPath, Command::new(fullPath))
}

fn panicIfProcessNotSuccessful(programName: &'static str, absoluteHomeFolderPath: &str, mut command: Command) -> String
{
	let output = command.output().unwrap_or_else(|error|
	{
		panic!("Failed to execute '{}' in '{}' error was '{}'", programName, absoluteHomeFolderPath, error);
	});
	
	let code = output.status.code().unwrap_or_else(||
	{
		panic!("Failed to retrieve exit status from command - was it killed by a signal?");
	});

	let standardOut = String::from_utf8_lossy(&output.stdout);
	if code == 0
	{
		return standardOut.into_owned();
	}
	
	let standardError = String::from_utf8_lossy(&output.stderr);
	panic!("Command '{}' failed with exit code '{}' (standard out was '{}'; standard error was '{}')", programName, code, standardOut.into_owned(), standardError.into_owned());
}
