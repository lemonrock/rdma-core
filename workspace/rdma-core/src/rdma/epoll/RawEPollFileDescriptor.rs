// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait RawEPollFileDescriptor: Sized
{
	#[inline(always)]
	fn epoll_create1(closeOnExec: bool, nonBlocking: bool) -> Self;
	
	/// self is also known as epfd
	/// fd2 is also known as fd
	/// evn is also known as event, and must be null if op is EPOLL_CTL_DEL and Linux version is greater than 2.6.9. This kernel version is ancient.
	#[inline(always)]
	fn epoll_ctl(self, op: c_int, fd2: c_int, ev: *mut epoll_event) -> c_int;
	
	/// self is also known as epfd
	/// ev is also known as events
	/// cnt is also known as maxevents
	/// to is also known as timeout
	/// The to argument specifies the minimum number of milliseconds that epoll_wait() will block. (This interval will be rounded up to the system clock granularity, and kernel scheduling delays mean that the blocking interval may overrun by a small amount.) Specifying a timeout of -1 causes epoll_wait() to block indefinitely, while specifying a timeout equal to zero cause epoll_wait() to return immediately, even if no events are available
	/// The cnt arguments must be greater than zero
	#[inline(always)]
	fn epoll_wait(self, ev: *mut epoll_event, cnt: c_int, to: c_int) -> u31;
	
	/// self is also known as epfd
	/// ev is also known as events
	/// cnt is also known as maxevents
	/// to is also known as timeout
	/// sigs is also known as sigmask; it may also be null()
	/// The to argument specifies the minimum number of milliseconds that epoll_wait() will block. (This interval will be rounded up to the system clock granularity, and kernel scheduling delays mean that the blocking interval may overrun by a small amount.) Specifying a timeout of -1 causes epoll_wait() to block indefinitely, while specifying a timeout equal to zero cause epoll_wait() to return immediately, even if no events are available
	/// The cnt arguments must be greater than zero
	/// When sigs is null() then this function is the same as `wait()`
	#[inline(always)]
	fn epoll_pwait(self, ev: *mut epoll_event, cnt: c_int, to: c_int, sigs: *const sigset_t) -> u31;
}

impl RawEPollFileDescriptor for RawFd
{
	#[doc(hidden)]
	#[inline(always)]
	fn epoll_create1(closeOnExec: bool, nonBlocking: bool) -> Self
	{
		let mut flags = if unlikely(closeOnExec)
		{
			EPOLL_CLOEXEC as c_int
		}
		else
		{
			0
		};
		if unlikely(nonBlocking)
		{
			flags |= EPOLL_NONBLOCK as c_int;
		}
		
		let result = unsafe { Syscall::epoll_create1.syscall1(flags as SyscallArgument) };
		if likely(result >= 0)
		{
			return result as Self;
		}
		match result as PosixErrorNumber
		{
			NegativeE::ENOSYS => panic!("This is an ancient version of Linux that doesn't support SYS_epoll_create1 (ie older than 2.6.27)"),
			NegativeE::EINVAL => panic!("Flags are invalid"),
			NegativeE::EMFILE => panic!("The per-user limit on the number of epoll instances imposed by
              /proc/sys/fs/epoll/max_user_instances was encountered or the per-process limit on the number of open file descriptors
              has been reached"),
			NegativeE::ENFILE => panic!("The system-wide limit on the total number of open files has
              been reached"),
			NegativeE::ENOMEM => panic!("There was insufficient memory to create the kernel object"),
			
			_ => panic!("Invalid negative errno '{}'", result),
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn epoll_ctl(self, op: c_int, fd2: c_int, ev: *mut epoll_event) -> c_int
	{
		debug_assert!(self != fd2, "self can not be the same as fd2");
		
		(unsafe { Syscall::epoll_ctl.syscall4(self as SyscallArgument, op as SyscallArgument, fd2 as SyscallArgument, ev as SyscallArgument) }) as c_int
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn epoll_wait(self, ev: *mut epoll_event, cnt: c_int, to: c_int) -> u31
	{
		return self.epoll_pwait(ev, cnt, to, null());
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn epoll_pwait(self, ev: *mut epoll_event, cnt: c_int, to: c_int, sigs: *const sigset_t) -> u31
	{
		debug_assert!(!ev.is_null(), "ev can not be null");
		debug_assert!(cnt > 0, "cnt may not be zero or negative");
		debug_assert!(to > -1, "to may not be less than -1");
		
		let result = unsafe { Syscall::epoll_pwait.syscall6(self as SyscallArgument, ev as SyscallArgument, cnt as SyscallArgument, to as SyscallArgument, sigs as SyscallArgument, (_NSIG / 8) as SyscallArgument) };
		
		if likely(result >= 0)
		{
			return result as u31;
		}
		match result as PosixErrorNumber
		{
			NegativeE::ENOSYS => panic!("This is an ancient version of Linux that doesn't support SYS_epoll_pwait (ie older than 2.6.19)"),
			NegativeE::EBADF => panic!("self is not an epoll file descriptor"),
			NegativeE::EFAULT => panic!("The memory area pointed to by ev is not accessible with write permissions"),
			NegativeE::EINTR => panic!("The call was interrupted by a signal handler before either any of the requested events occurred or the timeout expired"),
			NegativeE::EINVAL => panic!("self is not an epoll file descriptor, or cnt is less than or equal to zero"),
			
			_ => panic!("Invalid negative errno '{}'", result),
		}
	}
}
