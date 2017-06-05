// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn uct_worker_create(async: *mut ucs_async_context_t, thread_mode: ucs_thread_mode_t, worker_p: *mut uct_worker_h) -> ucs_status_t;
	pub fn uct_worker_destroy(worker: uct_worker_h);
	pub fn uct_worker_progress(worker: uct_worker_h);
	pub fn uct_worker_progress_register(worker: uct_worker_h, func: ucs_callback_t, arg: *mut c_void);
	pub fn uct_worker_progress_unregister(worker: uct_worker_h, func: ucs_callback_t, arg: *mut c_void);
	pub fn uct_worker_slowpath_progress_register(worker: uct_worker_h, elem: *mut ucs_callbackq_slow_elem_t);
	pub fn uct_worker_slowpath_progress_unregister(worker: uct_worker_h, elem: *mut ucs_callbackq_slow_elem_t);
}
