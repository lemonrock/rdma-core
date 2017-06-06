// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct ApplicationContext
{
	handle: ucp_context_h,
}

impl Drop for ApplicationContext
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_cleanup(self.handle) };
	}
}

impl PrintInformation for ApplicationContext
{
	#[inline(always)]
	fn printInformationToStream(&self, stream: *mut FILE)
	{
		unsafe { ucp_context_print_info(self.handle, stream) };
	}
}

impl QueryAttributes for ApplicationContext
{
	type Attributes = ApplicationContextAttributes;
	
	#[inline(always)]
	fn queryAttributes(&self) -> Self::Attributes
	{
		use ucp_context_attr_field::*;
		
		let mut attributes: ucp_context_attr_t = unsafe { uninitialized() };
		attributes.field_mask = UCP_ATTR_FIELD_REQUEST_SIZE as u64 | UCP_ATTR_FIELD_THREAD_MODE as u64;
		panic_on_error!(ucp_context_query, self.handle, &mut attributes);
		ApplicationContextAttributes(attributes)
	}
}

impl ApplicationContext
{
	#[inline(always)]
	pub fn mapAndAllocateMemory<'a>(&'a self, length: usize) -> (MappedMemory<'a>, *mut c_void)
	{
		debug_assert!(length != 0, "length is zero");
		
		use ucp_mem_map_params_field::*;
		
		let parameters = ucp_mem_map_params_t
		{
			field_mask: UCP_MEM_MAP_PARAM_FIELD_LENGTH as u64 | UCP_MEM_MAP_PARAM_FIELD_FLAGS as u64,
			address: null_mut(),
			length: length,
			flags: UCP_MEM_MAP_ALLOCATE.0 | UCP_MEM_MAP_NONBLOCK.0,
		};
		
		let mut memh = unsafe { uninitialized() };
		panic_on_error!(ucp_mem_map, self.handle, &parameters, &mut memh);
		let mappedMemory = MappedMemory
		{
			handle: memh,
			applicationContext: self,
		};
		(mappedMemory, parameters.address)
	}
	
	#[inline(always)]
	pub fn mapMemory<'a>(&'a self, address: *mut c_void, length: usize) -> MappedMemory<'a>
	{
		debug_assert!(!address.is_null(), "address is null");
		debug_assert!(length != 0, "length is zero");
		
		use ucp_mem_map_params_field::*;
		
		let parameters = ucp_mem_map_params_t
		{
			field_mask: UCP_MEM_MAP_PARAM_FIELD_ADDRESS as u64 | UCP_MEM_MAP_PARAM_FIELD_LENGTH as u64,
			address: address,
			length: length,
			flags: 0,
		};
		
		let mut memh = unsafe { uninitialized() };
		panic_on_error!(ucp_mem_map, self.handle, &parameters, &mut memh);
		MappedMemory
		{
			handle: memh,
			applicationContext: self,
		}
	}
	
	#[inline(always)]
	pub fn createWorker<'a>(&'a self, workerThreadMode: WorkerThreadMode) -> Worker<'a>
	{
		use ucp_worker_params_field::*;
		use ucp_wakeup_event_types::*;
		
		let parameters = ucp_worker_params_t
		{
			field_mask: UCP_WORKER_PARAM_FIELD_THREAD_MODE as u64 | UCP_WORKER_PARAM_FIELD_CPU_MASK as u64 | UCP_WORKER_PARAM_FIELD_EVENTS as u64,
			thread_mode: workerThreadMode.as_ucs_thread_mode_t(),
			cpu_mask: ucs_cpu_set_t::createForCurrentCpuIndex(),
			events: UCP_WAKEUP_RMA as u32 | UCP_WAKEUP_AMO as u32 | UCP_WAKEUP_TAG_SEND as u32 | UCP_WAKEUP_TAG_RECV as u32,
		};
		let mut worker = unsafe { uninitialized() };
		panic_on_error!(ucp_worker_create, self.handle, &parameters, &mut worker);
		
		Worker
		{
			handle: worker,
			applicationContext: self,
		}
	}
}
