// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone)]
pub struct MultiCastGroupIdentifier
{
	localIdentifier: LocalIdentifier,
	multicastGroupId: ibv_gid,
}

impl PartialEq for MultiCastGroupIdentifier
{
	fn eq(&self, other: &Self) -> bool
	{
		self.localIdentifier == other.localIdentifier && unsafe { self.multicastGroupId.raw == other.multicastGroupId.raw }
	}
}

impl Eq for MultiCastGroupIdentifier
{
}

impl Hash for MultiCastGroupIdentifier
{
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.localIdentifier.hash(state);
		unsafe { self.multicastGroupId.raw.hash(state) };
	}
}

impl MultiCastGroupIdentifier
{
	#[inline(always)]
	pub fn new(subnetPrefix: __be64, interfaceId: __be64, localIdentifier: LocalIdentifier) -> Self
	{
		Self
		{
			localIdentifier: localIdentifier,
			multicastGroupId: ibv_gid
			{
				global: ibv_gid__bindgen_ty_1
				{
					subnet_prefix: subnetPrefix,
					interface_id: interfaceId,
				}
			},
		}
	}
}

