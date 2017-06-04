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


#ifndef MXM_API_REQ_H_
#define MXM_API_REQ_H_

#include <mxm/api/mxm_def.h>

BEGIN_C_DECLS

/**
 * Maximal number of IOV entries in a vector.
 */
#define MXM_REQ_DATA_MAX_IOV       65535


/*
 * Request states.
 */
typedef enum {
    MXM_REQ_NEW        = MXM_BIT(0), /* New request, not queued, owned by user */
    MXM_REQ_EXPECTED   = MXM_BIT(1), /* Receive only: In the expected queue, owned by MQ */
    MXM_REQ_INPROGRESS = MXM_BIT(2), /* In progress, owned by PTL */
    MXM_REQ_SENT       = MXM_BIT(3), /* In progress, owned by PTL, but user data is no longer needed */
    MXM_REQ_READY      = MXM_BIT(4), /* Done, but still owned by MXM context until callback is invoked */
    MXM_REQ_COMPLETED  = MXM_BIT(5)  /* Completed, owned by user */
} mxm_req_state_t;


/*
 * Data types.
 */
typedef enum {
    MXM_REQ_DATA_BUFFER,
    MXM_REQ_DATA_STREAM,
    MXM_REQ_DATA_IOV
} mxm_req_data_type_t;


/*
 * Send operations.
 */
typedef enum {
    MXM_REQ_OP_SEND,            /* Send data to remote peer */
    MXM_REQ_OP_SEND_SYNC,       /* Send data to remote peer and wait for a remote tag match */
    MXM_REQ_OP_PUT,             /* Put data in remote peer memory */
    MXM_REQ_OP_PUT_SYNC,        /* Put data in remote peer memory and wait until data is written*/
    MXM_REQ_OP_GET,             /* Read data from remote peer memory */
    MXM_REQ_OP_AM,              /* Ask to execute remote callback */
    MXM_REQ_OP_ATOMIC_ADD,      /* Atomic add */
    MXM_REQ_OP_ATOMIC_FADD,     /* Atomic fetch-and-add, result is returned in 'data' */
    MXM_REQ_OP_ATOMIC_SWAP,     /* Atomic swap, 'data' is swapped with remote data */
    MXM_REQ_OP_ATOMIC_CSWAP,    /* Atomic compare-and-swap */
    MXM_REQ_OP_LAST
} mxm_req_opcode_t;


/*
 * Try to make the request complete as soon as possible, even in the cost of
 * wasting resources, because the caller is blocking on the completion of this
 * request.
 */
#define MXM_REQ_SEND_FLAG_BLOCKING     MXM_BIT(0)

/*
 * The request may never complete, at least until a request with a SYNC flag
 * is submitted to the same connection.
 */
#define MXM_REQ_SEND_FLAG_LAZY         MXM_BIT(4)

/*
 * The operation begin processing only after previously submitted operations.
 * (Except for SEND/SEND_SYNC which do not necessarily complete before next FENCE)
 */
#define MXM_REQ_SEND_FLAG_FENCE        MXM_BIT(5)

/*
 * Call completion callback only from progress context, which allows it to call
 * mxm_req_send (by default it's disallowed because causes a recursion).
 */
#define MXM_REQ_SEND_FLAG_REENTRANT    MXM_BIT(6)

/*
 * Don't use send protocols where receiver interaction is required (like RENDEZVOUS for example)
 */
#define MXM_REQ_SEND_FLAG_BCOPY        MXM_BIT(7)


/*
 * Atomic operation order
 */
enum {
    MXM_ATOMIC_OP_ORDER_8BIT  = 0,
    MXM_ATOMIC_OP_ORDER_16BIT = 1,
    MXM_ATOMIC_OP_ORDER_32BIT = 2,
    MXM_ATOMIC_OP_ORDER_64BIT = 3
};


/**
 * Remote access memory key
 */
struct mxm_mem_key {
    char             reserved[48];
};


/**
 * Streaming writer callback function.
 *
 * @param buffer    Buffer to read or write the data
 * @param length    Max. number of bytes to copy.
 * @param offset    Offset in the data stream. First time called with offset = 0,
 *                  and it's incremented by the return value.
 * @param context   User-define context, from req->context.
 *
 * @return Number of bytes processed from the buffer.
 *         For writers - if it's smaller than `length', it means to end the
 *          fragment prematurely, and the callback will be called again until
 *          total length is processed.
 *         For readers, the callback must process the entire buffer, if it returns
 *          a smaller value it means the message is truncated.
 *
 * @note It's guaranteed that (offset + length) is not larger than the declared
 *       stream size (mxm_req_base::data.stream.length).
 *
 */
typedef size_t (*mxm_stream_cb_t)(void *buffer, size_t length, size_t offset,
                                  void *context);


/**
 * Describes a single data buffer, or an IOV entry.
 */
typedef struct mxm_req_buffer mxm_req_buffer_t;
struct mxm_req_buffer {
    void             *ptr;      /* Data buffer */
    size_t           length;    /* Data length */
    void             *memh;     /* Memory handle - deprecated */
};


/**
 * Base type for send / receive requests.
 * A request is allocated by the user, and may NOT be released until MXM completes
 * the request. The data the request points to (data buffer, scatter-gather list)
 * should also remain valid.
 *
 * Request lifetime is:
 *  - Allocated by the user, state = MXM_REQ_NEW. All fields should be initialized,
 *    except for 'reserved' and 'error'.
 *  - Submitted to MXM, via mxm_req_send() or mxm_req_recv().
 *  - Owned by MXM, and progressed during calls to MXM, such as mxm_progress().
 *  - Completed by MXM, state = MXM_REQ_COMPLETED.
 *  - Released by the user.
 */
typedef struct mxm_req_base mxm_req_base_t;
struct mxm_req_base {
    mxm_req_state_t        state;     /* Current request state */
    mxm_mq_h               mq;        /* MQ this request belongs to */
    mxm_conn_h             conn;      /* Connection to send or receive from.
                                         If NULL, receive from any. */
    mxm_req_data_type_t    data_type; /* Request data type */

    /* Request data */
    union {

        /* Single data buffer */
        mxm_req_buffer_t    buffer;   /* Pointer to the buffer */

        /* Streaming data */
        struct {
            mxm_stream_cb_t  cb;      /* Streaming callback */
            size_t           length;  /* Total data length */
        } stream;

        /* Scatter/gather list */
        struct {
            unsigned         count;    /* Number of IOV entries */
            mxm_req_buffer_t *vector;  /* IOV entries */
        } iov;

    } data;

    /* User defined context */
    void                   *context;

    /* Completion callback. The user-defined context is passed as the argument.
     * The callback is not allowed to call mxm_progress().
     * The callback is allowed to free the request structure.
     * If set to NULL, will be ignored.
     */
    void                   (*completed_cb)(void *context);

    /* Completion status code. Valid only for completed requests */
    mxm_error_t            error;

    /* Reserved space for internal use */
    char                   reserved[8];
};


/**
 * Send request.
 */
typedef struct mxm_send_req mxm_send_req_t;
struct mxm_send_req {
    mxm_req_base_t         base;
    mxm_req_opcode_t       opcode;   /* Send operation */
    unsigned               flags;    /* Send flags */

    union {

        /* Send operation */
        struct {
            mxm_tag_t      tag;      /* Send tag */
            mxm_imm_t      imm_data; /* Immediate data */
        } send;

        /* One-sided memory operation */
        struct {
            mxm_vaddr_t    remote_vaddr; /* Remote virtual address */
            mxm_mem_key_t  *remote_mkey; /* Remote memory key */
        } mem;

        /* Active message */
        struct {
            mxm_hid_t      hid;        /* Remote handler id to execute */
            mxm_imm_t      imm_data;   /* Immediate data */
        } am;

        /* Atomic operation.
         * Semantics ("value" is req.atomic.value??, "data" is req.base.data):
         *
         * - ADD   : *remote_vaddr += value
         * - FADD  : *data = *remote_vaddr; *remote_vaddr += value
         * - SWAP  : tmp = *data; *data = *remote_vaddr; *remote_vaddr = tmp
         * - CSWAP : if (*remote_vaddr == value) {
         *               tmp = *data; *data = *remote_vaddr; *remote_vaddr = tmp
         *           }
         */
        struct {
            mxm_vaddr_t    remote_vaddr; /* Remote virtual address */
            mxm_mem_key_t  *remote_mkey; /* Remote memory key */
            uint64_t       value;
            uint8_t        order;        /* log2(size) - can be 0, 1, 2, or 3 */
        } atomic;

    } op;

    /* Reserved space for internal use */
    char                   reserved[72];
};


/*
 * Receive request completion status.
 */
typedef struct mxm_recv_completion mxm_recv_completion_t;
struct mxm_recv_completion {
    mxm_conn_h         source;      /* sender address handle */
    mxm_tag_t          sender_tag;  /* sender original tag */
    mxm_imm_t          sender_imm;  /* sender immediate data */
    size_t             sender_len;  /* sender original length */
    size_t             actual_len;  /* actual received length */
};


/**
 * Receive request.
 */
typedef struct mxm_recv_req mxm_recv_req_t;
struct mxm_recv_req {
    mxm_req_base_t         base;
    mxm_tag_t              tag;        /* Tag to expect */
    mxm_tag_t              tag_mask;   /* Masks the bits in 'tag' to compare */
    mxm_recv_completion_t  completion; /* Completion, valid only for completed requests */

    /* Reserved space for internal use */
    char                   reserved[48];
};


END_C_DECLS

#endif /* MXM_REQ_H_ */
