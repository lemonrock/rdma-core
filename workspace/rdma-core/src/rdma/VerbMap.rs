// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


//noinspection SpellCheckingInspection
/// A context can also be known as 'verbs' and as a 'device' (in RDMA-speak, not ibverbs-speak)
pub struct VerbMap<'a, VME: VerbMapEntry<'a>>
where VME: 'a
{
	map: HashMap<*mut ibv_context, VME>,
	phantomData: PhantomData<&'a u32>,
}

//noinspection SpellCheckingInspection
impl<'a, VME: VerbMapEntry<'a>> VerbMap<'a, VME>
{
	#[inline(always)]
	pub fn new(constructionParameters: &'a VME::ConstructionParameters) -> Self
	{
		let mut numberOfVerbs = unsafe { uninitialized() };
		let listOfVerbs = panic_on_null!(rdma_get_devices, &mut numberOfVerbs);
		
		debug_assert!(numberOfVerbs >= 0, "numberOfVerbs '{}' was negative", numberOfVerbs);
		
		let mut result = Self
		{
			map: HashMap::with_capacity(numberOfVerbs as usize),
			phantomData: PhantomData,
		};
		
		unsafe { rdma_free_devices(listOfVerbs) };
		
		let mut verbs = unsafe { *listOfVerbs };
		let mut counter = 0;
		while likely(!verbs.is_null())
		{
			result.map.insert(verbs, VME::create(constructionParameters, verbs));
			
			verbs = unsafe { verbs.offset(1) };
			counter = counter + 1;
		}
		debug_assert!(counter == numberOfVerbs, "NULL-terminated list had '{}' elements but was supposed to have numberOfVerbs '{}' elements", counter, numberOfVerbs);
		
		result
	}
	
	#[inline(always)]
	pub fn get(&self, key: *mut ibv_context) -> Option<&VME>
	{
		self.map.get(&key)
	}
	
	#[inline(always)]
	pub fn destroy(&mut self)
	{
		for (_, entry) in self.map.drain()
		{
			drop(entry);
		}
	}
}
