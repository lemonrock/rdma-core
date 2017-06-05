// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn uct_iface_close(iface: uct_iface_h);
	pub fn uct_iface_config_read(tl_name: *const c_char, env_prefix: *const c_char, filename: *const c_char, config_p: *mut *mut uct_iface_config_t) -> ucs_status_t;
	pub fn uct_iface_get_address(iface: uct_iface_h, addr: *mut uct_iface_addr_t) -> ucs_status_t;
	pub fn uct_iface_get_device_address(iface: uct_iface_h, addr: *mut uct_device_addr_t) -> ucs_status_t;
	pub fn uct_iface_is_reachable(iface: uct_iface_h, dev_addr: *const uct_device_addr_t, iface_addr: *const uct_iface_addr_t) -> c_int;
	pub fn uct_iface_mem_alloc(iface: uct_iface_h, length: usize, flags: c_uint, name: *const c_char, mem: *mut uct_allocated_memory_t) -> ucs_status_t;
	pub fn uct_iface_mem_free(mem: *const uct_allocated_memory_t);
	pub fn uct_iface_open(md: uct_md_h, worker: uct_worker_h, params: *const uct_iface_params_t, config: *const uct_iface_config_t, iface_p: *mut uct_iface_h) -> ucs_status_t;
	pub fn uct_iface_query(iface: uct_iface_h, iface_attr: *mut uct_iface_attr_t) -> ucs_status_t;
	pub fn uct_iface_set_am_handler(iface: uct_iface_h, id: u8, cb: uct_am_callback_t, arg: *mut c_void, flags: u32) -> ucs_status_t;
	pub fn uct_iface_set_am_tracer(iface: uct_iface_h, tracer: uct_am_tracer_t, arg: *mut c_void) -> ucs_status_t;
}
