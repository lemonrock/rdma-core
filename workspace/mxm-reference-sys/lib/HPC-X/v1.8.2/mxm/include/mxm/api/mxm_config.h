/*
* Copyright (C) Mellanox Technologies Ltd. 2001-2011.  ALL RIGHTS RESERVED.
* This software product is a proprietary product of Mellanox Technologies Ltd.
* (the "Company") and all right, title, and interest and to the software product,
* including all associated intellectual property rights, are and shall
* remain exclusively with the Company.
*
* This software product is governed by the End User License Agreement
* provided with the software product.
* $COPYRIGHT$
* $HEADER$
*/


#ifndef MXM_API_CONFIG_H_
#define MXM_API_CONFIG_H_

#include <mxm/api/mxm_def.h>

#include <stdio.h>

/*
 * NOTE:
 *  This file is not included in the basic MXM API, and may be subject to changes.
 *  In order to keep backward compatibility, avoid including this file.
 *
 * All time values are in seconds.
 */

BEGIN_C_DECLS

/**
 * MXM global options.
 */
struct mxm_global_opts {

    /* Log level above which log messages will be printed */
    mxm_log_level_t          log_level;

    /* Log file */
    char                     *log_file;

    /* Size of log buffer for one message */
    size_t                   log_buffer_size;

    /* Maximal amount of packet data to print per packet */
    size_t                   log_data_size;

    /* Handle errors mode */
    mxm_handle_error_t       handle_errors;

    /* Error signals */
    struct {
        int                  *signals;
        unsigned             count;
    } error_signals;

    /* If not NULL, attach gdb to the process in case of error */
    char                     *gdb_command;

    /* Signal number which causes MXM to enter debug mode */
    unsigned                 debug_signo;

    /* File name to dump instrumentation records to */
    char                     *instrument_file;

    /* Limit for instrumentation data size */
    size_t                   instrument_max_size;

    /* Asynchronous progress interval. Lower values may cause poor performance,
     * but better responsiveness.
     */
    double                   async_interval;

    /* Destination for statistics: udp:host:port / file:path / stdout
     */
    char                     *stats_dest;

    /* Trigger to dump statistics */
    char                     *stats_trigger;

    /* Named pipe file path for tuning.
     */
    char                     *tuning_path;

    /* Number of performance stall loops to perform */
    size_t                   perf_stall_loops;

    /* Signal number used by async handler (for signal mode) */
    unsigned                 async_signo;

    /* Destination for detailed memory tracking results: none / stdout / stderr
     */
    char                     *memtrack_dest;

};


/**
 * IB single port specification
 */
typedef struct mxm_ib_port_spec {
    char                     *device_name;
    unsigned                 port_num;
} mxm_ib_port_spec_t;


/**
 * IB multiple port specification
 */
typedef struct mxm_ib_ports_config {
    mxm_ib_port_spec_t       *specs;
    unsigned                 count;
} mxm_ib_port_configs_t;


/**
 * MXM context and PTL initialization options.
 */
struct mxm_context_opts {

    /*
     * Select asynchronous mode
     * Signals are less portable, but do not incur a locking overhead.
     */
    mxm_async_mode_t         async_mode;

    /*
     * Memory management options
     */
    struct {

        /* Allocators priority */
        struct {
            mxm_allocator_t  *allocs;
            unsigned         count;
        } allocators;

        /* Enable on-demand memory mapping. The user is responsible for invalidating
         * released memory using mxm_mem_unmap() */
        int                  on_demand_map;

        /* Use on-demand paging (when it's possible) to manage memory */
        int                  enable_odp;

        /* Don't do on demand mapping if num of mapped regions is equal or greater than this value */
        unsigned             max_mapped_regs;

    } mem;

    /* Path to hook script to execute during mxm_init */
    char                     *init_hook;

    /* Select the use of threads - multiple/single thread */
    int                      is_thread_single;

    /* Select the mode of the kernel copy - off/knem/autodetect */
    mxm_shm_kcopy_mode_t     shm_kcopy_mode;

    struct {

        /* Which physical ports to use */
        mxm_ib_port_configs_t ports;

        /* Fork safety support */
        int                  fork_safe;

        /* Fork safety support for hugetlb */
        int                  hugetlb_safe;

        /* Randomize VA of atomic operations for reduce lock contention */
        int                  hw_atomic_algo;

        /* ODP data size limitations */
        struct {
            size_t           prefetch_thresh;
        } odp;

    } ib;
};


/**
 * Common endpoint options for all transports
 */
typedef struct mxm_tl_ep_opts {

    /* Threshold for using rendezvous protocol */
    size_t                   rndv_thresh;

    /* Rendezvous protocol mode */
    mxm_rndv_mode_t          rndv_mode;

    /* Threshold for forcibly waking up remote peer for rendezvous messages */
    size_t                   rndv_wakeup_thresh;

    /* Threshold for checking using zero copy */
    size_t                   zcopy_thresh;

    /* Threshold for employing using zero copy */
    size_t                   hard_zcopy_thresh;

    /* Maximal segment size. The actual size used may be limited by HW */
    size_t                   mss;

    /* Timer to progress the transport asynchronously */
    double                   timer;

} mxm_tl_ep_opts_t;


/**
 * Loopback options
 */
typedef struct mxm_self_ep_opts {

    mxm_tl_ep_opts_t         tl;

} mxm_self_ep_opts_t;


/**
 * Shared memory options
 */
typedef struct mxm_shm_ep_opts {

    mxm_tl_ep_opts_t         tl;

    struct {
        unsigned             max_bufs;          /* RX inline sized buffers to allow*/
        unsigned             max_medium_bufs;   /* RX medium sized buffers to allow */
    } rx;

    /* Size of the endpoint's shmem fifo */
    unsigned                 fifo_size;

    unsigned                 write_retry_count;

    unsigned                 read_retry_count;

    unsigned                 knem_max_simultaneous;

    size_t                   dma_chunk_size;

    /* Select the use of knem - with or without the offload to the dma engine */
    int                      is_using_knem_dma;

    double                   release_fifo_factor;

    /** Enable using huge pages for shared memory buffers */
    mxm_ternary_value_t      hugetlb_mode;

} mxm_shm_ep_opts_t;


/*
 * Common IB options
 */
typedef struct mxm_ib_ep_opts {

    mxm_tl_ep_opts_t         tl;

    /* HCA port mapping (to process) mode */
    mxm_ib_map_mode_t        map_mode;

    /* Do progress till CQ is drained */
    int                      drain_cq;

    /* consider ep congested if poll cq returns more than n wqes */
    unsigned                 cq_wmark;

    /* Allow using resize_cq */
    int                      resize_cq;

    /* First service level number to use */
    int                      first_sl;

    /* How many service levels to use */
    unsigned                 num_sls;

    /* Interrupt mode flags */
    unsigned                 int_mode;

    /* Interrupt threshold (seconds) */
    double                   int_thresh;

    /* Use direct PRM access */
    mxm_ternary_value_t      exp_connectib;

    /* Write combining mode */
    unsigned                 wc_mode;

    /* Enable CQ stall */
    mxm_ternary_value_t      cq_stall;

    /* Stall loops for CPU workaround */
    unsigned                 cq_stall_loops;

    /* Array of IB Path bits which will be the low portion of the LID,
     * according to the LMC in the fabric.
     * It is also the offset to add to the base lid when using LMC (LID Mask Control)
     */
    struct {
        unsigned             *bits;
        unsigned             count;
    } lid_path;

    unsigned                 max_path_bits;

    mxm_ib_lid_path_policy_t lid_path_policy;

    /* RX options */
    struct {
        unsigned             queue_len;   /* RX queue length */
        unsigned             max_bufs;    /* Amount of RX buffers */
        size_t               max_inline;  /* Max inline receive */
        unsigned             max_batch;   /* How many buffers to post-receive at once*/
        unsigned             poll_batch;  /* How many completions to poll on at once*/
    } rx;

    /* TX options */
    struct {
        unsigned             queue_len;    /* TX queue length */
        unsigned             max_bufs;     /* Amount of TX buffers */
        size_t               max_inline;   /* Inline space to reserve */
        unsigned             num_sge;      /* How many SG entries to support */
        unsigned             cq_moderation;/* How many TX messages are batched to one CQE */
        unsigned             max_batch;    /* How many fragments can be batched to one post-send */
    } tx;

    /* Minimum number of element in rx/tx buffers mempool */
    int                      min_chunk;

    /* For ibv_query_gid call: Requested GID index, values can be [0..port_attr.gid_tbl_len-1]  */
    unsigned                 gid_index;

    /* Enable using global address */
    mxm_ternary_value_t      use_grh;

} mxm_ib_ep_opts_t;


/*
 * OOB options
 */
typedef struct mxm_oob_ep_opts {

    mxm_ib_ep_opts_t         ib;

    double                   ack_timeout;

} mxm_oob_ep_opts_t;


/*
 * UD options
 */
typedef struct mxm_ud_ep_opts {

    mxm_ib_ep_opts_t         ib;

    /* Timeout for getting acknowledgment from a remote peer */
    double                   ack_timeout;

    /* microsecond timeouts for fast ack and fast retransmit */
    double                   fast_ack_timeout;

    /* Fast timer resolution */
    double                   fast_timer_res;

    /* How many un-acknowledged packets may be on the fly */
    unsigned                 window_size;

    /* Congestion avoidance algorithm */
    mxm_ud_ca_t              ca;

    /* when switch from ai/md to bic */
    int                      ca_low_window;

    /* when last unacknowledged packet is smaller than this, send it as CHK */
    unsigned                 chk_max_size;

    /* fatal timeout */
    double                   timeout;

    /* RX options */
    struct {
        unsigned             drop_rate;   /* For how many packets, on average, one
                                             will be dropped on purpose */
        int                  ooo_pkts;    /* enable out of order receive */
    } rx;

    /* UD RNDV recv zcopy protocol options */
    struct {
        /* Timeout for getting window acknowledgment from a remote peer */
        double               win_timeout;

        /* Enable UD RNDV recv zcopy protocol */
        int                  enable;

        /* Window size */
        unsigned             win_size;

        /* Num of RNDV UD QPs */
        unsigned             num_qps;

        /* The same meaning as rx.drop_rate applying to RNDV fragments */
        unsigned             frag_drop_rate;

        /* for rndv unexpected fragments simulation */
        unsigned             frag_unexpected_rate;
    } zcopy_rndv;

} mxm_ud_ep_opts_t;


/*
 * Common RC and DC options.
 */
typedef struct mxm_cib_ep_opts {

    mxm_ib_ep_opts_t         ib;

    /* Maximal amount of QPs allowed */
    unsigned                 qp_limit;

    /* Path MTU */
    mxm_ib_mtu_t             path_mtu;

    /* InfiniBand minimum receiver not ready timer, in seconds */
    unsigned                 min_rnr_timer; // XXX time

    /* InfiniBand transmit timeout */
    unsigned                 timeout; // XXX time

    /* InfiniBand maximum pending RDMA destination operations */
    unsigned                 max_rdma_dst_ops;

    /* InfiniBand transmit retry count */
    unsigned                 retry_count;

    /* InfiniBand 'receiver not ready' retry count */
    unsigned                 rnr_retry;

    /* Use HW atomics - depended on device capabilities */
    int                      use_hw_atomics;

    struct {
        /* Enable eager-rdma */
        int                  enable;

        /* Use RDMA for short messages after this number of messages are received */
        size_t               threshold;

        /* Maximum number of peers allowed to use RDMA for short messages */
        unsigned             max_channels;

        /* Number of RDMA buffers to allocate per channel */
        unsigned             buffs_per_channel;

        /* Maximum size (in bytes) of eager RDMA messages */
        size_t               buff_size;

    } eager_rdma;

    /* TX options */
    struct {
        unsigned             cq_size;      /* Send CQ length */

        unsigned             progress_count; /* Number of times to progress pending */

    } tx;

    /* RX options */
    struct {
        unsigned             srq_fill_size; /* RX buffers to prepost */

        double               srq_resize_factor; /* By how much to resize SRQ */
    } rx;

} mxm_cib_ep_opts_t;


/*
 * RC options.
 */
typedef struct mxm_rc_ep_opts {

    mxm_cib_ep_opts_t        cib;

} mxm_rc_ep_opts_t;


/*
 * DC options.
 */
typedef struct mxm_dc_ep_opts {

    mxm_cib_ep_opts_t        cib;

    /* Use WR_NOP to get completion notifications sooner */
    int                      use_nop;

    /* DC QPs allocated for RDMA sends */
    unsigned                 rdma_qp_limit;

    /* DC TX usage policy */
    mxm_dc_tx_policy_t       tx_policy;

    /* DCS TX policy params */
    struct {
        double               dci_increment;
        int                  cc_enable;
    } dcs;

} mxm_dc_ep_opts_t;


/**
 * MXM endpoint options.
 */
struct mxm_ep_opts {

    /* Endpoint name, for debugging / logging purposes */
    char                     *ep_name;

    /* Endpoint name size limit */
    int                      ep_name_max;

    /* Transports to use */
    unsigned                 tl_bitmap;

    /*
     * Transport manager options.
     */
    struct {
        /* Bitmask to determine TM considerations update frequency */
        unsigned             update_threshold_mask;

        /* Threshold of channel promotion for difference in counters */
        unsigned             promote_threshold;

        /* Factor to multiply all counters upon promotion as backoff */
        double               backoff_factor;
    } tm;

    /* Enable/Disable endpoint preconnect */
    int                      preconnect;

    /* Loopback transport options */
    mxm_self_ep_opts_t       self;

    /* SHM transport options */
    mxm_shm_ep_opts_t        shm;

    /* OOB transport options */
    mxm_oob_ep_opts_t        oob;

    /* UD transport options */
    mxm_ud_ep_opts_t         ud;

    /* RC transport options */
    mxm_rc_ep_opts_t         rc;

    /* DC transport options */
    mxm_dc_ep_opts_t         dc;

};


END_C_DECLS

#endif /* MXM_CONFIG_H_ */
