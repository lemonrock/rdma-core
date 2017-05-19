// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub mod AttributeFlags
{
	bitflags!
	{
		#[derive(Default)]
		pub flags Flags: i32
		{
			const None = 0,
			
			/// Modify qp_state
			const STATE = 1 << 0,
			
			/// Set cur_qp_state
			const CUR_STATE = 1 << 1,
			
			/// Set en_sqd_async_notify
			const EN_SQD_ASYNC_NOTIFY = 1 << 2,
			
			/// Set qp_access_flags
			const ACCESS_FLAGS = 1 << 3,
			
			/// Set pkey_index
			const PKEY_INDEX = 1 << 4,
			
			/// Set port_num
			const PORT = 1 << 5,
			
			/// Set qkey
			const QKEY = 1 << 6,
			
			/// Set ah_attr
			const AV = 1 << 7,
			
			/// Set path_mtu
			const PATH_MTU = 1 << 8,
			
			/// Set timeout
			const TIMEOUT = 1 << 9,
			
			/// Set retry_cnt
			const RETRY_CNT = 1 << 10,
			
			/// Set rnr_retry
			const RNR_RETRY = 1 << 11,
			
			/// Set rq_psn
			const RQ_PSN = 1 << 12,
			
			/// Set max_rd_atomic
			const MAX_QP_RD_ATOMIC = 1 << 13,
			
			/// Set the alternative path via: alt_ah_attr, alt_pkey_index, alt_port_num, alt_timeout
			const ALT_PATH = 1 << 14,
			
			/// Set min_rnr_timer
			const MIN_RNR_TIMER = 1 << 15,
			
			/// Set sq_psn
			const SQ_PSN = 1 << 16,
			
			/// Set max_dest_rd_atomic
			const MAX_DEST_RD_ATOMIC = 1 << 17,
			
			/// Set path_mig_state
			const PATH_MIG_STATE = 1 << 18,
			
			/// Set cap
			const CAP = 1 << 19,
			
			/// Set dest_qp_num
			const DEST_QPN = 1 << 20,
			
			/// ?
			const RATE_LIMIT = 1 << 25,
			
			const All = STATE.bits | CUR_STATE.bits | EN_SQD_ASYNC_NOTIFY.bits | ACCESS_FLAGS.bits | PKEY_INDEX.bits | PORT.bits | QKEY.bits | AV.bits | PATH_MTU.bits | TIMEOUT.bits | RETRY_CNT.bits | RNR_RETRY.bits | RQ_PSN.bits | MAX_QP_RD_ATOMIC.bits | ALT_PATH.bits | MIN_RNR_TIMER.bits | SQ_PSN.bits | MAX_DEST_RD_ATOMIC.bits | PATH_MIG_STATE.bits | CAP.bits | DEST_QPN.bits | RATE_LIMIT.bits,
		}
	}
}
