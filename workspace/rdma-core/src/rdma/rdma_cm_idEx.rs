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
	fn portNumber(self) -> u8;
	
	#[inline(always)]
	fn portSpace(self) -> rdma_port_space;
	
	#[inline(always)]
	fn rejectWithPrivateData(self, privateData: &[u8; 256], privateDataLength: u8);
	
	#[inline(always)]
	fn rejectWithoutPrivateData(self);
	
	#[inline(always)]
	fn accept(self, parameter: &mut rdma_conn_param);
	
	#[inline(always)]
	fn disconnect(self);
	
	#[inline(always)]
	fn setTypeOfService(self, toS: u8);
}

impl HasContextPointer for *mut rdma_cm_id
{
	#[inline(always)]
	fn getContext(self) -> *mut c_void
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).context }
	}
	
	#[inline(always)]
	fn setContext(self, context: *mut c_void)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).context = context };
	}
}

impl HasVerbsPointer for *mut rdma_cm_id
{
	#[inline(always)]
	fn verbs(self) -> *mut ibv_context
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).verbs }
	}
}

impl HasProtectionDomainPointer for *mut rdma_cm_id
{
	#[inline(always)]
	fn protectionDomain(self) -> *mut ibv_pd
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).pd }
	}
}

impl HasQueuePairType for *mut rdma_cm_id
{
	#[inline(always)]
	fn queuePairType(self) -> ibv_qp_type
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).qp_type }
	}
}

impl rdma_cm_idEx for *mut rdma_cm_id
{
	#[inline(always)]
	fn contextIsNotNull(self) -> bool
	{
		debug_assert!(!self.is_null(), "self is null");
		
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
		debug_assert!(!self.is_null(), "self is null");
		
		if likely(self.contextIsNotNull())
		{
			drop(self.reconstituteContext());
		}
		
		panic_on_error!(rdma_destroy_id, self);
	}
	
	#[inline(always)]
	fn portNumber(self) -> u8
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).port_num }
	}
	
	#[inline(always)]
	fn portSpace(self) -> rdma_port_space
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).ps }
	}
	
	#[inline(always)]
	fn rejectWithPrivateData(self, privateData: &[u8; 256], privateDataLength: u8)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		panic_on_error!(rdma_reject, self, privateData as *const u8 as *const _, privateDataLength);
	}
	
	#[inline(always)]
	fn rejectWithoutPrivateData(self)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		panic_on_error!(rdma_reject, self, null(), 0);
	}
	
	#[inline(always)]
	fn accept(self, parameter: &mut rdma_conn_param)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		panic_on_error!(rdma_accept, self, parameter);
	}
	
	#[inline(always)]
	fn disconnect(self)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		panic_on_error!(rdma_disconnect, self);
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	fn setTypeOfService(self, mut typeOfService: u8)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		panic_on_error!(rdma_set_option, self, RDMA_OPTION_ID as i32, RDMA_OPTION_ID_TOS as i32, &mut typeOfService as *mut _ as *mut c_void, size_of::<u8>());
	}
}
