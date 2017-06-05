// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#![feature(associated_consts)]


#![allow(missing_copy_implementations)]
#![allow(missing_debug_implementations)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![warn(trivial_casts)]
#![warn(unused_import_braces)]


extern crate libc;


//use ::libc::c_int;

/*
	uct_dc_iface_ops is a child of uct_rc_iface_ops_t which is uct_rc_iface_ops
	which is a child of uct_ib_iface_ops_t which is uct_ib_iface_ops
	which is a child of uct_iface_ops_t which is uct_iface_ops which is defined in tl.h

	Initialised using UCS_CLASS_INIT_FUNC(uct_dc_iface_t, uct_dc_iface_ops_t *ops, uct_md_h md,
                    uct_worker_h worker, const uct_iface_params_t *params,
                    unsigned rx_priv_len, uct_dc_iface_config_t *config) in dc_iface.c
	
	Static field static uct_dc_iface_ops_t uct_dc_mlx5_iface_ops = { in dc_mlx5.c  gives an example of how it is initialised

typedef struct uct_iface_ops {

    void         (*iface_close)(uct_iface_h iface);

    ucs_status_t (*iface_query)(uct_iface_h iface,
                                uct_iface_attr_t *iface_attr);

    ucs_status_t (*iface_flush)(uct_iface_h iface, unsigned flags,
                                uct_completion_t *comp);

    ucs_status_t (*iface_fence)(uct_iface_h iface, unsigned flags);

    ucs_status_t (*iface_wakeup_open)(uct_iface_h iface, unsigned events,
                                     uct_wakeup_h wakeup);

    ucs_status_t (*iface_wakeup_get_fd)(uct_wakeup_h wakeup, int *fd_p);

    ucs_status_t (*iface_wakeup_arm)(uct_wakeup_h wakeup);

    ucs_status_t (*iface_wakeup_wait)(uct_wakeup_h wakeup);

    ucs_status_t (*iface_wakeup_signal)(uct_wakeup_h wakeup);

    void         (*iface_wakeup_close)(uct_wakeup_h wakeup);

    ucs_status_t (*iface_tag_recv_zcopy)(uct_iface_h iface, uct_tag_t tag,
                                         uct_tag_t tag_mask,
                                         const uct_iov_t *iov,
                                         size_t iovcnt,
                                         uct_tag_context_t *ctx);

    ucs_status_t (*iface_tag_recv_cancel)(uct_iface_h iface,
                                          uct_tag_context_t *ctx,
                                          int force);


    /* Connection establishment */

    ucs_status_t (*ep_create)(uct_iface_h iface, uct_ep_h *ep_p);

    ucs_status_t (*ep_create_connected)(uct_iface_h iface,
                                        const uct_device_addr_t *dev_addr,
                                        const uct_iface_addr_t *iface_addr,
                                        uct_ep_h* ep_p);

    void         (*ep_destroy)(uct_ep_h ep);

    ucs_status_t (*ep_get_address)(uct_ep_h ep, uct_ep_addr_t *addr);

    ucs_status_t (*ep_connect_to_ep)(uct_ep_h ep,
                                     const uct_device_addr_t *dev_addr,
                                     const uct_ep_addr_t *ep_addr);

    ucs_status_t (*iface_get_device_address)(uct_iface_h iface,
                                             uct_device_addr_t *addr);

    ucs_status_t (*iface_get_address)(uct_iface_h iface, uct_iface_addr_t *addr);

    int          (*iface_is_reachable)(const uct_iface_h iface,
                                       const uct_device_addr_t *dev_addr,
                                       const uct_iface_addr_t *iface_addr);

    /* Put */

    ucs_status_t (*ep_put_short)(uct_ep_h ep, const void *buffer, unsigned length,
                                 uint64_t remote_addr, uct_rkey_t rkey);

    ssize_t      (*ep_put_bcopy)(uct_ep_h ep, uct_pack_callback_t pack_cb,
                                 void *arg, uint64_t remote_addr, uct_rkey_t rkey);

    ucs_status_t (*ep_put_zcopy)(uct_ep_h ep, const uct_iov_t *iov, size_t iovcnt,
                                 uint64_t remote_addr, uct_rkey_t rkey,
                                 uct_completion_t *comp);

    /* Get */

    ucs_status_t (*ep_get_bcopy)(uct_ep_h ep, uct_unpack_callback_t unpack_cb,
                                 void *arg, size_t length,
                                 uint64_t remote_addr, uct_rkey_t rkey,
                                 uct_completion_t *comp);

    ucs_status_t (*ep_get_zcopy)(uct_ep_h ep, const uct_iov_t *iov, size_t iovcnt,
                                 uint64_t remote_addr, uct_rkey_t rkey,
                                 uct_completion_t *comp);

    /* Active message */

    ucs_status_t (*ep_am_short)(uct_ep_h ep, uint8_t id, uint64_t header,
                                const void *payload, unsigned length);

    ssize_t      (*ep_am_bcopy)(uct_ep_h ep, uint8_t id,
                                uct_pack_callback_t pack_cb, void *arg);

    ucs_status_t (*ep_am_zcopy)(uct_ep_h ep, uint8_t id, const void *header,
                                unsigned header_length, const uct_iov_t *iov,
                                size_t iovcnt, uct_completion_t *comp);

    /* Atomics */

    ucs_status_t (*ep_atomic_add64)(uct_ep_h ep, uint64_t add,
                                    uint64_t remote_addr, uct_rkey_t rkey);

    ucs_status_t (*ep_atomic_fadd64)(uct_ep_h ep, uint64_t add,
                                     uint64_t remote_addr, uct_rkey_t rkey,
                                     uint64_t *result, uct_completion_t *comp);

    ucs_status_t (*ep_atomic_swap64)(uct_ep_h ep, uint64_t swap,
                                     uint64_t remote_addr, uct_rkey_t rkey,
                                     uint64_t *result, uct_completion_t *comp);

    ucs_status_t (*ep_atomic_cswap64)(uct_ep_h ep, uint64_t compare, uint64_t swap,
                                      uint64_t remote_addr, uct_rkey_t rkey,
                                      uint64_t *result, uct_completion_t *comp);

    ucs_status_t (*ep_atomic_add32)(uct_ep_h ep, uint32_t add,
                                    uint64_t remote_addr, uct_rkey_t rkey);

    ucs_status_t (*ep_atomic_fadd32)(uct_ep_h ep, uint32_t add,
                                     uint64_t remote_addr, uct_rkey_t rkey,
                                     uint32_t *result, uct_completion_t *comp);

    ucs_status_t (*ep_atomic_swap32)(uct_ep_h ep, uint32_t swap,
                                     uint64_t remote_addr, uct_rkey_t rkey,
                                     uint32_t *result, uct_completion_t *comp);

    ucs_status_t (*ep_atomic_cswap32)(uct_ep_h ep, uint32_t compare, uint32_t swap,
                                      uint64_t remote_addr, uct_rkey_t rkey,
                                      uint32_t *result, uct_completion_t *comp);

    /* Pending queue */

    ucs_status_t (*ep_pending_add)(uct_ep_h ep, uct_pending_req_t *n);

    void         (*ep_pending_purge)(uct_ep_h ep, uct_pending_purge_callback_t cb,
                                     void * arg);

    /* TODO purge per iface */

    /* Synchronization */

    ucs_status_t (*ep_flush)(uct_ep_h ep, unsigned flags,
                             uct_completion_t *comp);

    ucs_status_t (*ep_fence)(uct_ep_h ep, unsigned flags);

    ucs_status_t (*ep_check)(uct_ep_h ep, unsigned flags, uct_completion_t *comp);

    /* Tagged operations */

    ucs_status_t (*ep_tag_eager_short)(uct_ep_h ep, uct_tag_t tag,
                                       const void *data, size_t length);

    ssize_t      (*ep_tag_eager_bcopy)(uct_ep_h ep, uct_tag_t tag, uint64_t imm,
                                       uct_pack_callback_t pack_cb, void *arg);

    ucs_status_t (*ep_tag_eager_zcopy)(uct_ep_h ep, uct_tag_t tag, uint64_t imm,
                                       const uct_iov_t *iov, size_t iovcnt,
                                       uct_completion_t *comp);

    ucs_status_ptr_t (*ep_tag_rndv_zcopy)(uct_ep_h ep, uct_tag_t tag,
                                          const void *header,
                                          unsigned header_length,
                                          const uct_iov_t *iov,
                                          size_t iovcnt,
                                          uct_completion_t *comp);

    ucs_status_t (*ep_tag_rndv_cancel)(uct_ep_h ep, void *op);

    ucs_status_t (*ep_tag_rndv_request)(uct_ep_h ep, uct_tag_t tag,
                                        const void* header,
                                        unsigned header_length);
} uct_iface_ops_t;


// ib_iface.h

struct uct_ib_iface_ops {
    uct_iface_ops_t         super;
    ucs_status_t            (*arm_tx_cq)(uct_ib_iface_t *iface);
    ucs_status_t            (*arm_rx_cq)(uct_ib_iface_t *iface, int solicited);
    void                    (*handle_failure)(uct_ib_iface_t *iface, void *arg);
    void                    (*set_ep_failed)(uct_ib_iface_t *iface, uct_ep_h ep);
};


// rc_iface.h

typedef struct uct_rc_iface_ops {
    uct_ib_iface_ops_t   super;
    ucs_status_t         (*fc_ctrl)(uct_ep_t *ep, unsigned op,
                                    uct_rc_fc_request_t *req);
    ucs_status_t         (*fc_handler)(uct_rc_iface_t *iface, unsigned qp_num,
                                       uct_rc_hdr_t *hdr, unsigned length,
                                       uint32_t imm_data, uint16_t lid,
                                       unsigned flags);
} uct_rc_iface_ops_t;


// dc_iface.h

typedef struct uct_dc_iface_ops {
    uct_rc_iface_ops_t            super;
    ucs_status_t                  (*reset_dci)(uct_dc_iface_t *iface, int dci);
} uct_dc_iface_ops_t;
*/

#[cfg(any(target_os="linux", target_os="android"))] include!("GlobalRoutingHeader.rs");
