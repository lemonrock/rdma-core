// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct mxm_send_req
{
	pub base: mxm_req_base_t,
	pub opcode: mxm_req_opcode_t,
	pub flags: c_uint,
	pub op: mxm_send_req__bindgen_ty_1,
	pub reserved: [c_char; 72usize],
}
