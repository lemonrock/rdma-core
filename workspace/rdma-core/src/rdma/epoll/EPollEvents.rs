// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub mod EPollEvents
{
	use ::rdma_core_sys::*;
	
	bitflags!
	{
		#[derive(Default)]
		pub flags Flags: u32
		{
			const EdgeTriggered = EPOLLET,
			
			/// Never needs to be set in events
			const Error = EPOLLERR,
			
			const Exclusive = EPOLLEXCLUSIVE,
			
			/// Only ever returned from epoll_wait(); never should be set
			const HangUp = EPOLLHUP,
			
			const In = EPOLLIN,
			
			const Message = EPOLLMSG,
			
			const Out = EPOLLOUT,
			
			/// Immediate data, TCP priority data
			const Priority = EPOLLPRI,
			
			/// Re-Arm will modify() after event fired
			const OneShot = EPOLLONESHOT,
			
			/// Out of band data
			const ReadOutOfBandData = EPOLLRDBAND,
			const ReadNormal = EPOLLRDNORM,
			const ReadHangUp = EPOLLRDHUP,
			
			/// Meaningless and only for backward compatibility
			const WakeUp = EPOLLWAKEUP,
			
			const WriteOutOfBandData = EPOLLWRBAND,
			const WriteNormal = EPOLLWRNORM,
		}
	}
}

