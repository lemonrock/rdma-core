// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash)]
pub union ib_addr__bindgen_ty_1
{
	pub uib_addr8: [__u8; 16usize],
	pub uib_addr16: [__be16; 8usize],
	pub uib_addr32: [__be32; 4usize],
	pub uib_addr64: [__be64; 2usize],
	_bindgen_union_align: [u64; 2usize],
	pub _address: u8,
}
