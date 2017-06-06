// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


macro_rules! panic_on_error_with_clean_up
{
	($status: ident, $failureBlock: block, $function: path$(,$argument: expr)*) =>
	{
		{
			let $status = unsafe { $function($($argument),*) };
			if $crate::rust_extra::unlikely($status != ucs_status_t::UCS_OK)
			{
				$failureBlock
				let description = unsafe { CStr::from_ptr(ucs_status_string($status)) };
				panic!("{} failed with status code '{:?}' ({:?})", stringify!($function), $status, description);
			}
		}
	}
}
