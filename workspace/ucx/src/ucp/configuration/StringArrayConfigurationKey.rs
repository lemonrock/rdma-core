// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StringArrayConfigurationKey
{
	/// Valid values:-
	/// * `all`
	/// * `DEVICE_NAME` where `DEVICE_NAME` is a device name
	/// Default is `all`
	/// Order is ignored
	NetworkDeviceNames,
	
	/// Valid values:-
	/// * `all`
	/// * `DEVICE_NAME` where `DEVICE_NAME` is a device name
	/// Default is `all`
	/// Order is ignored
	SharedMemoryDeviceNames,
	
	/// Valid values:-
	/// * `all`
	/// * `DEVICE_NAME` where `DEVICE_NAME` is a device name
	/// Default is `all`
	/// Order is ignored
	AcceleratedDeviceNames,
	
	/// Valid values:-
	/// * `all`
	/// * `DEVICE_NAME` where `DEVICE_NAME` is a device name
	/// Default is `all`
	/// Order is ignored
	SelfDeviceNames,
	
	/// Valid values:-
	/// * `all`
	/// * `\\DEVICE_NAME` where `DEVICE_NAME` is a device name (note leading double slash)
	/// * `shm` (or `sm`) Shared memory, or
	/// 	* `mm` Shared memory memory-mapper only (SysV, POSIX)
	/// * `ugni` Cray Aries
	/// * `ib` All InfiniBand Verbs transports, or
	///     * `rc` InfiniBand Verbs, reliably connected and unreliable datagram
	///     * `rc_x` InfiniBand Verbs, reliably connected and unreliable datagram, accelerated (requires Mellanox libibverbs and Mellanox ConnectX-4+ hardware)
	///     * `ud_x` InfiniBand Verbs, unreliable datagram, accelerated (requires Mellanox libibverbs and Mellanox ConnectX-4+ hardware)
	///     * `dc_x` InfiniBand Verbs, dynamically connected, accelerated (requires Mellanox libibverbs and Mellanox ConnectX-4+ hardware)
	/// Default is `all`
	/// Order is ignored
	TransportLayersToUseIfAvailable,
	
	/// Valid values:-
	/// * `huge` Huge pages from hugetlbfs
	/// * `thp` Transparent huge pages
	/// * `mmap` Use mmap
	/// * ?`libc` Use libc
	/// * `heap` Use libc heap
	/// * `md:MEMORY_DOMAIN`, Use a memory domain, where `MEMORY_DOMAIN` is one of:-
	///     * `sysv` Use SysV shared memory
	///     * `posix` Use POSIX shared memory
	///     * `\*` Use all memory domains
	///     * ? Anything else ?
	/// Default is `md:sysv,md:posix,huge,thp,md:*,mmap,heap`
	/// Order is priority of memory allocator; left to right is highest priority to least
	MemoryAllocatorPriority,
}

impl ConfigurationKey for StringArrayConfigurationKey
{
	#[inline(always)]
	fn key(&self) -> *const c_char
	{
		const_cstr!
		{
			NET_DEVICES = "NET_DEVICES";
			SHM_DEVICES = "SHM_DEVICES";
			ACC_DEVICES = "ACC_DEVICES";
			SELF_DEVICES = "SELF_DEVICES";
			TLS = "TLS";
			ALLOC_PRIO = "ALLOC_PRIO";
		}
		
		let constant = match *self
		{
			StringArrayConfigurationKey::NetworkDeviceNames => NET_DEVICES,
			StringArrayConfigurationKey::SharedMemoryDeviceNames => SHM_DEVICES,
			StringArrayConfigurationKey::AcceleratedDeviceNames => ACC_DEVICES,
			StringArrayConfigurationKey::SelfDeviceNames => SELF_DEVICES,
			StringArrayConfigurationKey::TransportLayersToUseIfAvailable => TLS,
			StringArrayConfigurationKey::MemoryAllocatorPriority => ALLOC_PRIO,
		};
		constant.as_ptr()
	}
	
	#[inline(always)]
	fn defaultValue(&self) -> CString
	{
		let value = match *self
		{
			StringArrayConfigurationKey::NetworkDeviceNames => "all",
			StringArrayConfigurationKey::SharedMemoryDeviceNames => "all",
			StringArrayConfigurationKey::AcceleratedDeviceNames => "all",
			StringArrayConfigurationKey::SelfDeviceNames => "all",
			StringArrayConfigurationKey::TransportLayersToUseIfAvailable => "all",
			StringArrayConfigurationKey::MemoryAllocatorPriority => "md:sysv,md:posix,huge,thp,md:*,mmap,heap",
		};
		CString::new(value).unwrap()
	}
}

impl StringArrayConfigurationKey
{
	#[inline(always)]
	pub fn validateValue(&self, value: &str) -> bool
	{
		if value.is_empty()
		{
			return false;
		}
		
		match *self
		{
			StringArrayConfigurationKey::NetworkDeviceNames => true,
			StringArrayConfigurationKey::SharedMemoryDeviceNames => true,
			StringArrayConfigurationKey::AcceleratedDeviceNames => true,
			StringArrayConfigurationKey::SelfDeviceNames => true,
			StringArrayConfigurationKey::TransportLayersToUseIfAvailable =>
			{
				match value
				{
					"all" => true,
					"sm" => true,
					"shm" => true,
					"mm" => true,
					"ugni" => true,
					"ib" => true,
					"rc" => true,
					"rc_x" => true,
					"ud_x" => true,
					"dc_x" => true,
					_ => value.len() > 2 && value.starts_with("\\\\")
				}
			},
			StringArrayConfigurationKey::MemoryAllocatorPriority =>
			{
				match value
				{
					"huge" => true,
					"thp" => true,
					"mmap" => true,
					"libc" => true,
					"heap" => true,
					"md:sysv" => true,
					"md:posix" => true,
					"md:*" => true,
					_ => false,
				}
			}
		}
	}
}
