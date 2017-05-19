// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ibv_pdEx
{
	#[inline(always)]
	fn createWorkQueue(self, context: *mut c_void, requestedSettings: SharedReceiveQueueSettings, cvLanStripping: bool, fcsFieldWillBeScatteredToHostMemory: bool, completionQueue: *mut ibv_cq) -> (*mut ibv_wq, SharedReceiveQueueSettings);
	
	#[inline(always)]
	fn createUnextendedSharedReceiveQueue(self, context: *mut c_void, requestedSettings: SharedReceiveQueueSettings) -> (*mut ibv_srq, SharedReceiveQueueSettings);
	
	#[inline(always)]
	fn createExtendedSharedReceiveQueue(self, context: *mut c_void, requestedSettings: SharedReceiveQueueSettings, extendedReliableConnectionDomain: *mut ibv_xrcd, completionQueue: *mut ibv_cq) -> (*mut ibv_srq, SharedReceiveQueueSettings);
	
	#[inline(always)]
	fn populateAddressHandleForPortAttributes(self, portNumber: u8, workCompletion: *mut ibv_wc, globalRoutingHeader: *mut ibv_grh) -> ibv_ah_attr;
	
	/// It seems in practice that the attributes are not mutated
	#[inline(always)]
	fn createAddressHandle(self, attributes: &mut ibv_ah_attr) -> *mut ibv_ah;
	
	#[inline(always)]
	fn createAddressHandleForPort(self, portNumber: u8, workCompletion: *mut ibv_wc, globalRoutingHeader: *mut ibv_grh) -> *mut ibv_ah;
}

impl ibv_pdEx for *mut ibv_pd
{
	#[inline(always)]
	fn createWorkQueue(self, context: *mut c_void, requestedSettings: SharedReceiveQueueSettings, cvLanStripping: bool, fcsFieldWillBeScatteredToHostMemory: bool, completionQueue: *mut ibv_cq) -> (*mut ibv_wq, SharedReceiveQueueSettings)
	{
		debug_assert!(!self.is_null(), "self is null");
		debug_assert!(!completionQueue.is_null(), "completionQueue is null");
		
		const IBV_WQ_INIT_ATTR_FLAGS: u32 = 1;
		
		// 'ibv_wq_flags'
		let mut creationFlags = 0;
		if cvLanStripping
		{
			const IBV_WQ_FLAGS_CVLAN_STRIPPING: u32 = 1 << 0;
			creationFlags |= IBV_WQ_FLAGS_CVLAN_STRIPPING;
		}
		if fcsFieldWillBeScatteredToHostMemory
		{
			const IBV_WQ_FLAGS_SCATTER_FCS: u32 = 1 << 1;
			creationFlags |= IBV_WQ_FLAGS_SCATTER_FCS;
		}
		
		let mut attributes = ibv_wq_init_attr
		{
			wq_context: context,
			wq_type: ibv_wq_type::IBV_WQT_RQ,
			max_wr: requestedSettings.maximumNumberOfOutstandingWorkRequestsInInTheSharedRequestQueue,
			max_sge: requestedSettings.maximumNumberOfScatterElementsPerWorkRequest,
			pd: self,
			cq: completionQueue,
			comp_mask: IBV_WQ_INIT_ATTR_FLAGS,
			create_flags: creationFlags,
		};
		
		let workQueue = panic_on_null!(rust_ibv_create_wq, self.verbs(), &mut attributes);
		let settings = SharedReceiveQueueSettings
		{
			maximumNumberOfOutstandingWorkRequestsInInTheSharedRequestQueue: attributes.max_wr,
			maximumNumberOfScatterElementsPerWorkRequest: attributes.max_sge,
		};
		(workQueue, settings)
	}
	
	#[inline(always)]
	fn createUnextendedSharedReceiveQueue(self, context: *mut c_void, requestedSettings: SharedReceiveQueueSettings) -> (*mut ibv_srq, SharedReceiveQueueSettings)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		let mut attributes = ibv_srq_init_attr
		{
			srq_context: context,
			attr: ibv_srq_attr
			{
				max_wr: requestedSettings.maximumNumberOfOutstandingWorkRequestsInInTheSharedRequestQueue,
				max_sge: requestedSettings.maximumNumberOfScatterElementsPerWorkRequest,
				srq_limit: 0,
			}
		};
		
		let unextendedSharedReceiveQueue = panic_on_null!(ibv_create_srq, self, &mut attributes);
		let settings = SharedReceiveQueueSettings
		{
			maximumNumberOfOutstandingWorkRequestsInInTheSharedRequestQueue: attributes.attr.max_wr,
			maximumNumberOfScatterElementsPerWorkRequest: attributes.attr.max_sge,
		};
		(unextendedSharedReceiveQueue, settings)
	}
	
	#[inline(always)]
	fn createExtendedSharedReceiveQueue(self, context: *mut c_void, requestedSettings: SharedReceiveQueueSettings, extendedReliableConnectionDomain: *mut ibv_xrcd, completionQueue: *mut ibv_cq) -> (*mut ibv_srq, SharedReceiveQueueSettings)
	{
		debug_assert!(!self.is_null(), "self is null");
		debug_assert!(!extendedReliableConnectionDomain.is_null(), "extendedReliableConnectionDomain is null");
		debug_assert!(!completionQueue.is_null(), "completionQueue is null");
		
		const IBV_SRQ_INIT_ATTR_TYPE: u32 = 1;
		const IBV_SRQ_INIT_ATTR_PD: u32 = 2;
		const IBV_SRQ_INIT_ATTR_XRCD: u32 = 4;
		const IBV_SRQ_INIT_ATTR_CQ: u32 = 8;
		
		const AllCurrentFields: u32 = IBV_SRQ_INIT_ATTR_TYPE | IBV_SRQ_INIT_ATTR_PD | IBV_SRQ_INIT_ATTR_XRCD | IBV_SRQ_INIT_ATTR_CQ;
		
		let mut attributes = ibv_srq_init_attr_ex
		{
			srq_context: context,
			attr: ibv_srq_attr
			{
				max_wr: requestedSettings.maximumNumberOfOutstandingWorkRequestsInInTheSharedRequestQueue,
				max_sge: requestedSettings.maximumNumberOfScatterElementsPerWorkRequest,
				srq_limit: 0,
			},
			comp_mask: AllCurrentFields,
			srq_type: ibv_srq_type::IBV_SRQT_XRC,
			pd: self,
			xrcd: extendedReliableConnectionDomain,
			cq: completionQueue,
		};
		
		let extendedSharedReceiveQueue = panic_on_null!(rust_ibv_create_srq_ex, self.verbs(), &mut attributes);
		let settings = SharedReceiveQueueSettings
		{
			maximumNumberOfOutstandingWorkRequestsInInTheSharedRequestQueue: attributes.attr.max_wr,
			maximumNumberOfScatterElementsPerWorkRequest: attributes.attr.max_sge,
		};
		(extendedSharedReceiveQueue, settings)
	}
	
	#[inline(always)]
	fn populateAddressHandleForPortAttributes(self, portNumber: u8, workCompletion: *mut ibv_wc, globalRoutingHeader: *mut ibv_grh) -> ibv_ah_attr
	{
		debug_assert!(!self.is_null(), "self is null");
		debug_assert!(!workCompletion.is_null(), "workCompletion is null");
		debug_assert!(!globalRoutingHeader.is_null(), "globalRoutingHeader is null");
		debug_assert!(portNumber < self.verbs().attributes().numberOfPhysicalPorts(), "port number '{}' is not less than the maximum '{}'", portNumber, self.verbs().attributes().numberOfPhysicalPorts());
		
		let mut attributes = unsafe { uninitialized() };
		panic_on_error!(ibv_init_ah_from_wc, self.verbs(), portNumber, workCompletion, globalRoutingHeader, &mut attributes);
		attributes
	}
	
	#[inline(always)]
	fn createAddressHandle(self, attributes: &mut ibv_ah_attr) -> *mut ibv_ah
	{
		debug_assert!(!self.is_null(), "self is null");
		debug_assert!(attributes.port_num < self.verbs().attributes().numberOfPhysicalPorts(), "port number '{}' is not less than the maximum '{}'", attributes.port_num, self.verbs().attributes().numberOfPhysicalPorts());
		
		panic_on_null!(ibv_create_ah, self, attributes)
	}
	
	#[inline(always)]
	fn createAddressHandleForPort(self, portNumber: u8, workCompletion: *mut ibv_wc, globalRoutingHeader: *mut ibv_grh) -> *mut ibv_ah
	{
		debug_assert!(!self.is_null(), "self is null");
		debug_assert!(!workCompletion.is_null(), "workCompletion is null");
		debug_assert!(!globalRoutingHeader.is_null(), "globalRoutingHeader is null");
		debug_assert!(portNumber < self.verbs().attributes().numberOfPhysicalPorts(), "port number '{}' is not less than the maximum '{}'", portNumber, self.verbs().attributes().numberOfPhysicalPorts());
		
		panic_on_null!(ibv_create_ah_from_wc, self, workCompletion, globalRoutingHeader, portNumber)
	}
}
