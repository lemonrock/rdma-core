// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct RemoteMemoryAccessKey<'a, 'b, 'c, ErrorHandler: EndPointErrorHandler>
where 'a: 'b, 'b: 'c, ErrorHandler: 'c
{
	handle: ucp_rkey_h,
	endPoint: &'c EndPoint<'a, 'b, ErrorHandler>
}

impl<'a, 'b, 'c, ErrorHandler: EndPointErrorHandler> Drop for RemoteMemoryAccessKey<'a, 'b, 'c, ErrorHandler>
where 'a: 'b, 'b: 'c, ErrorHandler: 'c
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_rkey_destroy(self.handle) }
	}
}

macro_rules! address_is_32_bit_aligned
{
	($remoteAddress: ident) =>
	{
		debug_assert!($remoteAddress & 0x03 == 0, "remoteAddress '{}' is not 32-bit aligned", $remoteAddress);
	}
}

macro_rules! address_is_64_bit_aligned
{
	($remoteAddress: ident) =>
	{
		debug_assert!($remoteAddress & 0x07 == 0, "remoteAddress '{}' is not 64-bit aligned", $remoteAddress);
	}
}

impl<'a, 'b, 'c, ErrorHandler: EndPointErrorHandler> RemoteMemoryAccessKey<'a, 'b, 'c, ErrorHandler>
where 'a: 'b, 'b: 'c, ErrorHandler: 'c
{
//	/// Sadly this isn't even implemented yet...
//	/// Presumably we need to have received a message telling us the remoteAddress... perhaps at the same time we get the rkey
//	#[inline(always)]
//	pub fn localMemoryAddressThatCanBeUsedToDirectLoadsAndStoresInRemoteMemory(&self, remoteAddress: *mut c_void) -> *mut c_void
//	{
//		let mut localAddress = unsafe { uninitialized() };
//		panic_on_error!(ucp_rmem_ptr, self.endPoint.handle, remoteAddress, self.handle, &mut localAddress);
//		localAddress
//	}
	
	
	
	// TODO: Review panic_on_error! - we could be getting a disconnection event!!!! UCS_INPROGRESS!!!!
	
	
	
	/*
		Failures can be grouped as:-
			- Nothing to be done - we're shot
			- Application Context recycling
			- Worker recycling
			- Endpoint recycling ucs_status_t_UCS_ERR_FIRST_ENDPOINT_FAILURE, ucs_status_t_UCS_ERR_LAST_ENDPOINT_FAILURE, ucs_status_t_UCS_ERR_ENDPOINT_TIMEOUT
				- RemoteMemoryAccessKeyRecycling
			- 'Link' (is that the worker?) - ucs_status_t_UCS_ERR_FIRST_LINK_FAILURE ucs_status_t_UCS_ERR_LAST_LINK_FAILURE
			
			- ucs_status_t_UCS_ERR_CANCELED: Should only occur if a request was cancelled, ie should only be possible via a statusPointer
			
	*/
	
	
	
//	#[inline(always)]
//	pub fn putBlocking(&self, fromLocalBuffer: *const c_void, length: usize, remoteAddress: u64) -> Result<(), ()>
//	{
//		panic_on_error_with_clean_up!
//		(
//			status,
//			{
//				if status.UCS_IS_ENDPOINT_ERROR()
//				{
//
//				}
//
//				use ucs_status_t_*;
//
//				match status
//				{
//
//				}
//			},
//			ucp_put, self.endPoint.handle, fromLocalBuffer, length, remoteAddress, self.handle
//		);
//	}
	
	/// The user MUST call flush() on the parent worker (not endpoint) before touching or freeing the fromLocalBuffer
	#[inline(always)]
	pub fn putNonBlocking(&self, fromLocalBuffer: *const c_void, length: usize, remoteAddress: u64)
	{
		// TODO: Review panic_on_error! - we could be getting a disconnection event!!!!
		panic_on_error!(ucp_put_nbi, self.endPoint.handle, fromLocalBuffer, length, remoteAddress, self.handle);
	}
	
	#[inline(always)]
	pub fn getBlocking(&self, intoLocalBuffer: *mut c_void, length: usize, remoteAddress: u64)
	{
		// TODO: Review panic_on_error! - we could be getting a disconnection event!!!!
		panic_on_error!(ucp_get, self.endPoint.handle, intoLocalBuffer, length, remoteAddress, self.handle);
	}
	
	/// The user MUST call flush() on the parent worker (not endpoint) before touching or freeing the intoLocalBuffer
	#[inline(always)]
	pub fn getNonBlocking(&self, intoLocalBuffer: *mut c_void, length: usize, remoteAddress: u64)
	{
		// TODO: Review panic_on_error! - we could be getting a disconnection event!!!!
		panic_on_error!(ucp_get_nbi, self.endPoint.handle, intoLocalBuffer, length, remoteAddress, self.handle);
	}
	
	#[inline(always)]
	pub fn putAtomic32AddBlocking(&self, amountToAdd: u32, remoteAddress: u64)
	{
		address_is_32_bit_aligned!(remoteAddress);
		
		// TODO: Review panic_on_error! - we could be getting a disconnection event!!!!
		panic_on_error!(ucp_atomic_post, self.endPoint.handle, ucp_atomic_post_op_t::UCP_ATOMIC_POST_OP_ADD, amountToAdd as u64, 4, remoteAddress, self.handle);
	}
	
	#[inline(always)]
	pub fn putAtomic64AddBlocking(&self, amountToAdd: u64, remoteAddress: u64)
	{
		address_is_32_bit_aligned!(remoteAddress);
		
		// TODO: Review panic_on_error! - we could be getting a disconnection event!!!!
		panic_on_error!(ucp_atomic_post, self.endPoint.handle, ucp_atomic_post_op_t::UCP_ATOMIC_POST_OP_ADD, amountToAdd, 8, remoteAddress, self.handle);
	}
	
	/// The user MUST call flush() on the parent worker (not endpoint) to be certain the operation has completed
	#[inline(always)]
	pub fn putAtomic32AddNonBlocking(&self, amountToAdd: u32, remoteAddress: u64)
	{
		address_is_32_bit_aligned!(remoteAddress);
		
		// TODO: Review panic_on_error! - we could be getting a disconnection event!!!!
		panic_on_error!(ucp_atomic_add32, self.endPoint.handle, amountToAdd, remoteAddress, self.handle);
	}
	
	/// The user MUST call flush() on the parent worker (not endpoint) to be certain the operation has completed
	#[inline(always)]
	pub fn putAtomic64AddNonBlocking(&self, amountToAdd: u64, remoteAddress: u64)
	{
		address_is_64_bit_aligned!(remoteAddress);
		
		// TODO: Review panic_on_error! - we could be getting a disconnection event!!!!
		panic_on_error!(ucp_atomic_add64, self.endPoint.handle, amountToAdd, remoteAddress, self.handle);
	}
	
	/// Returns the original remote value before amountToAdd was added
	#[inline(always)]
	pub fn putAtomic32FetchAndAddBlocking(&self, amountToAdd: u32, remoteAddress: u64) -> u32
	{
		address_is_32_bit_aligned!(remoteAddress);
		
		// TODO: Review panic_on_error! - we could be getting a disconnection event!!!! UCS_INPROGRESS!!!!
		let mut result = unsafe { uninitialized() };
		panic_on_error!(ucp_atomic_fadd32, self.endPoint.handle, amountToAdd, remoteAddress, self.handle, &mut result);
		result
	}
	
	/// Returns the original remote value before amountToAdd was added
	#[inline(always)]
	pub fn putAtomic64FetchAndAddBlocking(&self, amountToAdd: u64, remoteAddress: u64) -> u64
	{
		address_is_64_bit_aligned!(remoteAddress);
		
		// TODO: Review panic_on_error! - we could be getting a disconnection event!!!!
		let mut result = unsafe { uninitialized() };
		panic_on_error!(ucp_atomic_fadd64, self.endPoint.handle, amountToAdd, remoteAddress, self.handle, &mut result);
		result
	}
	
	/// Returns the original remote value before swapRemoteWith was sent
	#[inline(always)]
	pub fn putAtomic32SwapBlocking(&self, swapRemoteWith: u32, remoteAddress: u64) -> u32
	{
		address_is_32_bit_aligned!(remoteAddress);
		
		// TODO: Review panic_on_error! - we could be getting a disconnection event!!!!
		let mut result = unsafe { uninitialized() };
		panic_on_error!(ucp_atomic_swap32, self.endPoint.handle, swapRemoteWith, remoteAddress, self.handle, &mut result);
		result
	}
	
	/// Returns the original remote value before swapRemoteWith was sent
	#[inline(always)]
	pub fn putAtomic64SwapBlocking(&self, swapRemoteWith: u64, remoteAddress: u64) -> u64
	{
		address_is_64_bit_aligned!(remoteAddress);
		
		// TODO: Review panic_on_error! - we could be getting a disconnection event!!!!
		let mut result = unsafe { uninitialized() };
		panic_on_error!(ucp_atomic_swap64, self.endPoint.handle, swapRemoteWith, remoteAddress, self.handle, &mut result);
		result
	}
	
	/// Returns the original remote value before swapRemoteWith was sent; only updates remote it remote matches compareRemoteWith
	/// This is compareRemoteWith == value returned => swap occurred
	#[inline(always)]
	pub fn putAtomic32CompareAndSwapBlocking(&self, compareRemoteWith: u32, swapRemoteWith: u32, remoteAddress: u64) -> u32
	{
		address_is_32_bit_aligned!(remoteAddress);
		
		// TODO: Review panic_on_error! - we could be getting a disconnection event!!!!
		let mut result = unsafe { uninitialized() };
		panic_on_error!(ucp_atomic_cswap32, self.endPoint.handle, compareRemoteWith, swapRemoteWith, remoteAddress, self.handle, &mut result);
		result
	}
	
	/// Returns the original remote value before swapRemoteWith was sent; only updates remote it remote matches compareRemoteWith
	/// This is compareRemoteWith == value returned => swap occurred
	#[inline(always)]
	pub fn putAtomic64CompareAndSwapBlocking(&self, compareRemoteWith: u64, swapRemoteWith: u64, remoteAddress: u64) -> u64
	{
		address_is_64_bit_aligned!(remoteAddress);
		
		// TODO: Review panic_on_error! - we could be getting a disconnection event!!!!
		let mut result = unsafe { uninitialized() };
		panic_on_error!(ucp_atomic_cswap64, self.endPoint.handle, compareRemoteWith, swapRemoteWith, remoteAddress, self.handle, &mut result);
		result
	}
	
	// result does not need to be initialized or zeroed
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn putAtomic32FetchAndAddNonBlocking(&self, amountToAdd: u32, remoteAddress: u64, callbackWhichIsCalledOnlyIfResultNotImmediatelyAvailable: unsafe extern "C" fn(request: *mut c_void, status: ucs_status_t)) -> (NonBlockingRequest, Box<u32>)
	{
		address_is_32_bit_aligned!(remoteAddress);
		
		let mut result: Box<u32> = Box::new(unsafe { uninitialized() });
		
		let statusPointer = unsafe { ucp_atomic_fetch_nb(self.endPoint.handle, ucp_atomic_fetch_op_t::UCP_ATOMIC_FETCH_OP_FADD, amountToAdd as u64, result.as_mut() as *mut u32 as *mut c_void, 4, remoteAddress, self.handle, Some(callbackWhichIsCalledOnlyIfResultNotImmediatelyAvailable)) };
		(NonBlockingRequest(statusPointer), result)
	}
	
	// result does not need to be initialized or zeroed
//	#[inline(always)]
//	pub fn putAtomic64FetchAndAddNonBlocking(&self, amountToAdd: u64, remoteAddress: u64, result: &mut u64, callbackWhichIsCalledOnlyIfResultNotImmediatelyAvailable: unsafe extern "C" fn(request: *mut c_void, status: ucs_status_t)) -> (NonBlockingRequest, &mut u64)
//	{
//		address_is_64_bit_aligned!(remoteAddress);
//
//		let statusPointer = unsafe { ucp_atomic_fetch_nb(self.endPoint.handle, ucp_atomic_fetch_op_t::UCP_ATOMIC_FETCH_OP_FADD, amountToAdd, result as *mut _ as *mut c_void, 8, remoteAddress, self.handle, Some(callbackWhichIsCalledOnlyIfResultNotImmediatelyAvailable)) };
//		(NonBlockingRequest(statusPointer), result)
//	}

}
