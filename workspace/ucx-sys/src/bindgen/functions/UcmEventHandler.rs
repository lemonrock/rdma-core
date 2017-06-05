// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn ucm_set_event_handler(events: c_int, priority: c_int, cb: ucm_event_callback_t, arg: *mut c_void) -> ucs_status_t;
	pub fn ucm_set_external_event(events: c_int);
	pub fn ucm_unset_event_handler(events: c_int, cb: ucm_event_callback_t, arg: *mut c_void);
	pub fn ucm_unset_external_event(events: c_int);
}
