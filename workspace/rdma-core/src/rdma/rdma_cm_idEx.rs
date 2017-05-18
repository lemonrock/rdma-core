// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait rdma_cm_idEx
{
	#[inline(always)]
	fn contextIsNotNull(self) -> bool;
	
	#[inline(always)]
	fn reconstituteContext(self) -> Box<Box<CommunicationIdentifierContext>>;
	
	#[inline(always)]
	fn destroy(self);
	
	#[inline(always)]
	fn portSpace(self) -> rdma_port_space;
}

impl rdma_cm_idEx for *mut rdma_cm_id
{
	#[inline(always)]
	fn contextIsNotNull(self) -> bool
	{
		!unsafe { (*self).context }.is_null()
	}
	
	#[inline(always)]
	fn reconstituteContext(self) -> Box<Box<CommunicationIdentifierContext>>
	{
		debug_assert!(self.contextIsNotNull(), "context is null");
		
		unsafe { Box::from_raw((*self).context as *mut Box<CommunicationIdentifierContext>) }
	}
	
	#[inline(always)]
	fn destroy(self)
	{
		if likely(self.contextIsNotNull())
		{
			drop(self.reconstituteContext());
		}
		
		panic_on_error!(rdma_destroy_id, self);
	}
	
	#[inline(always)]
	fn portSpace(self) -> rdma_port_space
	{
		unsafe { (*self).ps }
	}
}
