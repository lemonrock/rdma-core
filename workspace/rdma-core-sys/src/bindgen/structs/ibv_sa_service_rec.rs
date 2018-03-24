// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_sa_service_rec
{
	pub id: u64,
	pub gid: ibv_gid,
	pub pkey: u16,
	pub lease: u32,
	pub key: [u8; 16usize],
	pub name: [u8; 64usize],
	pub data8: [u8; 16usize],
	pub data16: [u16; 8usize],
	pub data32: [u32; 4usize],
	pub data64: [u64; 2usize],
}

impl Default for ibv_sa_service_rec
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_sa_service_rec
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"ibv_sa_service_rec {{ gid: {:?}, key: {:?}, name: [{}], data8: {:?}, data16: {:?}, data32: {:?}, data64: {:?} }}",
			self.gid,
			self.key,
			self.name
				.iter()
				.enumerate()
				.map(|(i, v)| format!(
					"{}{:?}",
					if i > 0
					{
						", "
					}
					else
					{
						""
					},
					v
				))
				.collect::<String>(),
			self.data8,
			self.data16,
			self.data32,
			self.data64
		)
	}
}
