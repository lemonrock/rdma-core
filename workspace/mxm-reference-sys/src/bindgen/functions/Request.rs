// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn mxm_req_cancel_recv(req: *mut mxm_recv_req_t) -> mxm_error_t;
	pub fn mxm_req_cancel_send(req: *mut mxm_send_req_t) -> mxm_error_t;
	pub fn mxm_req_mprobe(req: *mut mxm_recv_req_t, msgp: *mut mxm_message_h) -> mxm_error_t;
	pub fn mxm_req_probe(req: *mut mxm_recv_req_t) -> mxm_error_t;
	pub fn mxm_req_recv(req: *mut mxm_recv_req_t) -> mxm_error_t;
	pub fn mxm_req_send(req: *mut mxm_send_req_t) -> mxm_error_t;
}
