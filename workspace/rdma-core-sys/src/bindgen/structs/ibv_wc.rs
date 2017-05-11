// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct ibv_wc
{
	pub wr_id: u64,
	pub status: ibv_wc_status,
	pub opcode: ibv_wc_opcode,
	pub vendor_err: u32,
	pub byte_len: u32,
	pub imm_data: u32,
	pub qp_num: u32,
	pub src_qp: u32,
	pub wc_flags: c_int,
	pub pkey_index: u16,
	pub slid: u16,
	pub sl: u8,
	pub dlid_path_bits: u8,
}

impl Clone for ibv_wc
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ibv_wc
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl ::core::ops::BitOr<ibv_access_flags> for ibv_access_flags
{
	type Output = Self;
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		ibv_access_flags(self.0 | other.0)
	}
}
