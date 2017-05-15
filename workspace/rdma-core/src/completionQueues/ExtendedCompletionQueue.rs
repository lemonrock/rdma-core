// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct ExtendedCompletionQueue<'a>
{
	pointer: *mut ibv_cq_ex,
	lifetime: Option<&'a CompletionChannel<'a>>,
}

impl<'a> Drop for ExtendedCompletionQueue<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let pointer = self.pointer();
		panic_on_errno!(ibv_destroy_cq, pointer);
	}
}

impl<'a> CompletionQueue<'a> for ExtendedCompletionQueue<'a>
{
	#[doc(hidden)]
	#[inline(always)]
	fn pointer(&self) -> *mut ibv_cq
	{
		unsafe { rust_ibv_cq_ex_to_cq(self.pointer) }
	}
}

impl<'a> ExtendedCompletionQueue<'a>
{
	#[inline(always)]
	pub(crate) fn new(pointer: *mut ibv_cq_ex, lifetime: Option<&'a CompletionChannel>) -> Self
	{
		debug_assert!(!pointer.is_null(), "pointer is null");
		
		Self
		{
			pointer: pointer,
			lifetime: lifetime,
		}
	}
	
	/*
		pub fn rust_ibv_end_poll(cq: *mut ibv_cq_ex);
		pub fn rust_ibv_next_poll(cq: *mut ibv_cq_ex) -> c_int
		pub fn rust_ibv_start_poll(cq: *mut ibv_cq_ex, attr: *mut ibv_poll_cq_attr) -> c_int;
		
		pub fn rust_ibv_wc_read_byte_len(cq: *mut ibv_cq_ex) -> u32;
		pub fn rust_ibv_wc_read_completion_ts(cq: *mut ibv_cq_ex) -> u64;
		pub fn rust_ibv_wc_read_cvlan(cq: *mut ibv_cq_ex) -> u16;
		pub fn rust_ibv_wc_read_dlid_path_bits(cq: *mut ibv_cq_ex) -> u8;
		pub fn rust_ibv_wc_read_flow_tag(cq: *mut ibv_cq_ex) -> u32;
		pub fn rust_ibv_wc_read_imm_data(cq: *mut ibv_cq_ex) -> u32;
		pub fn rust_ibv_wc_read_opcode(cq: *mut ibv_cq_ex) -> ibv_wc_opcode;
		pub fn rust_ibv_wc_read_qp_num(cq: *mut ibv_cq_ex) -> u32;
		pub fn rust_ibv_wc_read_sl(cq: *mut ibv_cq_ex) -> u8;
		pub fn rust_ibv_wc_read_slid(cq: *mut ibv_cq_ex) -> u32;
		pub fn rust_ibv_wc_read_src_qp(cq: *mut ibv_cq_ex) -> u32;
		pub fn rust_ibv_wc_read_vendor_err(cq: *mut ibv_cq_ex) -> u32;
		pub fn rust_ibv_wc_read_wc_flags(cq: *mut ibv_cq_ex) -> c_int;
	*/
	
	// See https://www.mankier.com/3/ibv_create_cq_ex for other methods
}
