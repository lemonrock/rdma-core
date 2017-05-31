// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn gaspi_barrier(group: gaspi_group_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_connect(rank: gaspi_rank_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_disconnect(rank: gaspi_rank_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_notify(segment_id: gaspi_segment_id_t, rank: gaspi_rank_t, notification_id: gaspi_notification_id_t, notification_value: gaspi_notification_t, queue: gaspi_queue_id_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_notify_reset(segment_id: gaspi_segment_id_t, notification_id: gaspi_notification_id_t, old_notification_val: *mut gaspi_notification_t) -> gaspi_return_t;
	pub fn gaspi_notify_waitsome(segment_id: gaspi_segment_id_t, notific_begin: gaspi_notification_id_t, notification_num: gaspi_number_t, first_id: *mut gaspi_notification_id_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_wait(queue: gaspi_queue_id_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
}
