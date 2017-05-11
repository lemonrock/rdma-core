// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#![feature(static_nobundle)]
#![feature(untagged_unions)]

#![allow(missing_copy_implementations)]
#![allow(missing_debug_implementations)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![warn(trivial_casts)]
#![warn(unused_import_braces)]


extern crate errno;
extern crate libc;
extern crate rdma_core_sys;
extern crate rust_extra;
extern crate syscall_alt;


use ::errno::errno;
use ::rdma_core_sys::*;
use ::rust_extra::unlikely;
use ::std::ffi::CStr;
use ::std::marker::PhantomData;
use ::std::mem::uninitialized;
use ::syscall_alt::constants::E;


include!("Device.rs");
include!("DeviceListIterator.rs");
include!("GUID.rs");


macro_rules! panic_on_error
{
	($function: path$(,$argument: expr)*) =>
	{
		{
			let result = unsafe { $function($($argument),*) };
			debug_assert!(result == 0 || result == 1, "{} returned a result '{}' which was not 0 or 1", stringify!($function), result);
			if $crate::rust_extra::unlikely(result == 1)
			{
				let errno = errno();
				panic!("{} failed with error number '{}' ('{}')", stringify!($function), errno.0, errno);
			}
		}
	}
}

pub struct Context(*mut ibv_context, ibv_device_attr);

impl Drop for Context
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let result = unsafe { ibv_close_device(self.0) };
		debug_assert!(result == 0 || result == 1, "ibv_close_device returned a result '{}' which was not 0 or 1", result);
		if unlikely(result == -1)
		{
			// Examination of C source code suggests it can't actually fail
			panic!("ibv_close_device failed (no further details)");
		}
	}
}

impl Context
{
	#[inline(always)]
	fn new(pointer: *mut ibv_context) -> Self
	{
		debug_assert!(!pointer.is_null(), "pointer is null");
		
		let mut attributes = unsafe { uninitialized() };
		panic_on_error!(ibv_query_device, pointer, &mut attributes);
		
		Context(pointer, attributes)
	}
	
	/// See <https://linux.die.net/man/3/ibv_query_device> for explanations of fields of ibv_device_attr
	#[inline(always)]
	pub fn attributes(&self) -> &ibv_device_attr
	{
		&self.1
	}
	
	/// See <https://linux.die.net/man/3/ibv_get_async_event>
	#[inline(always)]
	pub fn blockOnAsynchronousEvent(&self) -> AsynchronousEvent
	{
		let mut asynchronousEvent = unsafe { uninitialized() };
		panic_on_error!(ibv_get_async_event, self.0, &mut asynchronousEvent);
		AsynchronousEvent(asynchronousEvent)
	}
	
	#[inline(always)]
	pub fn port<'a>(&'a self, portNumber: u8) -> Port<'a>
	{
		debug_assert!(portNumber < self.1.phys_port_cnt, "portNumber '{}' exceeds maximum number of ports '{}'", portNumber, self.1.phys_port_cnt);
		
		Port::new(self, portNumber)
	}
	
	/*
		pub fn ibv_alloc_pd(context: *mut ibv_context) -> *mut ibv_pd;
		
		pub fn ibv_create_comp_channel(context: *mut ibv_context) -> *mut ibv_comp_channel;
		
		pub fn ibv_create_cq(context: *mut ibv_context, cqe: c_int, cq_context: *mut c_void, channel: *mut ibv_comp_channel, comp_vector: c_int) -> *mut ibv_cq;
	*/
}

pub struct AsynchronousEvent(ibv_async_event);

impl Drop for AsynchronousEvent
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ibv_ack_async_event(&mut self.0) }
	}
}

#[derive(Copy, Clone)]
pub struct Port<'a>
{
	context: &'a Context,
	portNumber: u8,
	attributes: ibv_port_attr,
}

impl<'a> Port<'a>
{
	#[inline(always)]
	fn new(context: &'a Context, portNumber: u8) -> Self
	{
		let mut attributes = unsafe { uninitialized() };
		panic_on_error!(ibv_query_port, context.0, portNumber, &mut attributes);
		
		Self
		{
			context: context,
			portNumber: portNumber,
			attributes: attributes
		}
	}
	
	/// See <https://linux.die.net/man/3/ibv_query_port> for explanations of fields of ibv_port_attr
	#[inline(always)]
	pub fn attributes(&self) -> &ibv_port_attr
	{
		&self.attributes
	}
	
	#[inline(always)]
	pub fn protectionKey(&self, index: u16) -> u16
	{
		debug_assert!(index < self.attributes.pkey_tbl_len, "index '{}' is too big for table size of '{}'", index, self.attributes.pkey_tbl_len);
		
		let mut protectionKey = unsafe { uninitialized() };
		panic_on_error!(ibv_query_pkey, self.context.0, self.portNumber, index as i32, &mut protectionKey);
		protectionKey
	}
	
	#[inline(always)]
	pub fn gid(&self, index: i32) -> ibv_gid
	{
		debug_assert!(index < self.attributes.gid_tbl_len, "index '{}' is too big for table size of '{}'", index, self.attributes.gid_tbl_len);
		
		let mut gid = unsafe { uninitialized() };
		panic_on_error!(ibv_query_gid, self.context.0, self.portNumber, index, &mut gid);
		gid
	}
	
	/*
		pub fn ibv_resolve_eth_l2_from_gid(context: *mut ibv_context, attr: *mut ibv_ah_attr, eth_mac: *mut u8, vid: *mut u16) -> c_int;
		
		pub fn ibv_init_ah_from_wc(context: *mut ibv_context, port_num: u8, wc: *mut ibv_wc, grh: *mut ibv_grh, ah_attr: *mut ibv_ah_attr) -> c_int;
		
		pub fn ibv_query_gid(context: *mut ibv_context, port_num: u8, index: c_int, gid: *mut ibv_gid) -> c_int;
		
		pub fn ibv_query_pkey(context: *mut ibv_context, port_num: u8, index: c_int, pkey: *mut u16) -> c_int;
	*/
}

