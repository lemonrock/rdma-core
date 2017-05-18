// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait EventData
{
	#[doc(hidden)]
	#[inline(always)]
	fn pointer(&self) -> *mut rdma_cm_event;
	
	/// .len() may be greater than the amount of data sent by the remote side, but all excess bytes are g'teed to be zero
	/// .len() will be between 1 and 255 inclusive, but "The length of the private data provided by the user is limited to 196 bytes for RDMA_PS_TCP, or 136 bytes for RDMA_PS_UDP." (rdma_accept man page)
	/// Note that private data of length zero is not transmitted on rejection; it is impossible to distinguish no private data from null
	#[inline(always)]
	fn privateData<'a>(&'a self) -> Option<&'a [u8]>;
	
	#[inline(always)]
	fn remoteQueuePairNumber(&self) -> QueuePairNumber;
}

#[inline(always)]
fn privateDataConn<'a>(data: rdma_conn_param) -> Option<&'a [u8]>
{
	let pointer = data.private_data;
	
	if unlikely(pointer.is_null())
	{
		None
	}
	else
	{
		Some(unsafe { from_raw_parts(pointer as *const u8, data.private_data_len as usize) })
	}
}

#[inline(always)]
fn remoteQueuePairNumberConn(data: rdma_conn_param) -> QueuePairNumber
{
	data.qp_num
}

#[inline(always)]
fn privateDataUd<'a>(data: rdma_ud_param) -> Option<&'a [u8]>
{
	let pointer = data.private_data;
	
	if unlikely(pointer.is_null())
	{
		None
	}
	else
	{
		Some(unsafe { from_raw_parts(pointer as *const u8, data.private_data_len as usize) })
	}
}

#[inline(always)]
fn remoteQueuePairNumberUd(data: rdma_ud_param) -> QueuePairNumber
{
	data.qp_num
}
