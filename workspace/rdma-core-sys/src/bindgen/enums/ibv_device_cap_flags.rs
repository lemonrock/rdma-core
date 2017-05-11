// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_device_cap_flags
{
	IBV_DEVICE_RESIZE_MAX_WR = 1,
	IBV_DEVICE_BAD_PKEY_CNTR = 2,
	IBV_DEVICE_BAD_QKEY_CNTR = 4,
	IBV_DEVICE_RAW_MULTI = 8,
	IBV_DEVICE_AUTO_PATH_MIG = 16,
	IBV_DEVICE_CHANGE_PHY_PORT = 32,
	IBV_DEVICE_UD_AV_PORT_ENFORCE = 64,
	IBV_DEVICE_CURR_QP_STATE_MOD = 128,
	IBV_DEVICE_SHUTDOWN_PORT = 256,
	IBV_DEVICE_INIT_TYPE = 512,
	IBV_DEVICE_PORT_ACTIVE_EVENT = 1024,
	IBV_DEVICE_SYS_IMAGE_GUID = 2048,
	IBV_DEVICE_RC_RNR_NAK_GEN = 4096,
	IBV_DEVICE_SRQ_RESIZE = 8192,
	IBV_DEVICE_N_NOTIFY_CQ = 16384,
	IBV_DEVICE_MEM_WINDOW = 131072,
	IBV_DEVICE_UD_IP_CSUM = 262144,
	IBV_DEVICE_XRC = 1048576,
	IBV_DEVICE_MEM_MGT_EXTENSIONS = 2097152,
	IBV_DEVICE_MEM_WINDOW_TYPE_2A = 8388608,
	IBV_DEVICE_MEM_WINDOW_TYPE_2B = 16777216,
	IBV_DEVICE_RC_IP_CSUM = 33554432,
	IBV_DEVICE_RAW_IP_CSUM = 67108864,
	IBV_DEVICE_MANAGED_FLOW_STEERING = 536870912,
}
