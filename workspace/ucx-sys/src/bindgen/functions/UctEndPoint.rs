// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn uct_ep_check(ep: uct_ep_h, flags: c_uint, comp: *mut uct_completion_t) -> ucs_status_t;
	pub fn uct_ep_connect_to_ep(ep: uct_ep_h, dev_addr: *const uct_device_addr_t, ep_addr: *const uct_ep_addr_t) -> ucs_status_t;
	pub fn uct_ep_create(iface: uct_iface_h, ep_p: *mut uct_ep_h) -> ucs_status_t;
	pub fn uct_ep_create_connected(iface: uct_iface_h, dev_addr: *const uct_device_addr_t, iface_addr: *const uct_iface_addr_t, ep_p: *mut uct_ep_h) -> ucs_status_t;
	pub fn uct_ep_destroy(ep: uct_ep_h);
	pub fn uct_ep_get_address(ep: uct_ep_h, addr: *mut uct_ep_addr_t) -> ucs_status_t;
}
