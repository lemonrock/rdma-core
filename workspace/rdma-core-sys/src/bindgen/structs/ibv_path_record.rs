// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub struct ibv_path_record
{
	pub service_id: __be64,
	pub dgid: ibv_gid,
	pub sgid: ibv_gid,
	pub dlid: __be16,
	pub slid: __be16,
	pub flowlabel_hoplimit: __be32,
	pub tclass: u8,
	pub reversible_numpath: u8,
	pub pkey: __be16,
	pub qosclass_sl: __be16,
	pub mtu: u8,
	pub rate: u8,
	pub packetlifetime: u8,
	pub preference: u8,
	pub reserved: [u8; 6usize],
}

impl Clone for ibv_path_record
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ibv_path_record
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
