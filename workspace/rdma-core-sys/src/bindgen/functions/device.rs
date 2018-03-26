// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn ibv_free_device_list(list: *mut *mut ibv_device);
	pub fn ibv_get_device_guid(device: *mut ibv_device) -> __be64;
	pub fn ibv_get_device_list(num_devices: *mut c_int) -> *mut *mut ibv_device;
	pub fn ibv_get_device_name(device: *mut ibv_device) -> *const c_char;
	pub fn ibv_open_device(device: *mut ibv_device) -> *mut ibv_context;
}
