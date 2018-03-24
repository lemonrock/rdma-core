// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_send_wr
{
	pub wr_id: u64,
	pub next: *mut ibv_send_wr,
	pub sg_list: *mut ibv_sge,
	pub num_sge: c_int,
	pub opcode: ibv_wr_opcode,
	pub send_flags: c_uint,
	pub __bindgen_anon_1: ibv_send_wr__bindgen_ty_1,
	pub wr: ibv_send_wr__bindgen_ty_2,
	pub qp_type: ibv_send_wr__bindgen_ty_3,
	pub __bindgen_anon_2: ibv_send_wr__bindgen_ty_4,
}

impl Default for ibv_send_wr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_send_wr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_send_wr {{ next: {:?}, sg_list: {:?}, opcode: {:?}, __bindgen_anon_1: {:?}, wr: {:?}, qp_type: {:?}, __bindgen_anon_2: {:?} }}", self.next, self.sg_list, self.opcode, self.__bindgen_anon_1, self.wr, self.qp_type, self.__bindgen_anon_2)
	}
}
