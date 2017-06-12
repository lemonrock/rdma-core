// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Worker
{
	handle: ucp_worker_h,
	dropWrapper: Rc<HandleDropWrapper<ucp_worker_h>>,
	applicationContextDropWrapper: Rc<HandleDropWrapper<ucp_context_h>>,
}

impl PrintInformation for Worker
{
	#[inline(always)]
	fn printInformationToStream(&self, stream: *mut FILE)
	{
		unsafe { ucp_worker_print_info(self.handle, stream) };
	}
}

impl QueryAttributes for Worker
{
	type Attributes = WorkerAttributes;
	
	#[inline(always)]
	fn queryAttributes(&self) -> Self::Attributes
	{
		use ucp_worker_attr_field::*;
		
		let mut attributes: ucp_worker_attr_t = unsafe { uninitialized() };
		attributes.field_mask = UCP_WORKER_ATTR_FIELD_THREAD_MODE as u64;
		panic_on_error!(ucp_worker_query, self.handle, &mut attributes);
		WorkerAttributes(attributes)
	}
}

impl Worker
{
	#[inline(always)]
	pub fn addressHandle(&self) -> WorkerAddressHandle
	{
		let mut addressHandle = unsafe { uninitialized() };
		let mut addressHandleLength = unsafe { uninitialized() };
		panic_on_error!(ucp_worker_get_address, self.handle, &mut addressHandle, &mut addressHandleLength);
		WorkerAddressHandle
		{
			handle: addressHandle,
			length: addressHandleLength,
			worker: self.clone(),
		}
	}
	
	
	/*
	
		For a given endpoint to be created, we need:-
			- The bytes (in a buffer) from the remote's ucs_status_t ucp_worker_get_address(ucp_worker_h worker)
				cf WorkerAddressHandle
			- For each RemoteMemoryAccessKey, which we need to be able to do more than send tagged messages, we need:-
				- We don't have to have any if all we want to do is send tagged messages
				- It is probably wise to optimise for either one or a few, but not many
				- We may want to maintain a map of logical memory region to actual memory region, eg
					"Memory Region for Storing Car Registrations" => remote endpoint 46
						- then we can just pass out &xxx endpoint handles
				- These can de dynamically created, but this is expensive; probably best to just get a set all at once or something like that
		
		So, to create an endpoint, we need an out-of-band way to get
			- Endpoint Name + Endpoint Address Handle Blob (from ucp_worker_get_address())
				- If we are thread-per-core that means Endpoint Names are effectively core addresses
			
			- Remote address map for that endpoint
				- could be sent at the same time
				- could be done by publishing a tag message (? how often ?)
			
			- If we are sharing a fabric, then we want to be able to isolate ourselves as a particular 'application name'
				- and instance
				- messages should probably share a particular random token
				- and be cryptographically signed / encrypted, using public-private key crypto
			
			- Multicast DNS / DNS-SD
				- Numerous examples, so far:-
					- Rust client: https://crates.io/crates/mdns
						- eg https://github.com/dylanmckay/mdns/blob/master/src/mdns.rs
					- https://bitbucket.org/geekman/tinysvcmdns
						- looks simple enough
						- runs on a thread, uses mutexes to communicate
						- creates the thread itself
						- uses select() (not really an issue)
						- we could re-use our DPDK infrastructure; after all, it's just UDP payloads
				
				
				- Outline
					- We run one dedicated thread
					- This opens a UDP port, 5353
						- listens for incoming messages
						- sends outgoing messages
						- This multicasts to / from 224.0.0.251
					- It uses epoll, wakes up every now and again
						- We have a queue of incoming job items (ie changes)
							- Announce a new worker (address handle)
							- Retire an old worker (address handle)
								- these could be done as SRV records, where port numbers are actually worker ids and protocols are actually types of service
							- Announce a worker + rkey
								- TXT records
							- Reture a worker + rkey
						- All incoming jobs get added to a hash table that is shared by all cores, so we have access to 'self' details for endpoints
							- logical endpoints vs physical
								- logical endpoints need a 'role'
					- It receives incoming messages and maintains a cache
					
					- BIG IDEA: Ditch the DNS-SD part as it's a bit of a force-fit
						- Run multicast UDP
						- Re-broadcast our data set at intervals, eg 1 second
						- can be effectively blocking
						
						UdpSocket::bind("some local addrress and port")
						set_read_timeout
						set_write_timeout
						set_nonblocking
							- don't really have to, if we specify a read / write timeout
						set_ttl(1)
						set_broadcast
							set_multicast_loop_v4(false)
							set_multicast_ttl_v4(1)
							join_multicast_v4()
							leave_multicast_v4()
							
							vs
							set_multicast_loop_v6(false)
							join_multicast_v6()
							leave_multicast_v6()
						
						then use
							recv_from()
							send_to()
							
					
					
				
				
				
				
				
				
				- See also https://tools.ietf.org/html/rfc6763
				- Only works on IPv4
				
			
				- hmmm.
				- Avahi was always a bit naff
				- ? libmdns ? / mdnsd /
					- https://github.com/haesbaert/mdnsd (OpenBSD)
					- Seems alive
					- DNS Service Discovery (DNS-SD): https://tools.ietf.org/html/rfc6763
				- https://github.com/videolabs/libmicrodns
				- https://github.com/grandcat/zeroconf (in Go)
			- We could use an out-of-band socket-based server
			- We could publish in DNS
				- TXT record is a max of 255 characters
				- one is allowed multiple TXT records
				- these do sadly overlap with other users
				- some sort of namespace at the beginning
					- RFC 1464 NAME=VALUE
					- ucx=<somestuff>
					<somestuff> should be base64 encrypted?
				- we're starting to get into the domain of SRV records
					- hmmmm... each endpoint publishes itself in SRV
			- We could publish as DHCP additional data
				- required DHCP on the network, with all sorts of potential implications; not usually that popular
			- broadcast to an IP multicast group
				- might work
				- seems to be on the periphery of zookeeper or corosync
			- We could try to use IB UD, but that seems like a lot of work
			
			- Note we can create a 'self' endpoint by passing ucp_worker_get_address() to ucp_ep_create()!
			
			- We need to know when someone joins the fabric (or leaves voluntarily)
	*/
	
	
	
	
	// ucp_worker_get_address() is used to get destinationAddress
	#[inline(always)]
	pub fn createEndPoint<ErrorHandler: EndPointErrorHandler>(&self, errorHandler: ErrorHandler, destinationAddress: *const ucp_address_t) -> Rc<EndPoint<ErrorHandler>>
	{
		assert!(!destinationAddress.is_null(), "destinationAddress is null");
		
		use ucp_ep_params_field::*;
		use ucp_err_handling_mode_t::*;
		
		let mut endPoint = Rc::new(EndPoint
		{
			handle: null_mut(),
			worker: self.clone(),
			parameters: ucp_ep_params_t
			{
				field_mask: UCP_EP_PARAM_FIELD_REMOTE_ADDRESS as u64 | UCP_EP_PARAM_FIELD_ERR_HANDLING_MODE as u64 | UCP_EP_PARAM_FIELD_ERR_HANDLER as u64,
				address: null(),
				err_mode: UCP_ERR_HANDLING_MODE_NONE,
				err_handler: ucp_err_handler_t
				{
					cb: Some(EndPoint::<ErrorHandler>::errorHandlerCallback),
					arg: null_mut(),
				},
			},
			errorHandler: errorHandler,
		});
		
		#[allow(trivial_casts)]
		{
			let weakReference = endPoint.clone().downgrade();
			// weak is just Shared<RcBox<T>>
			endPoint.parameters.err_handler.arg = endPoint.as_mut() as *mut _ as *mut c_void;
		}
		
		endPoint.connectOrReconnect(destinationAddress);
		
		endPoint
	}
	
	#[inline(always)]
	pub fn flushAllOutstandingRemoteMemoryAccessAndAtomicOperations(&self)
	{
		panic_on_error!(ucp_worker_flush, self.handle);
	}
	
	#[inline(always)]
	pub fn getFileDescriptorSuitableForEPoll(&self) -> RawFd
	{
		let mut fileDescriptorSuitableForEPoll = unsafe { uninitialized() };
		panic_on_error!(ucp_worker_get_efd, self.handle, &mut fileDescriptorSuitableForEPoll);
		fileDescriptorSuitableForEPoll
	}
	
	/// Returns an Err if internal logical returns UCS_ERR_IO_ERROR
	#[inline(always)]
	pub fn blockingWaitForAnyEvent(&self) -> Result<(), ()>
	{
		panic_on_error_with_clean_up!
		(
			status,
			{
				match status
				{
					ucs_status_t_UCS_ERR_IO_ERROR => return Err(()),
					ucs_status_t_UCS_ERR_NO_MEMORY => panic!("Out of memory in 'ucp_worker_wait'"),
					_ => (),
				};
			},
			ucp_worker_wait,
			self.handle
		);
		Ok(())
	}
	
	/// Returns an Err if internal logical returns UCS_ERR_IO_ERROR
	#[inline(always)]
	pub fn blockingWaitForAMemoryEvent(&self, address: *mut c_void)
	{
		unsafe { ucp_worker_wait_mem(self.handle, address) }
	}
	
	/// Returns 'true' if one should call ucp_worker_progress(), ie the worker can not arm because it is 'busy'
	///
	#[inline(always)]
	pub fn arm(&self) -> bool
	{
		panic_on_error_with_clean_up!
		(
			status,
			{
				match status
				{
					ucs_status_t_UCS_ERR_BUSY  => return true,
					_ => (),
				};
			},
			ucp_worker_arm,
			self.handle
		);
		false
	}
	
	/// Wakes up a worker waiting in blockingWaitForAnyEvent(), blockingWaitForAnyEvent() or on epoll
	#[inline(always)]
	pub fn signal(&self)
	{
		panic_on_error!(ucp_worker_signal, self.handle);
	}
}
