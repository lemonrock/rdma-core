// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct ProtectionDomain<'a>
{
	pointer: *mut ibv_pd,
	context: &'a Context,
}

impl<'a> Drop for ProtectionDomain<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_errno!(ibv_dealloc_pd, self.pointer);
	}
}

impl<'a> ProtectionDomain<'a>
{
	#[inline(always)]
	fn new(pointer: *mut ibv_pd, context: &'a Context) -> Self
	{
		debug_assert!(!pointer.is_null(), "pointer is null");
		
		Self
		{
			pointer: pointer,
			context: context,
		}
	}
	
	#[inline(always)]
	pub fn createSharedRequestQueue(&'a self, requestedSettings: SharedRequestQueueSettings) -> SharedRequestQueue<'a>
	{
		let mut attributes = ibv_srq_init_attr
		{
			srq_context: null_mut(),
			attr: ibv_srq_attr
			{
				max_wr: requestedSettings.maximumNumberOfOutstandingWorkRequestsInInTheSharedRequestQueue,
				max_sge: requestedSettings.maximumNumberOfScatterElementsPerWorkRequest,
				srq_limit: 0,
			}
		};
		
		let pointer = panic_on_null!(ibv_create_srq, self.pointer, &mut attributes);
		SharedRequestQueue
		{
			pointer: pointer,
			settings: SharedRequestQueueSettings
			{
				maximumNumberOfOutstandingWorkRequestsInInTheSharedRequestQueue: attributes.attr.max_wr,
				maximumNumberOfScatterElementsPerWorkRequest: attributes.attr.max_sge,
			},
			protectionDomain: self,
		}
	}
	
	#[inline(always)]
	pub fn registerMemoryRegion(&'a self, address: *mut c_void, length: usize, access: &MemoryRegionAccess) -> MemoryRegion<'a>
	{
		debug_assert!(!address.is_null(), "address can not be null");
		debug_assert!(length != 0, "length can not be zero");
		debug_assert!(length as u64 <= self.context.attributes().max_mr_size, "length '{}' exceeds device maximum '{}'", length, self.context.attributes().max_mr_size);
		
		let pointer = panic_on_null!(ibv_reg_mr, self.pointer, address, length, access.as_c_int());
		MemoryRegion
		{
			pointer: pointer,
			lifetime: PhantomData,
		}
	}
	
	#[inline(always)]
	pub fn createAddressHandleForPort(&'a self, portNumber: u8, workCompletion: &mut WorkCompletion, globalRoutingHeader: &mut GlobalRoutingHeader) -> AddressHandle<'a>
	{
		let mut attributes = self.context.port(portNumber).initialiseAddressHandleAttributes(workCompletion, globalRoutingHeader);
		self.createAddressHandle(&mut attributes)
	}
	
	#[inline(always)]
	pub fn createAddressHandle(&'a self, attributes: &mut ibv_ah_attr) -> AddressHandle<'a>
	{
		debug_assert!(attributes.port_num < self.context.numberOfPhysicalPorts(), "port number '{}' is not less than the maximum '{}'", attributes.port_num, self.context.numberOfPhysicalPorts());
		
		let pointer = panic_on_null!(ibv_create_ah, self.pointer, attributes);
		AddressHandle
		{
			pointer: pointer,
			protectionDomain: self,
		}
	}
}
