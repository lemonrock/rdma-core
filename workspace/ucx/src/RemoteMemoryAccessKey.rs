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
	
	#[inline(always)]
	pub fn putBlocking(&self, fromLocalBuffer: *const c_void, length: usize, remoteAddress: u64)
	{
		// TODO: Review panic_on_error! - we could be getting a disconnection event!!!!
		panic_on_error!(ucp_put, self.endPoint.handle, fromLocalBuffer, length, remoteAddress, self.handle);
	}
	
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
		
		// TODO: Review panic_on_error! - we could be getting a disconnection event!!!!
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
	
	// ucp_atomic_post() / ucp_atomic_fetch_nb() (true request-non-blocking variant) => non-blocking variants of all atomic functions above...
}
