// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ucs_config_print_flags_t
{
	UCS_CONFIG_PRINT_CONFIG = 1,
	UCS_CONFIG_PRINT_HEADER = 2,
	UCS_CONFIG_PRINT_DOC = 4,
	UCS_CONFIG_PRINT_HIDDEN = 8,
}
