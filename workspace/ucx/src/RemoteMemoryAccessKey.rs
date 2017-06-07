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

#[repr(i8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TransientFailureReason
{
	/// Only seems to be relevant to receiving
	/// Does not seem to ever escape stats internal code
	NoPendingMessage = ucs_status_t_UCS_ERR_NO_MESSAGE,
	
	/// Seems to be caused by flush(); try-again
	NoResource = ucs_status_t_UCS_ERR_NO_RESOURCE,
	
	/// Usually called because we can not connect to the remote memory access key (rkey)
	/// Tear down RemoteMemoryAccessKey and probably EndPoint
	DestinationAddressIsUnreachable = ucs_status_t_UCS_ERR_UNREACHABLE,
	
	/// Tear down RemoteMemoryAccessKey and probably EndPoint
	EndPointTimeOut = ucs_status_t_UCS_ERR_ENDPOINT_TIMEOUT,
}

#[repr(i8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PermanentFailureReason
{
	OutOfMemory = ucs_status_t_UCS_ERR_NO_MEMORY,
	
	InvalidParameter = ucs_status_t_UCS_ERR_INVALID_PARAM,
	
	/// Failures with this reason occur early on because the address is just plain wrong
	/// *Except* for the UCS stats client, which returns this is gethostaddr() fails, ie if DNS fails
	InvalidRemoteAddressOrTcpAddressIsNotIpV6OrCanNotPackIntoRemoteAddressBuffer = ucs_status_t_UCS_ERR_INVALID_ADDR,
	
	UnimplementedFunctionality = ucs_status_t_UCS_ERR_NOT_IMPLEMENTED,
	
	/// Apart from configuration-time discovering that there are no devices (ucs_error), seems to indicate programming failure
	NoSuchElement = ucs_status_t_UCS_ERR_NO_ELEM,
	
	/// Should occur quite early on; in essence, there are no suitable devices available for a given transport, eg we asked for InfiniBand and there are no InfiniBand cards / ports
	NoTransportDeviceExists = ucs_status_t_UCS_ERR_NO_DEVICE,
	
	/// Differs to UnimplementedFunctionality in that a particular function exists but a particular path of (reasonable from an user's perspective) logic through it is not supported
	UnsupportedSubSetOfFunctionality = ucs_status_t_UCS_ERR_UNSUPPORTED,
	
	/// Is used for open, truncate, read, write, close and delete; hides errors from calls like open() and shmat()
	/// Whilst in theory some of these errors are probably transient or recoverable, in practice, since we don't have any knowledge from ucx about what it was doing, we can't
	PosixOrSysVSharedMemoryError = ucs_status_t_UCS_ERR_SHMEM_SEGMENT,
}

/// Represents errors that just don't occur despite being defined in the API
#[repr(i8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImpossibleFailureReason
{
	BufferTooSmall = ucs_status_t_UCS_ERR_BUFFER_TOO_SMALL,
	
	FailedToConnectToSomeOfTheRequestedEndPoints = ucs_status_t_UCS_ERR_SOME_CONNECTS_FAILED,
}

quick_error!
{
	#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub enum RemoteMemoryAccessFailure
	{
//		Progress
//		{
//			display("In progress")
//		}

		Transient(transientFailureReason: TransientFailureReason)
		{
			display("Transient failure '{:?}'", transientFailureReason)
		}
		
		Permanent(permanentFailureReason: PermanentFailureReason)
		{
			display("Permanent failure '{:?}'", permanentFailureReason)
		}
		
		Impossible(impossibleFailureReason: ImpossibleFailureReason)
		{
			display("Impossible failure '{:?}'", impossibleFailureReason)
		}
		
		Future(offset: u8)
		{
			display("offset was '{}'", offset)
		}
		
		Link(offset: u8)
		{
			display("offset was '{}'", offset)
		}
		
		EndPoint(offset: u8)
		{
			display("offset was '{}'", offset)
		}
		
		Unknown(offset: u8)
		{
			display("offset was '{}'", offset)
		}
	}
}

impl RemoteMemoryAccessFailure
{
	const FirstFutureError: ucs_status_t = ucs_status_t_UCS_ERR_UNSUPPORTED - 1;
	
	const LastFutureError: ucs_status_t = ucs_status_t_UCS_ERR_FIRST_LINK_FAILURE + 1;
	
	const FirstUnknownError: ucs_status_t = ucs_status_t_UCS_ERR_ENDPOINT_TIMEOUT - 1;
	
	#[inline(always)]
	pub fn as_ucs_status_t(&self) -> ucs_status_t
	{
		use RemoteMemoryAccessFailure::*;
		
		match *self
		{
			Transient(transientFailureReason) => transientFailureReason as i8,
			Permanent(permanentFailureReason) => permanentFailureReason as i8,
			Impossible(impossibleFailureReason) => impossibleFailureReason as i8,
			Future(offset) => Self::FirstFutureError - (offset as i8),
			Link(offset) => ucs_status_t_UCS_ERR_FIRST_LINK_FAILURE - (offset as i8),
			EndPoint(offset) => ucs_status_t_UCS_ERR_FIRST_ENDPOINT_FAILURE - (offset as i8),
			Unknown(offset) => Self::FirstUnknownError - (offset as i8),
		}
	}
	
	#[inline(always)]
	pub fn convertError(status: ucs_status_t) -> RemoteMemoryAccessFailure
	{
		debug_assert!(status < 0, "convert does not support UCS_OK or UCS_INPROGRESS");
		
		match status
		{
			ucs_status_t_UCS_ERR_NO_MESSAGE => RemoteMemoryAccessFailure::Transient(TransientFailureReason::NoPendingMessage),
			ucs_status_t_UCS_ERR_NO_RESOURCE => RemoteMemoryAccessFailure::Transient(TransientFailureReason::NoResource),
//			ucs_status_t_UCS_ERR_IO_ERROR = -3,
			ucs_status_t_UCS_ERR_NO_MEMORY => RemoteMemoryAccessFailure::Permanent(PermanentFailureReason::OutOfMemory),
			ucs_status_t_UCS_ERR_INVALID_PARAM => RemoteMemoryAccessFailure::Permanent(PermanentFailureReason::InvalidParameter),
			ucs_status_t_UCS_ERR_UNREACHABLE => RemoteMemoryAccessFailure::Transient(TransientFailureReason::DestinationAddressIsUnreachable),
			ucs_status_t_UCS_ERR_INVALID_ADDR => RemoteMemoryAccessFailure::Permanent(PermanentFailureReason::InvalidRemoteAddressOrTcpAddressIsNotIpV6OrCanNotPackIntoRemoteAddressBuffer),
			ucs_status_t_UCS_ERR_NOT_IMPLEMENTED => RemoteMemoryAccessFailure::Permanent(PermanentFailureReason::UnimplementedFunctionality),
//			ucs_status_t_UCS_ERR_MESSAGE_TRUNCATED = -9,
//			ucs_status_t_UCS_ERR_NO_PROGRESS = -10,
			ucs_status_t_UCS_ERR_BUFFER_TOO_SMALL => RemoteMemoryAccessFailure::Impossible(ImpossibleFailureReason::BufferTooSmall),
			ucs_status_t_UCS_ERR_NO_ELEM => RemoteMemoryAccessFailure::Permanent(PermanentFailureReason::NoSuchElement),
			ucs_status_t_UCS_ERR_SOME_CONNECTS_FAILED => RemoteMemoryAccessFailure::Impossible(ImpossibleFailureReason::FailedToConnectToSomeOfTheRequestedEndPoints),
			ucs_status_t_UCS_ERR_NO_DEVICE => RemoteMemoryAccessFailure::Permanent(PermanentFailureReason::NoTransportDeviceExists),
//			ucs_status_t_UCS_ERR_BUSY = -15,
//			ucs_status_t_UCS_ERR_CANCELED = -16,
			ucs_status_t_UCS_ERR_SHMEM_SEGMENT => RemoteMemoryAccessFailure::Permanent(PermanentFailureReason::PosixOrSysVSharedMemoryError),
//			ucs_status_t_UCS_ERR_ALREADY_EXISTS = -18,
//			ucs_status_t_UCS_ERR_OUT_OF_RANGE = -19,
//			ucs_status_t_UCS_ERR_TIMED_OUT = -20,
//			ucs_status_t_UCS_ERR_EXCEEDS_LIMIT = -21,
			ucs_status_t_UCS_ERR_UNSUPPORTED => RemoteMemoryAccessFailure::Permanent(PermanentFailureReason::UnsupportedSubSetOfFunctionality),
			Self::LastFutureError ... Self::FirstFutureError => RemoteMemoryAccessFailure::Future(-(status - Self::FirstFutureError) as u8),
			ucs_status_t_UCS_ERR_LAST_LINK_FAILURE ... ucs_status_t_UCS_ERR_FIRST_LINK_FAILURE => RemoteMemoryAccessFailure::Link(-(status - ucs_status_t_UCS_ERR_FIRST_LINK_FAILURE) as u8),
			ucs_status_t_UCS_ERR_LAST_ENDPOINT_FAILURE ... ucs_status_t_UCS_ERR_FIRST_ENDPOINT_FAILURE => RemoteMemoryAccessFailure::EndPoint(-(status - ucs_status_t_UCS_ERR_FIRST_ENDPOINT_FAILURE) as u8),
			ucs_status_t_UCS_ERR_ENDPOINT_TIMEOUT => RemoteMemoryAccessFailure::Transient(TransientFailureReason::EndPointTimeOut),
			ucs_status_t_UCS_ERR_LAST ... Self::FirstUnknownError => RemoteMemoryAccessFailure::Unknown(-(status - Self::FirstUnknownError) as u8),
			_ => panic!("Unknown status '{}'", status),
		}
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
