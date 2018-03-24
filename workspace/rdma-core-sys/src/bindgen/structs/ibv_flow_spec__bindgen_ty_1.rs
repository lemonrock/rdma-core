// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_flow_spec__bindgen_ty_1
{
	pub hdr: __BindgenUnionField<ibv_flow_spec__bindgen_ty_1__bindgen_ty_1>,
	pub eth: __BindgenUnionField<ibv_flow_spec_eth>,
	pub ipv4: __BindgenUnionField<ibv_flow_spec_ipv4>,
	pub tcp_udp: __BindgenUnionField<ibv_flow_spec_tcp_udp>,
	pub ipv4_ext: __BindgenUnionField<ibv_flow_spec_ipv4_ext>,
	pub ipv6: __BindgenUnionField<ibv_flow_spec_ipv6>,
	pub tunnel: __BindgenUnionField<ibv_flow_spec_tunnel>,
	pub flow_tag: __BindgenUnionField<ibv_flow_spec_action_tag>,
	pub drop: __BindgenUnionField<ibv_flow_spec_action_drop>,
	pub bindgen_union_field: [u32; 22usize],
}

impl Default for ibv_flow_spec__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_flow_spec__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_flow_spec__bindgen_ty_1 {{ union }}")
	}
}
