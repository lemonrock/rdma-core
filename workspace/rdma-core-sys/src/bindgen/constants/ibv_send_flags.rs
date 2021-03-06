// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub const ibv_send_flags_IBV_SEND_FENCE: ibv_send_flags = ibv_send_flags(1);
pub const ibv_send_flags_IBV_SEND_INLINE: ibv_send_flags = ibv_send_flags(8);
pub const ibv_send_flags_IBV_SEND_IP_CSUM: ibv_send_flags = ibv_send_flags(16);
pub const ibv_send_flags_IBV_SEND_SIGNALED: ibv_send_flags = ibv_send_flags(2);
pub const ibv_send_flags_IBV_SEND_SOLICITED: ibv_send_flags = ibv_send_flags(4);
