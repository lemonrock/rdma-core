// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


macro_rules! panic_on_errno
{
	($function: path$(,$argument: expr)*) =>
	{
		{
			let okOrErrorNumber = unsafe { $function($($argument),*) };
			debug_assert!(okOrErrorNumber >= 0, "{} returned a result '{}' which was negative", stringify!($function), okOrErrorNumber);
			if $crate::rust_extra::unlikely(okOrErrorNumber != 0)
			{
				panic!("{} failed with error number '{}'", stringify!($function), okOrErrorNumber);
			}
		}
	}
}
