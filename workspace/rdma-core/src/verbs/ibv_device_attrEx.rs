// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ibv_device_attrEx
{
	#[inline(always)]
	fn attributes(&self) -> &ibv_device_attr;
	
	#[inline(always)]
	fn deviceHasCapability(&self, capability: ibv_device_cap_flags) -> bool;
	
	#[inline(always)]
	fn numberOfPhysicalPorts(&self) -> u8;
}

impl ibv_device_attrEx for ibv_device_attr
{
	#[inline(always)]
	fn attributes(&self) -> &ibv_device_attr
	{
		self
	}
	
	#[inline(always)]
	fn deviceHasCapability(&self, capability: ibv_device_cap_flags) -> bool
	{
		let bit = capability as i32;
		self.device_cap_flags & bit == bit
	}
	
	#[inline(always)]
	fn numberOfPhysicalPorts(&self) -> u8
	{
		self.phys_port_cnt
	}
}

impl ibv_device_attrEx for ibv_device_attr_ex
{
	#[inline(always)]
	fn attributes(&self) -> &ibv_device_attr
	{
		&self.orig_attr
	}
	
	#[inline(always)]
	fn deviceHasCapability(&self, capability: ibv_device_cap_flags) -> bool
	{
		self.attributes().deviceHasCapability(capability)
	}
	
	#[inline(always)]
	fn numberOfPhysicalPorts(&self) -> u8
	{
		self.attributes().numberOfPhysicalPorts()
	}
}
