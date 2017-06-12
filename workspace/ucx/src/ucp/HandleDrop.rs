// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


trait HandleDrop
{
	#[inline(always)]
	unsafe fn handleDrop(self);
}

impl HandleDrop for ucp_context_h
{
	#[inline(always)]
	unsafe fn handleDrop(self)
	{
		ucp_cleanup(self)
	}
}

impl HandleDrop for ucp_worker_h
{
	#[inline(always)]
	unsafe fn handleDrop(self)
	{
		ucp_worker_destroy(self)
	}
}
