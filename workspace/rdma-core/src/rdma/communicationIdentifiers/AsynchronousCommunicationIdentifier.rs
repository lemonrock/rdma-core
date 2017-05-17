// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct AsynchronousCommunicationIdentifier<'a, Context, C: CommunicationEventHandler>
where C: 'a
{
	communicationIdentifier: CommunicationIdentifier<Context>,
	#[allow(dead_code)] eventChannel: &'a EventChannel<C>
}

impl<'a, Context, C: CommunicationEventHandler> AsynchronousCommunicationIdentifier<'a, Context, C>
{
	#[inline(always)]
	pub(crate) fn newListening(context: Rc<RefCell<Context>>, addressing: Addressing, backLog: u32, eventChannel: &'a EventChannel<C>) -> Self
	{
		Self
		{
			communicationIdentifier: CommunicationIdentifier::newListeningAsynchronous(context, addressing, backLog, eventChannel),
			eventChannel: eventChannel,
		}
	}
}
