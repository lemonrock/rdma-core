// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ucs_cpu_set_tEx
{
	#[inline(always)]
	fn createForCurrentCpuIndex() -> Self;
	
	#[inline(always)]
	fn setCpu(&mut self, cpuZeroBasedIndex: u16);
	
	#[inline(always)]
	fn unsetCpu(&mut self, cpuZeroBasedIndex: u16);
	
	#[inline(always)]
	fn isCpuSet(&mut self, cpuZeroBasedIndex: u16) -> bool;
	
	#[inline(always)]
	fn indexAndOffset(&self, cpuZeroBasedIndex: u16) -> (usize, usize);
	
	#[inline(always)]
	fn currentCpuIndex() -> u16
	{
		let result = unsafe { sched_getcpu() };
		debug_assert!(result >= -1, "result was less than -1, '{}'", result);
		if unlikely(result == -1)
		{
			panic!("sched_getcpu returned error '{}'", errno());
		}
		else
		{
			result as u16
		}
	}
}

impl ucs_cpu_set_tEx for ucs_cpu_set_t
{
	#[inline(always)]
	fn createForCurrentCpuIndex() -> Self
	{
		let mut set = Self
		{
			ucs_bits: unsafe { zeroed() },
		};
		
		set.setCpu(Self::currentCpuIndex());
		
		set
	}
	
	#[inline(always)]
	fn setCpu(&mut self, cpuZeroBasedIndex: u16)
	{
		let (index, offset) = self.indexAndOffset(cpuZeroBasedIndex);
		self.ucs_bits[index] |= 1 << offset;
	}
	
	#[inline(always)]
	fn unsetCpu(&mut self, cpuZeroBasedIndex: u16)
	{
		let (index, offset) = self.indexAndOffset(cpuZeroBasedIndex);
		self.ucs_bits[index] &= !(1 << offset);
	}
	
	#[inline(always)]
	fn isCpuSet(&mut self, cpuZeroBasedIndex: u16) -> bool
	{
		let (index, offset) = self.indexAndOffset(cpuZeroBasedIndex);
		0 != (self.ucs_bits[index] & (1 << offset))
	}
	
	#[inline(always)]
	fn indexAndOffset(&self, cpuZeroBasedIndex: u16) -> (usize, usize)
	{
		let sizeInBits = 8 * size_of_val(&self.ucs_bits[0]);
		let cpu = cpuZeroBasedIndex as usize;
		(cpu / sizeInBits, cpu % sizeInBits)
	}
}
