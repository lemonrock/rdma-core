// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait CommunicationEventHandler
{
	#[inline(always)]
	fn addressResolutionSucceeded(&self)
	{
	}
	
	#[inline(always)]
	fn addressResolutionFailed(&self)
	{
	}
	
	#[inline(always)]
	fn routeResolutionSucceeded(&self)
	{
	}
	
	#[inline(always)]
	fn routeResolutionFailed(&self)
	{
	}
	
	#[inline(always)]
	fn connectionRequest(&self)
	{
	}
	
	#[inline(always)]
	fn connectionResponse(&self)
	{
	}
	
	#[inline(always)]
	fn connectionError(&self)
	{
	}
	
	#[inline(always)]
	fn unreachable(&self)
	{
	}
	
	#[inline(always)]
	fn rejected(&self)
	{
	}
	
	#[inline(always)]
	fn established(&self)
	{
	}
	
	#[inline(always)]
	fn disconnected(&self)
	{
	}
	
	#[inline(always)]
	fn deviceRemoval(&self)
	{
	}
	
	#[inline(always)]
	fn multicastJoin(&self)
	{
	}
	
	#[inline(always)]
	fn multicastError(&self)
	{
	}
	
	#[inline(always)]
	fn addressChange(&self)
	{
	}
	
	#[inline(always)]
	fn timeWaitExit(&self)
	{
	}
}
