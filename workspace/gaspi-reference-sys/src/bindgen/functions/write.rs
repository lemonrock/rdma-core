// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn gaspi_write(segment_id_local: gaspi_segment_id_t, offset_local: gaspi_offset_t, rank: gaspi_rank_t, segment_id_remote: gaspi_segment_id_t, offset_remote: gaspi_offset_t, size: gaspi_size_t, queue: gaspi_queue_id_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_write_list(num: gaspi_number_t, segment_id_local: *const gaspi_segment_id_t, offset_local: *const gaspi_offset_t, rank: gaspi_rank_t, segment_id_remote: *const gaspi_segment_id_t, offset_remote: *const gaspi_offset_t, size: *const gaspi_size_t, queue: gaspi_queue_id_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_write_list_notify(num: gaspi_number_t, segment_id_local: *const gaspi_segment_id_t, offset_local: *const gaspi_offset_t, rank: gaspi_rank_t, segment_id_remote: *const gaspi_segment_id_t, offset_remote: *const gaspi_offset_t, size: *const gaspi_size_t, notification_id: gaspi_notification_id_t, notification_value: gaspi_notification_t, queue: gaspi_queue_id_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_write_notify(segment_id_local: gaspi_segment_id_t, offset_local: gaspi_offset_t, rank: gaspi_rank_t, segment_id_remote: gaspi_segment_id_t, offset_remote: gaspi_offset_t, size: gaspi_size_t, notification_id: gaspi_notification_id_t, notification_value: gaspi_notification_t, queue: gaspi_queue_id_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
}
