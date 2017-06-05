// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn ucp_mem_advise(context: ucp_context_h, memh: ucp_mem_h, params: *mut ucp_mem_advise_params_t) -> ucs_status_t;
	pub fn ucp_mem_map(context: ucp_context_h, params: *const ucp_mem_map_params_t, memh_p: *mut ucp_mem_h) -> ucs_status_t;
	pub fn ucp_mem_query(memh: ucp_mem_h, attr: *mut ucp_mem_attr_t) -> ucs_status_t;
	pub fn ucp_mem_unmap(context: ucp_context_h, memh: ucp_mem_h) -> ucs_status_t;
}
