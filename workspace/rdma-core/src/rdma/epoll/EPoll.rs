// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct EPoll<EPC: EPollContext>
{
	epollFileDescriptor: RawFd,
	phantomData: PhantomData<EPC>,
}

impl<EPC: EPollContext> Drop for EPoll<EPC>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_error!(close, self.epollFileDescriptor);
	}
}

impl<EPC: EPollContext> EPoll<EPC>
{
	#[inline(always)]
	pub fn new() -> Self
	{
		Self
		{
			epollFileDescriptor: RawFd::newEPollFileDescriptor(true, false),
			phantomData: PhantomData,
		}
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn registerEdgeTriggeredIn(&self, registrant: EPC) -> Box<EPC>
	{
		let fileDescriptorForEPoll = registrant.fileDescriptorForEPoll();
		let mut context = Box::new(registrant);
		
		let mut data = epoll_event
		{
			events: (EPollEvents::EdgeTriggered | EPollEvents::In).bits(),
			data: epoll_data_t
			{
				ptr: context.as_mut() as *mut _ as *mut _
			}
		};
		fileDescriptorForEPoll.makeNonBlocking();
		self.epollFileDescriptor.add(fileDescriptorForEPoll, &mut data);
		
		context
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	pub fn processAnyEventsWithoutWaitingUntilExhausted(&self)
	{
		loop
		{
			let mut into: [epoll_event; MaximumEvents] = unsafe { uninitialized() };
			
			loop
			{
				const DoNotWaitTimeOut: i32 = 0;
				
				let size = self.epollFileDescriptor.epoll_wait(into.as_mut_ptr() as *mut _ as *mut _, MaximumEvents as i32, DoNotWaitTimeOut) as usize;
				if unlikely(size == 0)
				{
					break;
				}
				else
				{
					debug_assert!(size <= MaximumEvents, "Overfilled; defect in epoll_wait()");
					let mut index = 0;
					while index < size
					{
						let epollEvent = unsafe { replace(into.get_unchecked_mut(index), uninitialized()) };
						if likely(epollEvent.events & EPOLLIN == EPOLLIN)
						{
							let context = unsafe { epollEvent.data.ptr };
							let mut epollContext = unsafe { Box::from_raw(context as *mut EPC) };
							epollContext.processEvents();
						}
						
						// TODO: What about EPOLLERR?
						
						index += 1;
					}
					
					if likely(size != MaximumEvents)
					{
						break;
					}
				}
			}
			
			forget(into);
		}
	}
	
	
	#[inline(always)]
	pub fn deregister(&self, registrant: RawFd)
	{
		self.epollFileDescriptor.delete(registrant);
	}
}
