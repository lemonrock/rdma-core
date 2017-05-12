// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct WorkRequestError
{
	status: ibv_wc_status,
	vendorErrorCode: u32,
}

impl WorkRequestError
{
	#[inline(always)]
	pub fn is(&self, status: ibv_wc_status) -> bool
	{
		self.status == status
	}
	
	/// See <http://www.rdmamojo.com/2013/02/15/ibv_poll_cq/> for details of these
	#[inline(always)]
	pub fn status(&self) -> ibv_wc_status
	{
		self.status
	}
	
	#[inline(always)]
	pub fn vendorErrorCode(&self) -> u32
	{
		self.vendorErrorCode
	}
}
