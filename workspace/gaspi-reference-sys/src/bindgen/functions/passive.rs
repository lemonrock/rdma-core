// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn gaspi_passive_queue_purge(timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_passive_receive(segment_id_local: gaspi_segment_id_t, offset_local: gaspi_offset_t, rank: *mut gaspi_rank_t, size: gaspi_size_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_passive_send(segment_id_local: gaspi_segment_id_t, offset_local: gaspi_offset_t, rank: gaspi_rank_t, size: gaspi_size_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_passive_transfer_size_max(transfer_size_max: *mut gaspi_size_t) -> gaspi_return_t;
}
