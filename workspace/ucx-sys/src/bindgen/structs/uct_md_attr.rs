// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct uct_md_attr
{
	pub cap: uct_md_attr__bindgen_ty_1,
	pub reg_cost: uct_linear_growth_t,
	pub component_name: [c_char; 8usize],
	pub rkey_packed_size: usize,
	pub local_cpus: cpu_set_t,
}
