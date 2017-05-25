// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait EPollFileDescriptor: RawEPollFileDescriptor
{
	#[inline(always)]
	fn newEPollFileDescriptor(closeOnExec: bool, nonBlocking: bool) -> Self
	{
		Self::epoll_create1(closeOnExec, nonBlocking)
	}
	
	#[inline(always)]
	fn add(self, fileDescriptor: RawFd, data: &mut epoll_event)
	{
		let result = self.epoll_ctl(EPOLL_CTL_ADD as i32, fileDescriptor, data);
		
		if likely(result == 0)
		{
			return;
		}
		match result as PosixErrorNumber
		{
			NegativeE::ENOSPC => panic!("The limit imposed by /proc/sys/fs/epoll/max_user_watches was encountered while trying to register (EPOLL_CTL_ADD) a new file descriptor on an epoll instance"),
			NegativeE::EEXIST => panic!("op was EPOLL_CTL_ADD, and the supplied file descriptor fd is already registered with this epoll instance; this is possible if using the same fd across multiple threads. Not normally a wise design choice"),
			NegativeE::EBADF => panic!("self is not an epoll file descriptor or fileDescriptor is not a file descriptor"),
			NegativeE::EINVAL => panic!("self is not an epoll file descriptor, or fileDescriptor is the same as self, or the requested operation EPOLL_CTL_ADD is not supported by this interface"),
			NegativeE::ENOMEM => panic!("There was insufficient memory to handle the requested op control operation"),
			NegativeE::EPERM => panic!("The target file fileDescriptor does not support epoll"),
			
			_ => panic!("Invalid negative errno '{}'", result),
		}
	}
	
	#[inline(always)]
	fn modify(self, fileDescriptor: RawFd, data: &mut epoll_event)
	{
		let result = self.epoll_ctl(EPOLL_CTL_MOD as i32, fileDescriptor, data);
		
		if likely(result == 0)
		{
			return;
		}
		match result as PosixErrorNumber
		{
			NegativeE::EBADF => panic!("self is not an epoll file descriptor or fileDescriptor is not a file descriptor"),
			NegativeE::EINVAL => panic!("self is not an epoll file descriptor, or fileDescriptor is the same as self, or the requested operation EPOLL_CTL_MOD is not supported by this interface"),
			NegativeE::ENOENT => panic!("fd2 is not registered with this epoll instance"),
			NegativeE::ENOMEM => panic!("There was insufficient memory to handle the requested op control operation"),
			NegativeE::EPERM => panic!("The target file fileDescriptor does not support epoll"),
			
			_ => panic!("Invalid negative errno '{}'", result),
		}
	}
	
	#[inline(always)]
	fn delete(self, fileDescriptor: RawFd)
	{
		let result = self.epoll_ctl(EPOLL_CTL_DEL as i32, fileDescriptor, null_mut());
		
		if likely(result == 0)
		{
			return;
		}
		match result as PosixErrorNumber
		{
			NegativeE::EBADF => panic!("self is not an epoll file descriptor or fileDescriptor is not a file descriptor"),
			NegativeE::EINVAL => panic!("self is not an epoll file descriptor, or fileDescriptor is the same as self, or the requested operation EPOLL_CTL_DEL is not supported by this interface"),
			NegativeE::ENOENT => panic!("op was EPOLL_CTL_MOD or EPOLL_CTL_DEL, and fileDescriptor is not registered with this epoll instance"),
			NegativeE::ENOMEM => panic!("There was insufficient memory to handle the requested op control operation"),
			NegativeE::EPERM => panic!("The target file fileDescriptor does not support epoll"),
			
			_ => panic!("Invalid negative errno '{}'", result),
		}
	}
	
	/// It is recommended that `into` is empty
	#[inline(always)]
	fn waitIndefinitely(self, into: &mut ArrayVec<[epoll_event; MaximumEvents]>) -> usize
	{
		self.wait(into, -1)
	}
	
	/// It is recommended that `into` is empty
	#[inline(always)]
	fn areThereAnyEvents(self, into: &mut ArrayVec<[epoll_event; MaximumEvents]>) -> usize
	{
		self.wait(into, 0)
	}
	
	/// It is recommended that `into` is empty
	#[inline(always)]
	fn waitUntilTimeOut(self, into: &mut ArrayVec<[epoll_event; MaximumEvents]>, timeOutInMilliseconds: u31) -> usize
	{
		self.wait(into, timeOutInMilliseconds as c_int)
	}
	
	/// It is recommended that `into` is empty
	#[doc(hidden)]
	#[inline(always)]
	fn wait(self, into: &mut ArrayVec<[epoll_event; MaximumEvents]>, timeOutInMilliseconds: c_int) -> usize
	{
		let length = into.len();
		
		let space = MaximumEvents - length;
		
		if unlikely(space == 0)
		{
			return 0;
		}
		
		let fillInto = unsafe { transmute(into.as_mut_ptr().offset(length as isize)) };
		
		self.epoll_wait(fillInto, space as c_int, timeOutInMilliseconds) as usize
	}
}

impl EPollFileDescriptor for RawFd
{
}
