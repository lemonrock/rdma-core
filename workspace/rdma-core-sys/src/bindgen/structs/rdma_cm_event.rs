// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct rdma_cm_event
{
	pub id: *mut rdma_cm_id,
	pub listen_id: *mut rdma_cm_id,
	pub event: rdma_cm_event_type,
	pub status: c_int,
	pub param: rdma_cm_event__bindgen_ty_1,
}

impl Default for rdma_cm_event
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rdma_cm_event
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rdma_cm_event {{ id: {:?}, listen_id: {:?}, event: {:?}, param: {:?} }}", self.id, self.listen_id, self.event, self.param)
	}
}
