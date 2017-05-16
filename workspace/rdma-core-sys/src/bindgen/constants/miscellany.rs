// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub const IBV_PATH_FLAG_ALTERNATE: c_uint = 4;
pub const IBV_PATH_FLAG_BIDIRECTIONAL: c_uint = 40;
pub const IBV_PATH_FLAG_GMP: c_uint = 1;
pub const IBV_PATH_FLAG_INBOUND: c_uint = 16;
pub const IBV_PATH_FLAG_INBOUND_REVERSE: c_uint = 32;
pub const IBV_PATH_FLAG_OUTBOUND: c_uint = 8;
pub const IBV_PATH_FLAG_PRIMARY: c_uint = 2;
pub const IBV_PATH_RECORD_REVERSIBLE: c_uint = 128;
pub const RAI_FAMILY: c_uint = 8;
pub const RAI_NOROUTE: c_uint = 4;
pub const RAI_NUMERICHOST: c_uint = 2;
pub const RAI_PASSIVE: c_uint = 1;
pub const RDMA_IB_IP_PORT_MASK: c_uint = 65535;
pub const RDMA_IB_IP_PS_MASK: c_int = -65536;
pub const RDMA_IB_IP_PS_TCP: c_uint = 17170432;
pub const RDMA_IB_IP_PS_UDP: c_uint = 17891328;
pub const RDMA_IB_PS_IB: c_uint = 20905984;
pub const RDMA_MAX_INIT_DEPTH: _bindgen_ty_6 = _bindgen_ty_6::RDMA_MAX_RESP_RES;
pub const RDMA_MAX_RESP_RES: _bindgen_ty_6 = _bindgen_ty_6::RDMA_MAX_RESP_RES;
pub const RDMA_OPTION_IB: _bindgen_ty_7 = _bindgen_ty_7::RDMA_OPTION_IB;
pub const RDMA_OPTION_IB_PATH: _bindgen_ty_8 = _bindgen_ty_8::RDMA_OPTION_ID_REUSEADDR;
pub const RDMA_OPTION_ID: _bindgen_ty_7 = _bindgen_ty_7::RDMA_OPTION_ID;
pub const RDMA_OPTION_ID_AFONLY: _bindgen_ty_8 = _bindgen_ty_8::RDMA_OPTION_ID_AFONLY;
pub const RDMA_OPTION_ID_REUSEADDR: _bindgen_ty_8 = _bindgen_ty_8::RDMA_OPTION_ID_REUSEADDR;
pub const RDMA_OPTION_ID_TOS: _bindgen_ty_8 = _bindgen_ty_8::RDMA_OPTION_ID_TOS;
pub const RDMA_UDP_QKEY: c_uint = 19088743;
pub const ibv_xrcd_init_attr_mask_IBV_XRCD_INIT_ATTR_FD: ibv_xrcd_init_attr_mask = ibv_xrcd_init_attr_mask(1);
pub const ibv_xrcd_init_attr_mask_IBV_XRCD_INIT_ATTR_OFLAGS: ibv_xrcd_init_attr_mask = ibv_xrcd_init_attr_mask(2);
pub const ibv_xrcd_init_attr_mask_IBV_XRCD_INIT_ATTR_RESERVED: ibv_xrcd_init_attr_mask = ibv_xrcd_init_attr_mask(4);
