// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum uct_alloc_method_t
{
	UCT_ALLOC_METHOD_MD = 0,
	UCT_ALLOC_METHOD_HEAP = 1,
	UCT_ALLOC_METHOD_MMAP = 2,
	UCT_ALLOC_METHOD_HUGE = 3,
	UCT_ALLOC_METHOD_LAST = 4,
}
