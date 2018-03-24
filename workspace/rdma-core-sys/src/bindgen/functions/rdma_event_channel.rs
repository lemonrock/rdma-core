// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_rdma_create_event_channel"] pub fn rdma_create_event_channel() -> *mut rdma_event_channel;
	#[link_name = "\u{1}_rdma_create_id"] pub fn rdma_create_id(channel: *mut rdma_event_channel, id: *mut *mut rdma_cm_id, context: *mut c_void, ps: rdma_port_space) -> c_int;
	#[link_name = "\u{1}_rdma_destroy_event_channel"] pub fn rdma_destroy_event_channel(channel: *mut rdma_event_channel);
	#[link_name = "\u{1}_rdma_get_cm_event"] pub fn rdma_get_cm_event(channel: *mut rdma_event_channel, event: *mut *mut rdma_cm_event) -> c_int;
}
