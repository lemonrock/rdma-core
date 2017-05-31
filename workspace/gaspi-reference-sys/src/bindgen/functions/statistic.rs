// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn gaspi_statistic_counter_get(counter: gaspi_statistic_counter_t, argument: gaspi_statistic_argument_t, value: *mut gaspi_number_t) -> gaspi_return_t;
	pub fn gaspi_statistic_counter_info(counter: gaspi_statistic_counter_t, argument: *mut gaspi_statistic_argument_t, counter_name: *mut gaspi_string_t, counter_description: *mut gaspi_string_t, verbosity_level: *mut gaspi_number_t) -> gaspi_return_t;
	pub fn gaspi_statistic_counter_max(counter_max: *mut gaspi_number_t) -> gaspi_return_t;
	pub fn gaspi_statistic_counter_reset(counter: gaspi_statistic_counter_t) -> gaspi_return_t;
	pub fn gaspi_statistic_verbosity_level(verbosity_level: gaspi_number_t) -> gaspi_return_t;
}
