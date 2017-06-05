// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;
use ::libc::c_void;
use ::rust_extra::likely;
use ::std::marker::PhantomData;
use ::std::mem::forget;
use ::std::panic::AssertUnwindSafe;
use ::std::panic::catch_unwind;
use ::std::ptr::null_mut;


pub enum TaggedState<T: Sized, P: PackState<T>, U: UnpackState<T>>
{
	Pack(P, PhantomData<T>),
	Unpack(U, PhantomData<T>),
}

pub trait State<T: Sized>: Drop
{
	fn finish(&mut self);
}

pub trait PackState<T: Sized>: State<T>
{
	fn pack(&mut self, offset: usize, destination: *mut c_void, maximumLength: usize) -> usize;
	
	fn packedSize(&mut self) -> usize;
}

pub trait UnpackState<T: Sized>: State<T>
{
	/// This can be called more than once
	fn unpack(&mut self, offset: usize, source: *const c_void, count: usize) -> bool;
}

pub trait Serialiser<T: Sized>
{
	type P: PackState<T>;
	
	fn start(&mut self, buffer: *const c_void, count: usize) -> Self::P;
}

pub trait Deserialiser<T: Sized>
{
	type U: UnpackState<T>;
	
	fn start(&mut self, buffer: *mut c_void, count: usize) -> Self::U;
}

pub struct GenericDataTypeOperations<T: Sized, S: Serialiser<T>, D: Deserialiser<T>>
{
	serializer: S,
	deserializer: D,
	marker: PhantomData<T>,
}

impl<T: Sized, S: Serialiser<T>, D: Deserialiser<T>> GenericDataTypeOperations<T, S, D>
{
	#[inline(always)]
	pub fn as_ucp_generic_dt_ops(&mut self) -> ucp_generic_dt_ops
	{
		ucp_generic_dt_ops
		{
			start_pack: Some(Self::start_pack),
			start_unpack: Some(Self::start_unpack),
			packed_size: Some(Self::packed_size),
			pack: Some(Self::pack),
			unpack: Some(Self::unpack),
			finish: Some(Self::finish),
		}
	}
	
	unsafe extern "C" fn start_pack(context: *mut c_void, buffer: *const c_void, count: usize) -> *mut c_void
	{
		let mut this = Box::from_raw(context as *mut Self);
		
		let resultantState = match catch_unwind(AssertUnwindSafe(||
		{
			this.serializer.start(buffer, count)
		}))
		{
			Ok(state) =>
			{
				let t: TaggedState<T, S::P, D::U> = TaggedState::Pack(state, PhantomData);
				let b: Box<TaggedState<T, S::P, D::U>> = Box::new(t);
				Box::into_raw(b) as *mut c_void
			},
			Err(_) => null_mut(),
		};
		
		forget(this);
		
		resultantState
	}
	
	unsafe extern "C" fn start_unpack(context: *mut c_void, buffer: *mut c_void, count: usize) -> *mut c_void
	{
		let mut this = Box::from_raw(context as *mut Self);
		
		let resultantState = match catch_unwind(AssertUnwindSafe(||
		{
			this.deserializer.start(buffer, count)
		}))
		{
			Ok(state) =>
			{
				let t: TaggedState<T, S::P, D::U> = TaggedState::Unpack(state, PhantomData);
				let b: Box<TaggedState<T, S::P, D::U>> = Box::new(t);
				Box::into_raw(b) as *mut c_void
			},
			Err(_) => null_mut(),
		};
		
		forget(this);
		
		resultantState
	}
	
	unsafe extern "C" fn packed_size(state: *mut c_void) -> usize
	{
		if state.is_null()
		{
			return 0;
		}
		
		let mut taggedState = Box::from_raw(state as *mut TaggedState<T, S::P, D::U>);
		let resultantSize = match catch_unwind(AssertUnwindSafe(||
		{
			match *taggedState
			{
				TaggedState::Pack(ref mut state, _) => state.packedSize(),
				TaggedState::Unpack(_, _) => panic!("Should not be possible"),
			}
		}))
		{
			Ok(size) => size,
			Err(_) => 0,
		};
		
		forget(taggedState);
		
		resultantSize
	}
	
	unsafe extern "C" fn pack(state: *mut c_void, offset: usize, dest: *mut c_void, max_length: usize) -> usize
	{
		if state.is_null()
		{
			return 0;
		}
		
		let mut taggedState = Box::from_raw(state as *mut TaggedState<T, S::P, D::U>);
		let resultantSize = match catch_unwind(AssertUnwindSafe(||
		{
			match *taggedState
			{
				TaggedState::Pack(ref mut state, _) =>
				{
					let size = state.pack(offset, dest, max_length);
					debug_assert!(size <= max_length, "size '{}' exceeds max_length '{}'", size, max_length);
					size
				},
				TaggedState::Unpack(_, _) => panic!("Should not be possible"),
			}
		}))
		{
			Ok(size) => size,
			Err(_) => 0,
		};
		
		forget(taggedState);
		
		resultantSize
	}
	
	unsafe extern "C" fn unpack(state: *mut c_void, offset: usize, src: *const c_void, count: usize) -> ucs_status_t
	{
		if state.is_null()
		{
			return ucs_status_t::UCS_ERR_IO_ERROR;
		}
		
		let mut taggedState = Box::from_raw(state as *mut TaggedState<T, S::P, D::U>);
		let resultantStatusCode = match catch_unwind(AssertUnwindSafe(||
		{
			match *taggedState
			{
				TaggedState::Pack(_, _) => panic!("Should not be possible"),
				TaggedState::Unpack(ref mut state, _) =>
				{
					if likely(state.unpack(offset, src, count))
					{
						ucs_status_t::UCS_OK
					}
					else
					{
						ucs_status_t::UCS_ERR_NO_MEMORY
					}
				},
			}
		}))
		{
			Ok(size) => size,
			Err(_) => ucs_status_t::UCS_ERR_INVALID_PARAM,
		};
		
		forget(taggedState);
		
		resultantStatusCode
	}
	
	unsafe extern "C" fn finish(state: *mut c_void)
	{
		if state.is_null()
		{
			return;
		}
		
		let taggedState = Box::from_raw(state as *mut TaggedState<T, S::P, D::U>);
		drop(taggedState)
	}
}
