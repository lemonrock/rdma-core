// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct rdma_addrinfo
{
	pub ai_flags: c_int,
	pub ai_family: c_int,
	pub ai_qp_type: c_int,
	pub ai_port_space: c_int,
	pub ai_src_len: socklen_t,
	pub ai_dst_len: socklen_t,
	pub ai_src_addr: *mut sockaddr,
	pub ai_dst_addr: *mut sockaddr,
	pub ai_src_canonname: *mut c_char,
	pub ai_dst_canonname: *mut c_char,
	pub ai_route_len: usize,
	pub ai_route: *mut c_void,
	pub ai_connect_len: usize,
	pub ai_connect: *mut c_void,
	pub ai_next: *mut rdma_addrinfo,
}

impl Clone for rdma_addrinfo
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for rdma_addrinfo
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
