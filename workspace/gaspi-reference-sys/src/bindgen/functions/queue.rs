// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn gaspi_queue_create(queue: *mut gaspi_queue_id_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_queue_delete(queue: gaspi_queue_id_t) -> gaspi_return_t;
	pub fn gaspi_queue_max(queue_max: *mut gaspi_number_t) -> gaspi_return_t;
	pub fn gaspi_queue_num(queue_num: *mut gaspi_number_t) -> gaspi_return_t;
	pub fn gaspi_queue_purge(queue: gaspi_queue_id_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_queue_size(queue: gaspi_queue_id_t, queue_size: *mut gaspi_number_t) -> gaspi_return_t;
	pub fn gaspi_queue_size_max(queue_size_max: *mut gaspi_number_t) -> gaspi_return_t;
}
