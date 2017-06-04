// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub struct mxm_req_base
{
	pub state: mxm_req_state_t,
	pub mq: mxm_mq_h,
	pub conn: mxm_conn_h,
	pub data_type: mxm_req_data_type_t,
	pub data: mxm_req_base__bindgen_ty_1,
	pub context: *mut c_void,
	pub completed_cb: Option<unsafe extern "C" fn(context: *mut c_void)>,
	pub error: mxm_error_t,
	pub reserved: [c_char; 8usize],
}
