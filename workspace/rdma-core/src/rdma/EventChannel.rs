// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct EventChannel<'a>
{
	ePoll: &'a EPoll<EPollContextChoice<'a>>,
	eventChannel: *mut rdma_event_channel,
	listeners: HashSet<*mut rdma_cm_id>,
	verbMap: VerbMap<'a, EventChannelVerbMapEntry<'a>>
}

impl<'a> Drop for EventChannel<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.verbMap.destroy();
		
		for listener in self.listeners.drain()
		{
			listener.destroy();
		}
		
		let fileDescriptor = self.fileDescriptorForEPoll();
		self.ePoll.deregister(fileDescriptor);
		self.eventChannel.destroy();
	}
}

impl<'a> EPollContext for EventChannel<'a>
{
	#[inline(always)]
	fn fileDescriptorForEPoll(&self) -> RawFd
	{
		self.eventChannel.fileDescriptorForEPoll()
	}
	
	#[inline(always)]
	fn processEvents(&mut self)
	{
		loop
		{
			let mut event = unsafe { uninitialized() };
			
			let result = unsafe { rdma_get_cm_event(self.eventChannel, &mut event) };
			debug_assert!(result == 0 || result == -1, "rdma_get_cm_event returned a result '{}' which was not 0 or -1", result);
			
			if unlikely(result == -1)
			{
				debug_assert!(event.is_null(), "event was not null");
				
				let errno = errno();
				if likely(errno.0 == E::EAGAIN)
				{
					return;
				}
				panic!("rdma_get_cm_event failed with error number '{}' ('{}')", errno.0, errno);
			}
			else
			{
				debug_assert!(!event.is_null(), "event was null - this should not occur");
				
				match event.eventType()
				{
					// rdma_resolve_addr()
					RDMA_CM_EVENT_ADDR_RESOLVED =>
					{
						let communicationIdentifierContext = event.reconstituteContext();
						communicationIdentifierContext.addressResolutionSucceeded(event.communicationIdentifier());
						event.acknowledge();
						forget(communicationIdentifierContext);
					},
					RDMA_CM_EVENT_ADDR_ERROR =>
					{
						let communicationIdentifierContext = event.reconstituteContext();
						let statusCode = event.nonZeroStatusCode();
						event.acknowledge();
						if likely(communicationIdentifierContext.addressResolutionFailed(statusCode))
						{
							forget(communicationIdentifierContext);
							event.communicationIdentifier().destroy();
						}
						else
						{
							forget(communicationIdentifierContext);
						}
					},
					
					// rdma_resolve_route()
					RDMA_CM_EVENT_ROUTE_RESOLVED =>
					{
						let communicationIdentifierContext = event.reconstituteContext();
						communicationIdentifierContext.routeResolutionSucceeded();
						event.acknowledge();
						forget(communicationIdentifierContext);
					},
					RDMA_CM_EVENT_ROUTE_ERROR =>
					{
						let communicationIdentifierContext = event.reconstituteContext();
						communicationIdentifierContext.routeResolutionFailed(event.nonZeroStatusCode());
						event.acknowledge();
						forget(communicationIdentifierContext);
					},
					
					RDMA_CM_EVENT_CONNECT_REQUEST =>
					{
						let listenerCommunicationIdentifier = event.listenerCommunicationIdentifier();
						debug_assert!(!listenerCommunicationIdentifier.is_null(), "listenerCommunicationIdentifier is null for a connect request");
						let mut listenerCommunicationIdentifierContext = listenerCommunicationIdentifier.reconstituteContext();
						
						let newCommunicationIdentifierWithNoContextYet = event.communicationIdentifier();
						debug_assert!(!newCommunicationIdentifierWithNoContextYet.is_null(), "newCommunicationIdentifierWithNoContextYet is null for a connect request");
						
						let mut privateDataBuffer = unsafe { uninitialized() };
						let (privateDataLength, result) = match event.portSpace()
						{
							RDMA_PS_TCP | RDMA_PS_IB =>
							{
								listenerCommunicationIdentifierContext.reliableConnectionRequest(newCommunicationIdentifierWithNoContextYet, RequestedConnectionEventData(event), &mut privateDataBuffer)
							},
							RDMA_PS_IPOIB | RDMA_PS_UDP =>
							{
								listenerCommunicationIdentifierContext.unreliableDatagramConnectionRequest(newCommunicationIdentifierWithNoContextYet, &mut privateDataBuffer)
							},
						};
						if unlikely(result.is_err())
						{
							if privateDataLength == 0
							{
								newCommunicationIdentifierWithNoContextYet.rejectWithoutPrivateData();
							}
							else
							{
								newCommunicationIdentifierWithNoContextYet.rejectWithPrivateData(&privateDataBuffer, privateDataLength);
							}
							event.acknowledge();
							newCommunicationIdentifierWithNoContextYet.destroy();
						}
						else
						{
							let connectionAcceptance = result.unwrap();
							let mut parameter = unsafe { uninitialized() };
							connectionAcceptance.populate(&privateDataBuffer, privateDataLength, &mut parameter);
							newCommunicationIdentifierWithNoContextYet.accept(&mut parameter);
							event.acknowledge();
						}
						
						forget(privateDataBuffer);
						forget(listenerCommunicationIdentifierContext)
					},
					
					// Client side
					RDMA_CM_EVENT_CONNECT_RESPONSE =>
					{
						let communicationIdentifierContext = event.reconstituteContext();
						communicationIdentifierContext.connectionResponse();
						event.acknowledge();
						forget(communicationIdentifierContext);
					},
					
					// Client or Server side
					RDMA_CM_EVENT_CONNECT_ERROR =>
					{
						let communicationIdentifierContext = event.reconstituteContext();
						communicationIdentifierContext.connectionError(event.statusCode());
						event.acknowledge();
						forget(communicationIdentifierContext);
					},
					
					// ?Client side
					RDMA_CM_EVENT_UNREACHABLE =>
					{
						let communicationIdentifierContext = event.reconstituteContext();
						communicationIdentifierContext.unreachable(event.statusCode());
						event.acknowledge();
						forget(communicationIdentifierContext);
					},
					
					// ?Client side
					RDMA_CM_EVENT_REJECTED =>
					{
						let communicationIdentifierContext = event.reconstituteContext();
						communicationIdentifierContext.connectionRequestOrResponseRejected(event.statusCode());
						event.acknowledge();
						forget(communicationIdentifierContext);
					},
					
					// Until this occurs, a server can not assume a connection has been established
					RDMA_CM_EVENT_ESTABLISHED =>
					{
						let communicationIdentifierContext = event.reconstituteContext();
						match event.portSpace()
						{
							RDMA_PS_TCP | RDMA_PS_IB => communicationIdentifierContext.connectionEstablished(EstablishedConnectionEventData(event)),
							RDMA_PS_IPOIB | RDMA_PS_UDP => communicationIdentifierContext.multicastEstablished(UnreliableDatagramEventData(event)),
						}
						event.acknowledge();
						forget(communicationIdentifierContext);
					},
					
					RDMA_CM_EVENT_DISCONNECTED =>
					{
						let communicationIdentifierContext = event.reconstituteContext();
						communicationIdentifierContext.disconnected();
						event.acknowledge();
						forget(communicationIdentifierContext);
						
						event.communicationIdentifier().destroy();
					},
					
					RDMA_CM_EVENT_DEVICE_REMOVAL =>
					{
						let communicationIdentifierContext = event.reconstituteContext();
						communicationIdentifierContext.deviceRemoval(event.statusCode());
						event.acknowledge();
						
						event.communicationIdentifier().destroy();
					},
					
					RDMA_CM_EVENT_MULTICAST_JOIN =>
					{
						let communicationIdentifierContext = event.reconstituteContext();
						communicationIdentifierContext.multicastJoin(UnreliableDatagramEventData(event));
						event.acknowledge();
						forget(communicationIdentifierContext);
					},
					RDMA_CM_EVENT_MULTICAST_ERROR =>
					{
						let communicationIdentifierContext = event.reconstituteContext();
						communicationIdentifierContext.multicastError(event.nonZeroStatusCode());
						event.acknowledge();
						forget(communicationIdentifierContext);
					},
					
					RDMA_CM_EVENT_ADDR_CHANGE =>
					{
						let communicationIdentifierContext = event.reconstituteContext();
						communicationIdentifierContext.addressChange(event.statusCode());
						event.acknowledge();
						forget(communicationIdentifierContext);
					},
					
					RDMA_CM_EVENT_TIMEWAIT_EXIT =>
					{
						let communicationIdentifierContext = event.reconstituteContext();
						communicationIdentifierContext.timeWaitExit();
						event.acknowledge();
						forget(communicationIdentifierContext);
					},
				}
			}
		}
	}
}

impl<'a> EventChannel<'a>
{
	#[inline(always)]
	pub fn assignNewEventChannelToEPoll(ePoll: &'a EPoll<EPollContextChoice<'a>>, mut listeners: Vec<Listener>) -> Box<EPollContextChoice>
	{
		let pointer = panic_on_null!(rdma_create_event_channel);

		let mut eventChannel = EventChannel
		{
			ePoll: ePoll,
			eventChannel: pointer,
			verbMap: VerbMap::new(ePoll),
			listeners: HashSet::with_capacity(listeners.len()),
		};

		for listener in listeners.drain(..)
		{
			eventChannel.listeners.insert(listener.to_rdma_cm_id(pointer));
		}
		
		ePoll.registerEdgeTriggeredIn(EPollContextChoice::EventChannel(eventChannel))
	}
}
