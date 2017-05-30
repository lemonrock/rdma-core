// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_port_cap_flags
{
	IBV_PORT_SM = 2,
	IBV_PORT_NOTICE_SUP = 4,
	IBV_PORT_TRAP_SUP = 8,
	IBV_PORT_OPT_IPD_SUP = 16,
	IBV_PORT_AUTO_MIGR_SUP = 32,
	IBV_PORT_SL_MAP_SUP = 64,
	IBV_PORT_MKEY_NVRAM = 128,
	IBV_PORT_PKEY_NVRAM = 256,
	IBV_PORT_LED_INFO_SUP = 512,
	IBV_PORT_SYS_IMAGE_GUID_SUP = 2048,
	IBV_PORT_PKEY_SW_EXT_PORT_TRAP_SUP = 4096,
	IBV_PORT_EXTENDED_SPEEDS_SUP = 16384,
	IBV_PORT_CM_SUP = 65536,
	IBV_PORT_SNMP_TUNNEL_SUP = 131072,
	IBV_PORT_REINIT_SUP = 262144,
	IBV_PORT_DEVICE_MGMT_SUP = 524288,
	IBV_PORT_VENDOR_CLASS_SUP = 1048576,
	IBV_PORT_DR_NOTICE_SUP = 2097152,
	IBV_PORT_CAP_MASK_NOTICE_SUP = 4194304,
	IBV_PORT_BOOT_MGMT_SUP = 8388608,
	IBV_PORT_LINK_LATENCY_SUP = 16777216,
	IBV_PORT_CLIENT_REG_SUP = 33554432,
	IBV_PORT_IP_BASED_GIDS = 67108864,
}
