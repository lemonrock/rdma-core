// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[derive(Copy, Clone)]
pub struct Port<'a>
{
	context: &'a Context,
	portNumber: u8,
	attributes: ibv_port_attr,
}

impl<'a> Port<'a>
{
	#[inline(always)]
	fn new(context: &'a Context, portNumber: u8) -> Self
	{
		let mut attributes = unsafe { uninitialized() };
		panic_on_error!(ibv_query_port, context.0, portNumber, &mut attributes);
		
		Self
		{
			context: context,
			portNumber: portNumber,
			attributes: attributes
		}
	}
	
	/// See <https://linux.die.net/man/3/ibv_query_port> for explanations of fields of ibv_port_attr
	#[inline(always)]
	pub fn attributes(&self) -> &ibv_port_attr
	{
		&self.attributes
	}
	
	#[inline(always)]
	pub fn partitionKey(&self, index: PartitionKeyIndex) -> PartitionKey
	{
		debug_assert!(index < self.attributes.pkey_tbl_len, "index '{}' is too big for table size of '{}'", index, self.attributes.pkey_tbl_len);
		
		let mut protectionKey = unsafe { uninitialized() };
		panic_on_error!(ibv_query_pkey, self.context.0, self.portNumber, index as i32, &mut protectionKey);
		protectionKey
	}
	
	#[inline(always)]
	pub fn gid(&self, index: i32) -> ibv_gid
	{
		debug_assert!(index < self.attributes.gid_tbl_len, "index '{}' is too big for table size of '{}'", index, self.attributes.gid_tbl_len);
		
		let mut gid = unsafe { uninitialized() };
		panic_on_error!(ibv_query_gid, self.context.0, self.portNumber, index, &mut gid);
		gid
	}
}
