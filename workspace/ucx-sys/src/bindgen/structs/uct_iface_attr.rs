// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct uct_iface_attr
{
	pub cap: uct_iface_attr__bindgen_ty_1,
	pub device_addr_len: usize,
	pub iface_addr_len: usize,
	pub ep_addr_len: usize,
	pub overhead: f64,
	pub bandwidth: f64,
	pub latency: uct_linear_growth_t,
	pub priority: u8,
}
