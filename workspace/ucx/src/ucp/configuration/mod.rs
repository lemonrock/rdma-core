// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;
use ::std::collections::HashSet;
use ::std::ffi::CString;
use ::libc::c_char;


include!("ConfigurationKey.rs");
include!("ConfigurationSetting.rs");
include!("StringArrayConfigurationSetting.rs");
include!("StringArrayConfigurationKey.rs");


/*

static ucs_config_field_t ucp_config_table[] = {
{"NET_DEVICES", "all",
"Specifies which network device(s) to use. The order is not meaningful.\n"
"\"all\" would use all available devices.",
ucs_offsetof(ucp_config_t, devices[UCT_DEVICE_TYPE_NET]), UCS_CONFIG_TYPE_STRING_ARRAY},

{"SHM_DEVICES", "all",
"Specifies which intra-node device(s) to use. The order is not meaningful.\n"
"\"all\" would use all available devices.",
ucs_offsetof(ucp_config_t, devices[UCT_DEVICE_TYPE_SHM]), UCS_CONFIG_TYPE_STRING_ARRAY},

{"ACC_DEVICES", "all",
"Specifies which accelerator device(s) to use. The order is not meaningful.\n"
"\"all\" would use all available devices.",
ucs_offsetof(ucp_config_t, devices[UCT_DEVICE_TYPE_ACC]), UCS_CONFIG_TYPE_STRING_ARRAY},

{"SELF_DEVICES", "all",
"Specifies which loop-back device(s) to use. The order is not meaningful.\n"
"\"all\" would use all available devices.",
ucs_offsetof(ucp_config_t, devices[UCT_DEVICE_TYPE_SELF]), UCS_CONFIG_TYPE_STRING_ARRAY},

{"TLS", "all",
"Comma-separated list of transports to use. The order is not meaningful.\n"
"In addition it's possible to use a combination of the following aliases:\n"
" - all    : use all the available transports.\n"
" - sm/shm : all shared memory transports.\n"
" - mm     : shared memory transports - only memory mappers.\n"
" - ugni   : ugni_rdma and ugni_udt.\n"
" - ib     : all infiniband transports.\n"
" - rc     : rc and ud.\n"
" - rc_x   : rc with accelerated verbs and ud.\n"
" - ud_x   : ud with accelerated verbs.\n"
" - dc_x   : dc with accelerated verbs.\n"
" Using a \\ prefix before a transport name treats it as an explicit transport name\n"
" and disables aliasing.\n",
ucs_offsetof(ucp_config_t, tls), UCS_CONFIG_TYPE_STRING_ARRAY},

{"ALLOC_PRIO", "md:sysv,md:posix,huge,thp,md:*,mmap,heap",
"Priority of memory allocation methods. Each item in the list can be either\n"
"an allocation method (huge, thp, mmap, libc) or md:<NAME> which means to use the\n"
"specified memory domain for allocation. NAME can be either a MD component\n"
"name, or a wildcard - '*' - which expands to all MD components.",
ucs_offsetof(ucp_config_t, alloc_prio), UCS_CONFIG_TYPE_STRING_ARRAY},

{"BCOPY_THRESH", "0",
"Threshold for switching from short to bcopy protocol",
ucs_offsetof(ucp_config_t, ctx.bcopy_thresh), UCS_CONFIG_TYPE_MEMUNITS},

{"RNDV_THRESH", "auto",
"Threshold for switching from eager to rendezvous protocol",
ucs_offsetof(ucp_config_t, ctx.rndv_thresh), UCS_CONFIG_TYPE_MEMUNITS},

{"RNDV_THRESH_FALLBACK", "inf",
"Message size to start using the rendezvous protocol in case the calculated threshold "
"is zero or negative",
ucs_offsetof(ucp_config_t, ctx.rndv_thresh_fallback), UCS_CONFIG_TYPE_MEMUNITS},

{"RNDV_PERF_DIFF", "1",
"The percentage allowed for performance difference between rendezvous and "
"the eager_zcopy protocol",
ucs_offsetof(ucp_config_t, ctx.rndv_perf_diff), UCS_CONFIG_TYPE_DOUBLE},

{"ZCOPY_THRESH", "auto",
"Threshold for switching from buffer copy to zero copy protocol",
ucs_offsetof(ucp_config_t, ctx.zcopy_thresh), UCS_CONFIG_TYPE_MEMUNITS},

{"BCOPY_BW", "5800mb",
"Estimation of buffer copy bandwidth",
ucs_offsetof(ucp_config_t, ctx.bcopy_bw), UCS_CONFIG_TYPE_MEMUNITS},

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

{"TM_THRESH", "1024",
"Threshold for using tag matching offload capabilities.\n"
"Smaller buffers will not be posted to the transport.",
ucs_offsetof(ucp_config_t, ctx.tm_thresh), UCS_CONFIG_TYPE_MEMUNITS},

{NULL}
};

*/

