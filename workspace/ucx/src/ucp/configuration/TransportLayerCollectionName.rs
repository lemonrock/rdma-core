// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TransportLayerCollectionName
{
	/// Everything
	all,
	
	/// All shared memory transports; sm and shm are identical for some reason
	/// ie mm, knem, sysv, posix, cma and xpmem
	sm,
	
	/// All shared memory transports; sm and shm are identical for some reason
	/// ie mm, knem, sysv, posix, cma and xpmem
	shm,
	
	/// Selects sysv and posix shared memory transports
	mm,
	
	/// A specific shared memory transport
	knem,
	
	/// A specific shared memory transport
	sysv,
	
	/// A specific shared memory transport
	posix,
	
	/// A specific shared memory transport
	cma,
	
	/// A specific shared memory transport
	xpmem,
	
	/// Selects all Cray Aries transports
	/// ie ugni_smsg, ugni_udt and ugni_rdma
	ugni,
	
	/// Crag Aries ?Short Message?
	ugni_smsg,
	
	/// Cray Aries ?unreliable datagram?
	ugni_udt,
	
	/// Cray Aries RDMA
	ugni_rdma,
	
	/// Selects most InfiniBand / IB verbs / RDMA transports
	/// ie rc, ud, rc_mlx5 and ud_mlx5
	ib,
	
	/// Selects itself (!) and ud
	rc,
	
	/// Unreliable datagram, unaccelerated verbs library
	ud,
	
	/// Reliable connection, accelerated verbs library; selects additional transports
	/// ie rc_mlx5 and ud_mlx5
	rc_x,
	
	/// A specific accelerated transport for Mellanox ConnectX hardware
	rc_mlx5,
	
	/// Unreliable datagram, accelerated verbs library; selects additional transports
	/// ie ud_mlx5
	ud_x,
	
	/// Unreliable datagram, accelerated verbs library, for Mellanox ConnectX hardware
	ud_mlx5,
	
	/// Dynamic connection, accelerated verbs library; selects additional transports
	/// ie dc_mlx5
	dc_x,
	
	/// Dynamic connection, accelerated verbs library, for Mellanox ConnectX hardware
	dc_mlx5,
	
	Device(DeviceName),
}

impl ToString for TransportLayerCollectionName
{
	#[inline(always)]
	fn to_string(&self) -> String
	{
		use self::TransportLayerCollectionName::*;
		
		match *self
		{
			all => "all".to_owned(),
			sm => "sm".to_owned(),
			shm => "shm".to_owned(),
			mm => "mm".to_owned(),
			knem => "knem".to_owned(),
			sysv => "sysv".to_owned(),
			posix => "posix".to_owned(),
			cma => "cma".to_owned(),
			xpmem => "xpmem".to_owned(),
			ugni => "ugni".to_owned(),
			ugni_smsg => "ugni_smsg".to_owned(),
			ugni_udt => "ugni_udt".to_owned(),
			ugni_rdma => "ugni_rdma".to_owned(),
			ib => "ib".to_owned(),
			rc => "rc".to_owned(),
			ud => "ud".to_owned(),
			rc_x => "rc_x".to_owned(),
			rc_mlx5 => "rc_mlx5".to_owned(),
			ud_x => "ud_x".to_owned(),
			ud_mlx5 => "ud_mlx5".to_owned(),
			dc_x => "dc_x".to_owned(),
			dc_mlx5 => "dc_mlx5".to_owned(),
			Device(ref value) => match *value
			{
				DeviceName::all => "all".to_owned(),
				DeviceName::Specific(ref value) =>
				{
					let mut s = "\\\\".to_owned();
					s.push_str(value);
					s
				}
			},
		}.to_owned()
	}
}

impl TransportLayerCollectionName
{
	#[inline(always)]
	pub fn fromString(value: &str) -> Option<TransportLayerCollectionName>
	{
		use self::TransportLayerCollectionName::*;
		
		if value.is_empty()
		{
			None
		}
		else
		{
			match value
			{
				"all" => Some(all),
				"sm" => Some(sm),
				"shm" => Some(shm),
				"mm" => Some(mm),
				"knem" => Some(knem),
				"sysv" => Some(sysv),
				"posix" => Some(posix),
				"cma" => Some(cma),
				"xpmem" => Some(xpmem),
				"ugni" => Some(ugni),
				"ugni_smsg" => Some(ugni_smsg),
				"ugni_udt" => Some(ugni_udt),
				"ugni_rdma" => Some(ugni_rdma),
				"ib" => Some(ib),
				"rc" => Some(rc),
				"ud" => Some(ud),
				"rc_x" => Some(rc_x),
				"rc_mlx5" => Some(rc_mlx5),
				"ud_x" => Some(ud_x),
				"ud_mlx5" => Some(ud_mlx5),
				"dc_x" => Some(dc_x),
				"dc_mlx5" => Some(dc_mlx5),
				_ =>
				{
					if value.len() > 2 && value.starts_with("\\\\")
					{
						Some(Device(DeviceName::Specific(value[2..].to_string())))
					}
					else
					{
						None
					}
				},
			}
		}
	}
}
