// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn ucp_get(ep: ucp_ep_h, buffer: *mut c_void, length: usize, remote_addr: u64, rkey: ucp_rkey_h) -> ucs_status_t;
	pub fn ucp_get_nbi(ep: ucp_ep_h, buffer: *mut c_void, length: usize, remote_addr: u64, rkey: ucp_rkey_h) -> ucs_status_t;
	pub fn ucp_get_version(major_version: *mut c_uint, minor_version: *mut c_uint, release_number: *mut c_uint);
	pub fn ucp_get_version_string() -> *const c_char;
	pub fn ucp_put(ep: ucp_ep_h, buffer: *const c_void, length: usize, remote_addr: u64, rkey: ucp_rkey_h) -> ucs_status_t;
	pub fn ucp_put_nbi(ep: ucp_ep_h, buffer: *const c_void, length: usize, remote_addr: u64, rkey: ucp_rkey_h) -> ucs_status_t;
}
