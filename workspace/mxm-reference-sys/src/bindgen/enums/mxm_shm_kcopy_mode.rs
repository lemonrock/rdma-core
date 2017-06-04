// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum mxm_shm_kcopy_mode
{
	MXM_SHM_KCOPY_MODE_OFF = 0,
	MXM_SHM_KCOPY_MODE_KNEM = 1,
	MXM_SHM_KCOPY_MODE_AUTODETECT = 2,
	MXM_SHM_KCOPY_MODE_LAST = 3,
}
