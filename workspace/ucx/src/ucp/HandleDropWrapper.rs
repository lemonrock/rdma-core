// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct HandleDropWrapper<Handle: HandleDrop + Copy>(Handle);

impl<Handle: HandleDrop + Copy> Drop for HandleDropWrapper<Handle>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { Handle::handleDrop(self.0) }
	}
}

impl<Handle: HandleDrop + Copy> HandleDropWrapper<Handle>
{
	#[inline(always)]
	fn new(handle: Handle) -> Rc<Self>
	{
		Rc::new(HandleDropWrapper(handle))
	}
}
