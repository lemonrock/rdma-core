// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn ibv_attach_mcast(qp: *mut ibv_qp, gid: *const ibv_gid, lid: u16) -> c_int;
	pub fn ibv_destroy_qp(qp: *mut ibv_qp) -> c_int;
	pub fn ibv_detach_mcast(qp: *mut ibv_qp, gid: *const ibv_gid, lid: u16) -> c_int;
	pub fn ibv_modify_qp(qp: *mut ibv_qp, attr: *mut ibv_qp_attr, attr_mask: c_int) -> c_int;
	pub fn ibv_query_qp(qp: *mut ibv_qp, attr: *mut ibv_qp_attr, attr_mask: c_int, init_attr: *mut ibv_qp_init_attr) -> c_int;
}
