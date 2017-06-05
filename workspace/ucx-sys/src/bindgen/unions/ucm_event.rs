// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub union ucm_event
{
    pub mmap: ucm_event__bindgen_ty_1,
    pub munmap: ucm_event__bindgen_ty_2,
    pub mremap: ucm_event__bindgen_ty_3,
    pub shmat: ucm_event__bindgen_ty_4,
    pub shmdt: ucm_event__bindgen_ty_5,
    pub sbrk: ucm_event__bindgen_ty_6,
    pub vm_mapped: ucm_event__bindgen_ty_7,
    pub vm_unmapped: ucm_event__bindgen_ty_7,
}
