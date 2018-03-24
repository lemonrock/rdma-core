// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_cq_ex
{
	pub context: *mut ibv_context,
	pub channel: *mut ibv_comp_channel,
	pub cq_context: *mut c_void,
	pub handle: u32,
	pub cqe: c_int,
	pub mutex: pthread_mutex_t,
	pub cond: pthread_cond_t,
	pub comp_events_completed: u32,
	pub async_events_completed: u32,
	pub comp_mask: u32,
	pub status: ibv_wc_status,
	pub wr_id: u64,
	pub start_poll: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex, attr: *mut ibv_poll_cq_attr) -> c_int>,
	pub next_poll: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex) -> c_int>,
	pub end_poll: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex)>,
	pub read_opcode: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex) -> ibv_wc_opcode>,
	pub read_vendor_err: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex) -> u32>,
	pub read_byte_len: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex) -> u32>,
	pub read_imm_data: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex) -> __be32>,
	pub read_qp_num: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex) -> u32>,
	pub read_src_qp: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex) -> u32>,
	pub read_wc_flags: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex) -> c_uint>,
	pub read_slid: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex) -> u32>,
	pub read_sl: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex) -> u8>,
	pub read_dlid_path_bits: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex) -> u8>,
	pub read_completion_ts: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex) -> u64>,
	pub read_cvlan: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex) -> u16>,
	pub read_flow_tag: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex) -> u32>,
	pub read_tm_info: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex, tm_info: *mut ibv_wc_tm_info)>,
	pub read_completion_wallclock_ns: Option<unsafe extern "C" fn(current: *mut ibv_cq_ex) -> u64>,
}

impl Default for ibv_cq_ex
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_cq_ex
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_cq_ex {{ context: {:?}, channel: {:?}, cq_context: {:?}, status: {:?}, start_poll: {:?}, next_poll: {:?}, end_poll: {:?}, read_opcode: {:?}, read_vendor_err: {:?}, read_byte_len: {:?}, read_imm_data: {:?}, read_qp_num: {:?}, read_src_qp: {:?}, read_wc_flags: {:?}, read_slid: {:?}, read_sl: {:?}, read_dlid_path_bits: {:?}, read_completion_ts: {:?}, read_cvlan: {:?}, read_flow_tag: {:?}, read_tm_info: {:?}, read_completion_wallclock_ns: {:?} }}", self.context, self.channel, self.cq_context, self.status, self.start_poll, self.next_poll, self.end_poll, self.read_opcode, self.read_vendor_err, self.read_byte_len, self.read_imm_data, self.read_qp_num, self.read_src_qp, self.read_wc_flags, self.read_slid, self.read_sl, self.read_dlid_path_bits, self.read_completion_ts, self.read_cvlan, self.read_flow_tag, self.read_tm_info, self.read_completion_wallclock_ns)
	}
}
