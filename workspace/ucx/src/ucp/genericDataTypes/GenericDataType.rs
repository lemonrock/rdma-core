// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct GenericDataType<T: Sized, S: Serialiser<T>, D: Deserialiser<T>>
{
	// NOTE: In the current ucp code, dataType is actually a pointer which references ops and context with UCP_DATATYPE_GENERIC or'd onto the bottom (since this is 3 bits, and pointers are 64-bit aligned, this works but seems horrible)
	context: *mut GenericDataTypeCreator<T, S, D>,
	dataType: ucp_datatype_t,
}

impl<T: Sized, S: Serialiser<T>, D: Deserialiser<T>> Drop for GenericDataType<T, S, D>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_dt_destroy(self.dataType) };
		
		unsafe { Box::from_raw(self.context) };
	}
}

impl<T: Sized, S: Serialiser<T>, D: Deserialiser<T>> GenericDataType<T, S, D>
{
	#[inline(always)]
	pub fn genericDataType(&self) -> ucp_datatype_t
	{
		self.dataType
	}
}
