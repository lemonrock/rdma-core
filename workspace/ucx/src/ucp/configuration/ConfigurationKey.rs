// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone)]
pub struct ConfigurationKey<T: ConfigurationValueConverter>
{
	name: ConstCStr,
	defaultValue: T
}

impl<T: ConfigurationValueConverter> ConfigurationKey<T>
{
	#[inline(always)]
	pub fn NetworkDeviceNames() -> ConfigurationKey<HashSet<DeviceName>>
	{
		ConfigurationKey
		{
			name: const_cstr!("NET_DEVICES"),
			defaultValue:
			{
				let mut set = HashSet::with_capacity(1);
				set.insert(DeviceName::all);
				set
			},
		}
	}
	
	#[inline(always)]
	pub fn SharedMemoryDeviceNames() -> ConfigurationKey<HashSet<DeviceName>>
	{
		ConfigurationKey
		{
			name: const_cstr!("SHM_DEVICES"),
			defaultValue:
			{
				let mut set = HashSet::with_capacity(1);
				set.insert(DeviceName::all);
				set
			},
		}
	}
	
	#[inline(always)]
	pub fn AcceleratedDeviceNames() -> ConfigurationKey<HashSet<DeviceName>>
	{
		ConfigurationKey
		{
			name: const_cstr!("ACC_DEVICES"),
			defaultValue:
			{
				let mut set = HashSet::with_capacity(1);
				set.insert(DeviceName::all);
				set
			},
		}
	}
	
	#[inline(always)]
	pub fn SelfDeviceNames() -> ConfigurationKey<HashSet<DeviceName>>
	{
		ConfigurationKey
		{
			name: const_cstr!("SELF_DEVICES"),
			defaultValue:
			{
				let mut set = HashSet::with_capacity(1);
				set.insert(DeviceName::all);
				set
			},
		}
	}
	
	#[inline(always)]
	pub fn TransportLayersToUseIfAvailable() -> ConfigurationKey<HashSet<TransportLayerCollectionName>>
	{
		ConfigurationKey
		{
			name: const_cstr!("TLS"),
			defaultValue:
			{
				let mut set = HashSet::with_capacity(1);
				set.insert(TransportLayerCollectionName::all);
				set
			},
		}
	}
	
	#[inline(always)]
	pub fn MemoryAllocatorPriority() -> ConfigurationKey<OrderMap<MemoryAllocatorPriority, ()>>
	{
		ConfigurationKey
		{
			name: const_cstr!("ALLOC_PRIO"),
			defaultValue:
			{
				let mut set = OrderMap::with_capacity(7);
				set.insert(MemoryAllocatorPriority::md(MemoryDomain::sysv), ());
				set.insert(MemoryAllocatorPriority::md(MemoryDomain::posix), ());
				set.insert(MemoryAllocatorPriority::huge, ());
				set.insert(MemoryAllocatorPriority::thp, ());
				set.insert(MemoryAllocatorPriority::md(MemoryDomain::Wildcard), ());
				set.insert(MemoryAllocatorPriority::mmap, ());
				set.insert(MemoryAllocatorPriority::heap, ());
				set
			},
		}
	}
	
	pub const ThresholdForSwitchingFromShortToBufferCopyProtocol: ConfigurationKey<MemoryUnit> = ConfigurationKey
	{
		name: const_cstr!("BCOPY_THRESH"),
		defaultValue: MemoryUnit::UnitLess(0),
	};
	
	pub const ThresholdForSwitchingFromEagerToRendezvousProtocol: ConfigurationKey<MemoryUnit> = ConfigurationKey
	{
		name: const_cstr!("RNDV_THRESH"),
		defaultValue: MemoryUnit::Automatic,
	};
	
	pub const MessageSizeThresholdToStartUsingTheRendezvousProtocolInCaseTheCalculatedThresholdIsZeroOrNegative: ConfigurationKey<MemoryUnit> = ConfigurationKey
	{
		name: const_cstr!("RNDV_THRESH_FALLBACK"),
		defaultValue: MemoryUnit::Infinity,
	};
	
	pub const RendezvousProtocolAndEagerZeroCopyProtocolPercentageDifference: ConfigurationKey<f64> = ConfigurationKey
	{
		name: const_cstr!("RNDV_PERF_DIFF"),
		defaultValue: 1f64,
	};
	
	pub const ThresholdForSwitchingFromBufferCopyProtocolToZeroCopyProtocol: ConfigurationKey<MemoryUnit> = ConfigurationKey
	{
		name: const_cstr!("ZCOPY_THRESH"),
		defaultValue: MemoryUnit::Automatic,
	};
	
	pub const EstimationOfBufferCopyBandwidth: ConfigurationKey<MemoryUnit> = ConfigurationKey
	{
		name: const_cstr!("BCOPY_BW"),
		defaultValue: MemoryUnit::MegaBytes(5800),
	};
	
	/// Default is prefer `device` mode if at least one transport in use supports it, otherwise to fallback to `cpu` mode
	pub const AtomicOperationsSynchronizationMode: ConfigurationKey<AtomicOperationsSynchronizationMode> = ConfigurationKey
	{
		name: const_cstr!("ATOMIC_MODE"),
		defaultValue: AtomicOperationsSynchronizationMode::guess,
	};
	
	pub const MaximumLengthOfWorkerName: ConfigurationKey<u32> = ConfigurationKey
	{
		name: const_cstr!("MAX_WORKER_NAME"),
		defaultValue: 32,
	};
	
	pub const PreferSpinLockOverMutexWhenMultiThreading: ConfigurationKey<bool> = ConfigurationKey
	{
		name: const_cstr!("USE_MT_MUTEX"),
		defaultValue: false,
	};
	
	pub const ThresholdForUsingTagMatchingOffloadCapabilities: ConfigurationKey<MemoryUnit> = ConfigurationKey
	{
		name: const_cstr!("TM_THRESH"),
		defaultValue: MemoryUnit::UnitLess(1024),
	};
	
	#[inline(always)]
	pub fn withValue<'a>(&'a self, value: T) -> ConfigurationSetting<'a, T>
	{
		ConfigurationSetting
		{
			key: self,
			value: value,
		}
	}
	
	#[inline(always)]
	fn name(&self) -> &ConstCStr
	{
		&self.name
	}
}
