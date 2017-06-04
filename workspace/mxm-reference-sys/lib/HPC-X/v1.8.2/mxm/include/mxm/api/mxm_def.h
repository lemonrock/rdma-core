/*_
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


#ifndef MXM_API_DEF_H_
#define MXM_API_DEF_H_

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
# define BEGIN_C_DECLS  extern "C" {
# define END_C_DECLS    }
#else
# define BEGIN_C_DECLS
# define END_C_DECLS
#endif

BEGIN_C_DECLS


/* The i-th bit */
#define MXM_BIT(i)               (1ull << (i))

/* Mask of bits 0..i-1 */
#define MXM_MASK(i)              (MXM_BIT(i) - 1)

/* Deprecated */
#define MXM_INVALID_MEM_HANDLE   NULL

/* Maximal active message handler ID (handler id can be 0..MXM_HID_MAX-1) */
#define MXM_HID_MAX              32u


/**
 * MXM status codes.
 */
typedef enum {
    MXM_ERR_FIRST = -1,
    MXM_OK = 0,
    MXM_ERR_NO_MESSAGE,
    MXM_ERR_WOULD_BLOCK,
    MXM_ERR_IO_ERROR,
    MXM_ERR_NO_MEMORY,
    MXM_ERR_INVALID_PARAM,
    MXM_ERR_UNREACHABLE,
    MXM_ERR_INVALID_ADDR,
    MXM_ERR_NOT_IMPLEMENTED,
    MXM_ERR_MESSAGE_TRUNCATED,
    MXM_ERR_NO_PROGRESS,
    MXM_ERR_BUFFER_TOO_SMALL,
    MXM_ERR_NO_ELEM,
    MXM_ERR_SOME_CONNECTS_FAILED,
    MXM_ERR_NO_DEVICE,
    MXM_ERR_BUSY,
    MXM_ERR_CANCELED,
    MXM_ERR_SHMEM_SEGMENT,
    MXM_ERR_ALREADY_EXISTS,
    MXM_ERR_OUT_OF_RANGE,
    MXM_ERR_TIMED_OUT,
    MXM_ERR_EXCEEDS_LIMIT,
    MXM_ERR_UNSUPPORTED,
    MXM_ERR_LAST
} mxm_error_t;


/**
 * Transports, sorted by priority (highest to lowest)
 */
typedef enum mxm_tl_id {
    MXM_TL_FIRST,
    MXM_TL_SELF = MXM_TL_FIRST,
    MXM_TL_SHM,
    MXM_TL_RC,
    MXM_TL_DC,
    MXM_TL_UD,
    MXM_TL_OOB,
    MXM_TL_LAST
} mxm_tl_id_t;


/**
 * Logging levels.
 */
typedef enum {
    MXM_LOG_LEVEL_FATAL,
    MXM_LOG_LEVEL_ERROR,
    MXM_LOG_LEVEL_WARN,
    MXM_LOG_LEVEL_INFO,
    MXM_LOG_LEVEL_DEBUG,
    MXM_LOG_LEVEL_TRACE,
    MXM_LOG_LEVEL_TRACE_REQ,
    MXM_LOG_LEVEL_TRACE_DATA,
    MXM_LOG_LEVEL_TRACE_ASYNC,
    MXM_LOG_LEVEL_TRACE_FUNC,
    MXM_LOG_LEVEL_TRACE_POLL,
    MXM_LOG_LEVEL_LAST
} mxm_log_level_t;


/**
 * Logging categories.
 */
enum {
    MXM_LOG_CATEGORY_MXM,
    MXM_LOG_CATEGORY_ASYNC,
    MXM_LOG_CATEGORY_CONN,
    MXM_LOG_CATEGORY_EP,
    MXM_LOG_CATEGORY_MQ,
    MXM_LOG_CATEGORY_MEM
};


/**
 * Async progress mode.
 */
typedef enum {
    MXM_ASYNC_MODE_SIGNAL,
    MXM_ASYNC_MODE_THREAD,
    MXM_ASYNC_MODE_POLL, /* TODO keep only in debug version */
    MXM_ASYNC_MODE_LAST
} mxm_async_mode_t;


/**
 * Memory allocation mode.
 */
typedef enum mxm_allocator {
    MXM_ALLOCATOR_LIBC,      /* Use glibc's malloc */
    MXM_ALLOCATOR_HUGETLB,   /* Take pages from hugetlb */
    MXM_ALLOCATOR_CPAGES,    /* Use OFED's contiguous pages */
    MXM_ALLOCATOR_MMAP,      /* Use mmap() to get pages */
    MXM_ALLOCATOR_SYSV,      /* Use system V to allocate memory */
    MXM_ALLOCATOR_LAST
} mxm_allocator_t;


/**
 * Ternary logic value.
 */
typedef enum mxm_ternary_value {
    MXM_NO  = 0,
    MXM_YES = 1,
    MXM_TRY = 2,
    MXM_TERNARY_LAST,
} mxm_ternary_value_t;


/**
 * Congestion avoidance modes.
 */
typedef enum mxm_ud_ca {
    MXM_USE_CA_NONE,
    MXM_USE_CA_BIC,
    MXM_USA_CA_LAST
} mxm_ud_ca_t;


/**
 * IB port mapper modes.
 */
typedef enum mxm_ib_map_mode {
    MXM_IB_MAP_FIRST,
    MXM_IB_MAP_AFFINITY,
    MXM_IB_MAP_NEAREST,
    MXM_IB_MAP_RR,
    MXM_IB_MAP_LAST
} mxm_ib_map_mode_t;


/**
 * IB port/path MTU.
 */
typedef enum mxm_ib_mtu {
    MXM_IB_MTU_DEFAULT = 0,
    MXM_IB_MTU_512     = 1,
    MXM_IB_MTU_1024    = 2,
    MXM_IB_MTU_2048    = 3,
    MXM_IB_MTU_4096    = 4,
    MXM_IB_MTU_LAST
} mxm_ib_mtu_t;


/**
 * IB device write-combining modes.
 */
typedef enum mxm_ib_dev_wc_mode {
    MXM_IB_DEV_WC_FOR_WQE,
    MXM_IB_DEV_WC_FOR_DB,
    MXM_IB_DEV_WC_FORCE_FLUSH,
    MXM_IB_DEV_WC_MODE_LAST
} mxm_ib_dev_wc_mode_t;

/**
 * IB lid path policy in case if LMC is used.
 */
typedef enum mxm_ib_lid_path_policy {
    MXM_IB_LID_PATH_POLICY_RR,
    MXM_IB_LID_PATH_POLICY_LAST
} mxm_ib_lid_path_policy_t;

/**
 * Interrupt modes.
 */
typedef enum mxm_int_mode {
    MXM_INT_MODE_RX,
    MXM_INT_MODE_TX,
    MXM_INT_MODE_LAST
} mxm_int_mode_t;


/**
 * Rendezvous mode
 */
typedef enum mxm_rndv_mode {
    MXM_RNDV_MODE_READ,
    MXM_RNDV_MODE_WRITE,
    MXM_RNDV_MODE_DEFAULT,
    MXM_RNDV_MODE_LAST
} mxm_rndv_mode_t;


/*
 * Active message handler flags.
 */
enum {
    MXM_AM_FLAG_DEFAULT      = 0,
    MXM_AM_FLAG_THREAD_SAFE  = MXM_BIT(1),
    MXM_AM_FLAG_CONTEXT_SAFE = MXM_AM_FLAG_THREAD_SAFE | MXM_BIT(2)
};


/*
 * Memory mapping/unmapping flags
 */
enum {
    /* Don't perform any operation, just prevent future use of this memory region.
     * Using this flag ensures mxm_mem_unmap() will not call malloc/free or issue
     * system calls.
     */
    MXM_MEM_UNMAP_MARK_INVALID = MXM_BIT(0),
    MXM_MEM_MAP_ODP_REG        = MXM_BIT(1),
};


/**
 * Error handling modes
 */
typedef enum {
    MXM_HANDLE_ERROR_NONE,      /* No error handling */
    MXM_HANDLE_ERROR_BACKTRACE, /* Print backtrace */
    MXM_HANDLE_ERROR_FREEZE,    /* Freeze and wait for a debugger */
    MXM_HANDLE_ERROR_DEBUG,     /* Attach debugger */
    MXM_HANDLE_ERROR_LAST
} mxm_handle_error_t;


/**
 * Kernel copy modes.
 */
typedef enum mxm_shm_kcopy_mode {
    MXM_SHM_KCOPY_MODE_OFF,
    MXM_SHM_KCOPY_MODE_KNEM,
    MXM_SHM_KCOPY_MODE_AUTODETECT,
    MXM_SHM_KCOPY_MODE_LAST
} mxm_shm_kcopy_mode_t;


/**
 * Statistics aggregation mode
 */
typedef enum mxm_stats_aggregate_mode {
    MXM_STATS_AGGREGATE_SUM,
    MXM_STATS_AGGREGATE_AVERAGE,
    MXM_STATS_AGGREGATE_MIN,
    MXM_STATS_AGGREGATE_MAX,
    MXM_STATS_AGGREGATE_COUNT_NZ,
    MXM_STATS_AGGREGATE_LAST
} mxm_stats_aggregate_mode_t;

/**
 * DC TX usage policy.
 */
typedef enum mxm_dc_tx_policy {
    MXM_DC_TX_POLICY_RANDOM,
    MXM_DC_TX_POLICY_LRU,
    MXM_DC_TX_POLICY_HASH_CONN,
    MXM_DC_TX_POLICY_HASH_DLID,
    MXM_DC_TX_POLICY_DCS,
    MXM_DC_TX_POLICY_LAST
} mxm_dc_tx_policy_t;


/**
 * Configuration printing flags
 */
enum {
    MXM_CONFIG_PRINT_HEADER        = MXM_BIT(0),
    MXM_CONFIG_PRINT_VERSION       = MXM_BIT(1),
    MXM_CONFIG_PRINT_DOC           = MXM_BIT(2),
    MXM_CONFIG_PRINT_GLOBAL_OPTS   = MXM_BIT(3),
    MXM_CONFIG_PRINT_CONTEXT_OPTS  = MXM_BIT(4),
    MXM_CONFIG_PRINT_ENDPOINT_OPTS = MXM_BIT(5),
    MXM_CONFIG_PRINT_HIDDEN        = MXM_BIT(6),
    MXM_CONFIG_PRINT_BUILD_CONFIG  = MXM_BIT(7),

    MXM_CONFIG_PRINT_FULL = MXM_CONFIG_PRINT_HEADER |
                            MXM_CONFIG_PRINT_VERSION |
                            MXM_CONFIG_PRINT_DOC |
                            MXM_CONFIG_PRINT_GLOBAL_OPTS |
                            MXM_CONFIG_PRINT_CONTEXT_OPTS |
                            MXM_CONFIG_PRINT_ENDPOINT_OPTS |
                            MXM_CONFIG_PRINT_HIDDEN |
                            MXM_CONFIG_PRINT_BUILD_CONFIG
};


/**
 * MXM types.
 */
typedef struct mxm_context        *mxm_h;
typedef struct mxm_proto_ep       *mxm_ep_h;
typedef struct mxm_proto_conn     *mxm_conn_h;  /* MXM connection */
typedef struct mxm_proto_mq       *mxm_mq_h;
typedef struct mxm_global_opts    mxm_global_opts_t;
typedef struct mxm_context_opts   mxm_context_opts_t;
typedef struct mxm_ep_opts        mxm_ep_opts_t;
typedef uint32_t                  mxm_tag_t;    /* MXM tag type */
typedef uint32_t                  mxm_imm_t;    /* MXM immediate data type */
typedef uint16_t                  mxm_ctxid_t;  /* MXM context id type */
typedef unsigned long             mxm_vaddr_t;  /* MXM Process virtual address */
typedef uint8_t                   mxm_hid_t;    /* MXM handler id */
typedef struct mxm_proto_recv_seg *mxm_message_h;
typedef struct mxm_mem_key        mxm_mem_key_t;


/**
 * Handler for active messages.
 *
 * @param conn       Connection that received a message.
 * @param imm        Immediate value.
 * @param data       Pointer to data.
 * @param length     Data length.
 * @param offset     Offset of data from the beginning of original message.
 * @param last       True if this is last message fragment.
 */
typedef void  (*mxm_am_handler_t)(mxm_conn_h conn, mxm_imm_t imm, void *data,
                                  size_t length, size_t offset, int last);


END_C_DECLS

#endif /* MXM_DEF_H_ */
