// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn uct_wakeup_close(wakeup: uct_wakeup_h);
	pub fn uct_wakeup_efd_arm(wakeup: uct_wakeup_h) -> ucs_status_t;
	pub fn uct_wakeup_efd_get(wakeup: uct_wakeup_h, fd_p: *mut c_int) -> ucs_status_t;
	pub fn uct_wakeup_open(iface: uct_iface_h, events: c_uint, wakeup_p: *mut uct_wakeup_h) -> ucs_status_t;
	pub fn uct_wakeup_signal(wakeup: uct_wakeup_h) -> ucs_status_t;
	pub fn uct_wakeup_wait(wakeup: uct_wakeup_h) -> ucs_status_t;
}
