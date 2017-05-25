// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn epoll_create1(arg1: c_int) -> c_int;
	pub fn epoll_ctl(arg1: c_int, arg2: c_int, arg3: c_int, arg4: *mut epoll_event) -> c_int;
	pub fn epoll_wait(arg1: c_int, arg2: *mut epoll_event, arg3: c_int, arg4: c_int) -> c_int;
}
