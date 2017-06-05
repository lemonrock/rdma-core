// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn ucs_callbackq_add(cbq: *mut ucs_callbackq_t, cb: ucs_callback_t, arg: *mut c_void);
	pub fn ucs_callbackq_add_safe(cbq: *mut ucs_callbackq_t, cb: ucs_callback_t, arg: *mut c_void) -> ucs_status_t;
	pub fn ucs_callbackq_add_slow_path(cbq: *mut ucs_callbackq_t, elem: *mut ucs_callbackq_slow_elem_t);
	pub fn ucs_callbackq_cleanup(cbq: *mut ucs_callbackq_t);
	pub fn ucs_callbackq_init(cbq: *mut ucs_callbackq_t, size: usize, async: *mut ucs_async_context_t) -> ucs_status_t;
	pub fn ucs_callbackq_purge_slow_path(cbq: *mut ucs_callbackq_t, cb: ucs_callback_slow_t, list: *mut ucs_list_link_t);
	pub fn ucs_callbackq_remove(cbq: *mut ucs_callbackq_t, cb: ucs_callback_t, arg: *mut c_void) -> ucs_status_t;
	pub fn ucs_callbackq_remove_all(cbq: *mut ucs_callbackq_t, cb: ucs_callback_t, arg: *mut c_void);
	pub fn ucs_callbackq_remove_safe(cbq: *mut ucs_callbackq_t, cb: ucs_callback_t, arg: *mut c_void) -> ucs_status_t;
	pub fn ucs_callbackq_remove_slow_path(cbq: *mut ucs_callbackq_t, elem: *mut ucs_callbackq_slow_elem_t);
}
