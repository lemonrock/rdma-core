// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct uct_iface_ops
{
	pub iface_close: Option<unsafe extern "C" fn(iface: uct_iface_h)>,
	pub iface_query: Option<unsafe extern "C" fn(iface: uct_iface_h, iface_attr: *mut uct_iface_attr_t) -> ucs_status_t>,
	pub iface_flush: Option<unsafe extern "C" fn(iface: uct_iface_h, flags: c_uint, comp: *mut uct_completion_t) -> ucs_status_t>,
	pub iface_fence: Option<unsafe extern "C" fn(iface: uct_iface_h, flags: c_uint) -> ucs_status_t>,
	pub iface_wakeup_open: Option<unsafe extern "C" fn(iface: uct_iface_h, events: c_uint, wakeup: uct_wakeup_h) -> ucs_status_t>,
	pub iface_wakeup_get_fd: Option<unsafe extern "C" fn(wakeup: uct_wakeup_h, fd_p: *mut c_int) -> ucs_status_t>,
	pub iface_wakeup_arm: Option<unsafe extern "C" fn(wakeup: uct_wakeup_h) -> ucs_status_t>,
	pub iface_wakeup_wait: Option<unsafe extern "C" fn(wakeup: uct_wakeup_h) -> ucs_status_t>,
	pub iface_wakeup_signal: Option<unsafe extern "C" fn(wakeup: uct_wakeup_h) -> ucs_status_t>,
	pub iface_wakeup_close: Option<unsafe extern "C" fn(wakeup: uct_wakeup_h)>,
	pub iface_tag_recv_zcopy: Option<unsafe extern "C" fn(iface: uct_iface_h, tag: uct_tag_t, tag_mask: uct_tag_t, iov: *const uct_iov_t, iovcnt: usize, ctx: *mut uct_tag_context_t) -> ucs_status_t>,
	pub iface_tag_recv_cancel: Option<unsafe extern "C" fn(iface: uct_iface_h, ctx: *mut uct_tag_context_t, force: c_int) -> ucs_status_t>,
	pub ep_create: Option<unsafe extern "C" fn(iface: uct_iface_h, ep_p: *mut uct_ep_h) -> ucs_status_t>,
	pub ep_create_connected: Option<unsafe extern "C" fn(iface: uct_iface_h, dev_addr: *const uct_device_addr_t, iface_addr: *const uct_iface_addr_t, ep_p: *mut uct_ep_h) -> ucs_status_t>,
	pub ep_destroy: Option<unsafe extern "C" fn(ep: uct_ep_h)>,
	pub ep_get_address: Option<unsafe extern "C" fn(ep: uct_ep_h, addr: *mut uct_ep_addr_t) -> ucs_status_t>,
	pub ep_connect_to_ep: Option<unsafe extern "C" fn(ep: uct_ep_h, dev_addr: *const uct_device_addr_t, ep_addr: *const uct_ep_addr_t) -> ucs_status_t>,
	pub iface_get_device_address: Option<unsafe extern "C" fn(iface: uct_iface_h, addr: *mut uct_device_addr_t) -> ucs_status_t>,
	pub iface_get_address: Option<unsafe extern "C" fn(iface: uct_iface_h, addr: *mut uct_iface_addr_t) -> ucs_status_t>,
	pub iface_is_reachable: Option<unsafe extern "C" fn(iface: uct_iface_h, dev_addr: *const uct_device_addr_t, iface_addr: *const uct_iface_addr_t) -> c_int>,
	pub ep_put_short: Option<unsafe extern "C" fn(ep: uct_ep_h, buffer: *const c_void, length: c_uint, remote_addr: u64, rkey: uct_rkey_t) -> ucs_status_t>,
	pub ep_put_bcopy: Option<unsafe extern "C" fn(ep: uct_ep_h, pack_cb: uct_pack_callback_t, arg: *mut c_void, remote_addr: u64, rkey: uct_rkey_t) -> isize>,
	pub ep_put_zcopy: Option<unsafe extern "C" fn(ep: uct_ep_h, iov: *const uct_iov_t, iovcnt: usize, remote_addr: u64, rkey: uct_rkey_t, comp: *mut uct_completion_t) -> ucs_status_t>,
	pub ep_get_bcopy: Option<unsafe extern "C" fn(ep: uct_ep_h, unpack_cb: uct_unpack_callback_t, arg: *mut c_void, length: usize, remote_addr: u64, rkey: uct_rkey_t, comp: *mut uct_completion_t) -> ucs_status_t>,
	pub ep_get_zcopy: Option<unsafe extern "C" fn(ep: uct_ep_h, iov: *const uct_iov_t, iovcnt: usize, remote_addr: u64, rkey: uct_rkey_t, comp: *mut uct_completion_t) -> ucs_status_t>,
	pub ep_am_short: Option<unsafe extern "C" fn(ep: uct_ep_h, id: u8, header: u64, payload: *const c_void, length: c_uint) -> ucs_status_t>,
	pub ep_am_bcopy: Option<unsafe extern "C" fn(ep: uct_ep_h, id: u8, pack_cb: uct_pack_callback_t, arg: *mut c_void) -> isize>,
	pub ep_am_zcopy: Option<unsafe extern "C" fn(ep: uct_ep_h, id: u8, header: *const c_void, header_length: c_uint, iov: *const uct_iov_t, iovcnt: usize, comp: *mut uct_completion_t) -> ucs_status_t>,
	pub ep_atomic_add64: Option<unsafe extern "C" fn(ep: uct_ep_h, add: u64, remote_addr: u64, rkey: uct_rkey_t) -> ucs_status_t>,
	pub ep_atomic_fadd64: Option<unsafe extern "C" fn(ep: uct_ep_h, add: u64, remote_addr: u64, rkey: uct_rkey_t, result: *mut u64, comp: *mut uct_completion_t) -> ucs_status_t>,
	pub ep_atomic_swap64: Option<unsafe extern "C" fn(ep: uct_ep_h, swap: u64, remote_addr: u64, rkey: uct_rkey_t, result: *mut u64, comp: *mut uct_completion_t) -> ucs_status_t>,
	pub ep_atomic_cswap64: Option<unsafe extern "C" fn(ep: uct_ep_h, compare: u64, swap: u64, remote_addr: u64, rkey: uct_rkey_t, result: *mut u64, comp: *mut uct_completion_t) -> ucs_status_t>,
	pub ep_atomic_add32: Option<unsafe extern "C" fn(ep: uct_ep_h, add: u32, remote_addr: u64, rkey: uct_rkey_t) -> ucs_status_t>,
	pub ep_atomic_fadd32: Option<unsafe extern "C" fn(ep: uct_ep_h, add: u32, remote_addr: u64, rkey: uct_rkey_t, result: *mut u32, comp: *mut uct_completion_t) -> ucs_status_t>,
	pub ep_atomic_swap32: Option<unsafe extern "C" fn(ep: uct_ep_h, swap: u32, remote_addr: u64, rkey: uct_rkey_t, result: *mut u32, comp: *mut uct_completion_t) -> ucs_status_t>,
	pub ep_atomic_cswap32: Option<unsafe extern "C" fn(ep: uct_ep_h, compare: u32, swap: u32, remote_addr: u64, rkey: uct_rkey_t, result: *mut u32, comp: *mut uct_completion_t) -> ucs_status_t>,
	pub ep_pending_add: Option<unsafe extern "C" fn(ep: uct_ep_h, n: *mut uct_pending_req_t) -> ucs_status_t>,
	pub ep_pending_purge: Option<unsafe extern "C" fn(ep: uct_ep_h, cb: uct_pending_purge_callback_t, arg: *mut c_void)>,
	pub ep_flush: Option<unsafe extern "C" fn(ep: uct_ep_h, flags: c_uint, comp: *mut uct_completion_t) -> ucs_status_t>,
	pub ep_fence: Option<unsafe extern "C" fn(ep: uct_ep_h, flags: c_uint) -> ucs_status_t>,
	pub ep_check: Option<unsafe extern "C" fn(ep: uct_ep_h, flags: c_uint, comp: *mut uct_completion_t) -> ucs_status_t>,
	pub ep_tag_eager_short: Option<unsafe extern "C" fn(ep: uct_ep_h, tag: uct_tag_t, data: *const c_void, length: usize) -> ucs_status_t>,
	pub ep_tag_eager_bcopy: Option<unsafe extern "C" fn(ep: uct_ep_h, tag: uct_tag_t, imm: u64, pack_cb: uct_pack_callback_t, arg: *mut c_void) -> isize>,
	pub ep_tag_eager_zcopy: Option<unsafe extern "C" fn(ep: uct_ep_h, tag: uct_tag_t, imm: u64, iov: *const uct_iov_t, iovcnt: usize, comp: *mut uct_completion_t) -> ucs_status_t>,
	pub ep_tag_rndv_zcopy: Option<unsafe extern "C" fn(ep: uct_ep_h, tag: uct_tag_t, header: *const c_void, header_length: c_uint, iov: *const uct_iov_t, iovcnt: usize, comp: *mut uct_completion_t) -> ucs_status_ptr_t>,
	pub ep_tag_rndv_cancel: Option<unsafe extern "C" fn(ep: uct_ep_h, op: *mut c_void) -> ucs_status_t>,
	pub ep_tag_rndv_request: Option<unsafe extern "C" fn(ep: uct_ep_h, tag: uct_tag_t, header: *const c_void, header_length: c_uint) -> ucs_status_t>,
}

impl Clone for uct_iface_ops
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for uct_iface_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
