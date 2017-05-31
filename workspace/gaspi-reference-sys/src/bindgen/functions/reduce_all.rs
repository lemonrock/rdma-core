// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn gaspi_allreduce(buffer_send: gaspi_const_pointer_t, buffer_receive: gaspi_pointer_t, num: gaspi_number_t, operation: gaspi_operation_t, datatype: gaspi_datatype_t, group: gaspi_group_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_allreduce_buf_size(buf_size: *mut gaspi_size_t) -> gaspi_return_t;
	pub fn gaspi_allreduce_elem_max(elem_max: *mut gaspi_number_t) -> gaspi_return_t;
	pub fn gaspi_allreduce_user(buffer_send: gaspi_const_pointer_t, buffer_receive: gaspi_pointer_t, num: gaspi_number_t, size_element: gaspi_size_t, reduce_operation: gaspi_reduce_operation_t, reduce_state: gaspi_reduce_state_t, group: gaspi_group_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
}
