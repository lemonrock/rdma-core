// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub struct ibv_send_wr
{
	pub wr_id: u64,
	pub next: *mut ibv_send_wr,
	pub sg_list: *mut ibv_sge,
	pub num_sge: c_int,
	pub opcode: ibv_wr_opcode,
	pub send_flags: c_int,
	pub imm_data: __be32,
	pub wr: ibv_send_wr__bindgen_ty_1,
	pub qp_type: ibv_send_wr__bindgen_ty_2,
	pub __bindgen_anon_1: ibv_send_wr__bindgen_ty_3,
}
