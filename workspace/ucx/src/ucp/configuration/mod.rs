// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;
use ::std::collections::HashSet;
use ::std::ffi::CString;
use ::libc::c_char;


include!("ConfigurationKey.rs");
include!("ConfigurationSetting.rs");
include!("MemoryUnitsConfigurationKey.rs");
include!("MemoryUnitsConfigurationSetting.rs");
include!("StringArrayConfigurationSetting.rs");
include!("StringArrayConfigurationKey.rs");



/*


{"RNDV_PERF_DIFF", "1",
"The percentage allowed for performance difference between rendezvous and "
"the eager_zcopy protocol",
ucs_offsetof(ucp_config_t, ctx.rndv_perf_diff), UCS_CONFIG_TYPE_DOUBLE},

{"ATOMIC_MODE", "guess",
"Atomic operations synchronization mode.\n"
" cpu    - atomic operations are consistent with respect to the CPU.\n"
" device - atomic operations are performed on one of the transport devices,\n"
"          and there is guarantee of consistency with respect to the CPU."
" guess  - atomic operations mode is configured based on underlying\n"
"          transport capabilities. If one of active transports supports\n"
"          the DEVICE atomic mode, the DEVICE mode is selected.\n"
"          Otherwise the CPU mode is selected.",
ucs_offsetof(ucp_config_t, ctx.atomic_mode), UCS_CONFIG_TYPE_ENUM(ucp_atomic_modes)},

{"MAX_WORKER_NAME", UCS_PP_MAKE_STRING(UCP_WORKER_NAME_MAX),
"Maximal length of worker name. Affects the size of worker address in debug builds.",
ucs_offsetof(ucp_config_t, ctx.max_worker_name), UCS_CONFIG_TYPE_UINT},

{"USE_MT_MUTEX", "n", "Use mutex for multithreading support in UCP.\n"
"n      - Not use mutex for multithreading support in UCP (use spinlock by default).\n"
"y      - Use mutex for multithreading support in UCP.\n",
ucs_offsetof(ucp_config_t, ctx.use_mt_mutex), UCS_CONFIG_TYPE_BOOL},

{NULL}
};

*/

