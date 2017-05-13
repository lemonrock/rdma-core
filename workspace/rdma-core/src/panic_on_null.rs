// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


macro_rules! panic_on_null
{
	($function: path$(,$argument: expr)*) =>
	{
		{
			let pointer = unsafe { $function($($argument),*) };
			if $crate::rust_extra::unlikely(pointer.is_null())
			{
				let errno = $crate::errno::errno();
				panic!("{} failed with error number '{}' ('{}')", stringify!($function), errno.0, errno);
			}
			pointer
		}
	}
}
