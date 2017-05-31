// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn gaspi_build_infrastructure(build_infrastructure: *mut gaspi_number_t) -> gaspi_return_t;
	pub fn gaspi_network_type(network_type: *mut gaspi_network_t) -> gaspi_return_t;
	pub fn gaspi_notification_num(notification_num: *mut gaspi_number_t) -> gaspi_return_t;
	pub fn gaspi_print_error(error_code: gaspi_return_t, error_message: *mut gaspi_string_t) -> gaspi_return_t;
	pub fn gaspi_state_vec_get(state_vector: *mut gaspi_state_vector_t) -> gaspi_return_t;
	pub fn gaspi_transfer_size_max(transfer_size_max: *mut gaspi_size_t) -> gaspi_return_t;
	pub fn gaspi_version(version: *mut f32) -> gaspi_return_t;
}
