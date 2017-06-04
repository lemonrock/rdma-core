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

#ifndef MXM_API_H_
#define MXM_API_H_

#include <mxm/api/mxm_version.h>
#include <mxm/api/mxm_config.h>
#include <mxm/api/mxm_req.h>
#include <mxm/api/mxm_wait.h>
#include <mxm/api/mxm_def.h>

BEGIN_C_DECLS


/**
 * A memory key which can be used for zero-sized remote memory operations.
 */
extern mxm_mem_key_t mxm_empty_mem_key;


/**
 * @param  error MXM status code.
 *
 * @return Verbose status message.
 */
const char *
mxm_error_string(mxm_error_t error);


/**
 * Get MXM library version.
 *
 * @return Numeric library version.
 */

unsigned long
mxm_get_version(void);


/**
 * Get MXM library version as a string.
 *
 * @return String with full mxm version.
 */
const char *
mxm_get_version_string(void);


/**
 * Initialize MXM and create MXM context for later operations.
 *
 * @param opts      Options for creating MXM. On return, filled with actual values.
 * @param contextp  Filled with the MXM context handle.
 *
 * @return MXM status code.
 */
mxm_error_t
mxm_init(mxm_context_opts_t *opts, mxm_h *context_p);


/**
 * Cleanup MXM.
 */
void
mxm_cleanup(mxm_h context);


/**
 * Set active message handler for the API.
 * Passing NULL as the callback will remote it.
 *
 * @param context    Context to set the handler for.
 * @param hid        Handler ID to set. Must be in the range 0..MXM_HID_MAX-1
 * @param cb         Function to call when processing active message.
 * @param flags      Bitmask of flags that specify when handler can be called:
 *
 *   MXM_AM_FLAG_DEFAULT
 *          Handler will be never called from progress thread or communication
 *          context. If message arrives it will be queued and called from mxm
 *          progress on the main thread.
 *          MXM_REQ_FLAG_SEND_SYNC means - remote side has queued the message.
 *
 *   MXM_AM_FLAG_THREAD_SAFE
 *          Handler may be called from any thread, but not from communication
 *          context. If a message arrives, it will be queued and called when
 *          communication processing is done.
 *          MXM_REQ_FLAG_SEND_SYNC means - remote side has queued the message.
 *
 *   MXM_AM_FLAG_CONTEXT_SAFE
 *          Handler may be called from any context and any thread, which implies
 *          it does not call mxm functions itself. The handler will be called as
 *          soon as the message arrives.
 *          MXM_REQ_FLAG_SEND_SYNC means - remote side has called the handler.
 *
 * @note Setting another handler with the same handler id overrides the previous one.
 * @note Setting the handler to NULL will remove it.
 *
 * @return MXM status code.
 */
mxm_error_t
mxm_set_am_handler(mxm_h context, mxm_hid_t hid, mxm_am_handler_t cb, unsigned flags);


/**
 * Map/allocate a memory region.
 * Note that mappings are created with page-level granularity.
 *
 * @param context      MXM context.
 * @param address_p    Points to an address of area to map. If the address_p points to NULL,
 *                     a new area is allocated and address_p is filled with a pointer to it.
 *                     Otherwise, the given area is assumed to be allocated, and mapped.
 * @param length_p     Points to the length of area to allocated / map. In case
 *                     of allocation, updated with the actual length (which can be
 *                     larger than requested).
 * @param flags        Set to 0.
 * @param remote_mkey  If non-NULL, creates a mapping to a remote area. Works only
 *                     for keys obtained from processes on the same physical machine.
 * @param offset       Offset within remote memory region to map.
 *
 * @return MXM status code
 */
mxm_error_t
mxm_mem_map(mxm_h context, void **address_p, size_t *length_p, unsigned flags,
            mxm_mem_key_t *remote_mkey, size_t offset);


/**
 * Unmap a previously mapped area. If the area was allocated during mxm_mem_map(),
 * then it's released and becomes unavailable after this call.
 *
 * @param context    MXM context.
 * @param address    Address of area to unmap.
 * @param length     Length of area to unmap.
 * @param flags      Memory unmapping flags.
 */
mxm_error_t
mxm_mem_unmap(mxm_h context, void *address, size_t length, unsigned flags);


/**
 * Fill a memory access key which can be passed around and used to access / map
 * this memory from remote processes.
 *
 * @param context       MXM context.
 * @param address       Area whose memory key to obtain.
 * @param mkey          Memory key to fill.
 *
 * @return MXM status code
 */
mxm_error_t
mxm_mem_get_key(mxm_h context, void *address, mxm_mem_key_t *mkey);


/**
 * Create MXM endpoint to be used for communications.
 *
 * @param context  MXM context.
 * @param opts     Options for creating the endpoint. On return, filled with actual values.
 * @param ep_p     Filled with the created endpoint handle.
 *
 * @return MXM status code.
 */
mxm_error_t
mxm_ep_create(mxm_h context, mxm_ep_opts_t *opts, mxm_ep_h *ep_p);


/**
 * Ensure internal progress.
 *
 *
 * @return MXM status code.
 *     MXM_ERR_NO_PROGRESS if no progress was made.
 */
mxm_error_t
mxm_progress(mxm_h context);


/**
 * Wait for a condition.
 *
 * @param wait       Specifies what to wait for, see mxm_wait.h
 */
void
mxm_wait(mxm_wait_t *wait);


/**
 * Destroy MXM endpoint.
 *
 * @param ep Endpoint to destroy.
 */
void
mxm_ep_destroy(mxm_ep_h ep);


/**
 * Initialize all MXM endpoint connections.
 *
 * @param ep Endpoint to wire up.
 * @return MXM status code.
 */
mxm_error_t
mxm_ep_wireup(mxm_ep_h ep);

/**
 * Start closing all MXM endpoint connections.
 *
 * @param ep Endpoint to power down.
 * @return MXM status code.
 */
mxm_error_t
mxm_ep_powerdown(mxm_ep_h ep);

/**
 * Get endpoint network address.
 * Note: the length of the address may differ between endpoints!
 *
 * @param ep       Endpoint handle.
 * @param address  Buffer to fill with endpoint address.
 * @param addrlen  The user should pass the size of 'address' buffer, and the value
 *                 will be updated with the actual address size.
 *
 * @return MXM_OK on success
 *         MXM_ERR_BUFFER_TOO_SMALL if the address buffer is too small.
 *          In this case *addrlen_p is set to the required buffer size.
 */
mxm_error_t
mxm_ep_get_address(mxm_ep_h ep, void *address, size_t *addrlen_p);


/**
 * Connect to a remote endpoint. This operation completes immediately, and
 * no real connections are made. The wire-up happens when sending the first
 * message on the connection.
 *
 * @param ep          Endpoint handle.
 * @param address     Address which was filled by mxm_ep_address().
 * @param conn_p      Filled with handle to created connection.
 */
mxm_error_t
mxm_ep_connect(mxm_ep_h ep, void *address, mxm_conn_h *conn_p);


/**
 * Disconnect from a specific destination.
 * This function will block until the disconnect is acknowledged by the remote peer.
 *
 * @param conn        Connection handle to disconnect.
 *
 * @return MXM status code.
 */
mxm_error_t
mxm_ep_disconnect(mxm_conn_h conn);


/**
 * Set connection specific context
 *
 * @param conn        Connection handle.
 * @param ctx         User supplied context.
 */
void mxm_conn_ctx_set(mxm_conn_h conn, void *ctx);


/**
 * Get connection specific context
 *
 * @param conn        Connection handle
 * @return            User supplied context
 */
void *mxm_conn_ctx_get(mxm_conn_h conn);


/**
 * Create a matched queue.
 * Matched queue is a context of sending and receiving messages which maintains
 * ordering between requests. It resembles an MPI communicator.
 *
 * @param context   MXM context.
 * @param ctxid     MQ fabric-wide context id.
 * @param mqp       Filled with a handle to the created MQ.
 */
mxm_error_t
mxm_mq_create(mxm_h context, mxm_ctxid_t ctxid, mxm_mq_h *mqp);


/**
 * Destroy a matched queue.
 *
 * @param mq        MQ to destroy.
 */
void
mxm_mq_destroy(mxm_mq_h mq);

/**
 * Send a message.
 *
 * @param req       Send request.
 *
 *  See detailed documentation of 'mxm_send_req_t' in mxm_req.h.
 */
mxm_error_t
mxm_req_send(mxm_send_req_t *req);

/**
 * Receive a message.
 *
 * @param req        Receive request.
 *
 *  See detailed documentation of 'mxm_recv_req_t' in mxm_req.h.
 */
mxm_error_t
mxm_req_recv(mxm_recv_req_t *req);


/**
 * Check if a receive request can be satisfied immediately.
 *
 * @param req        Receive request to probe.
 * @param msgp       If non-NULL, filled with a handle to the incoming message.
 *                   This message must be either received or released.
 *
 * NOTE: The request data buffer field is not used.
 */
mxm_error_t
mxm_req_mprobe(mxm_recv_req_t *req, mxm_message_h *msgp);

/**
 * Check if a receive request can be satisfied immediately.
 *
 * @param req        Receive request to probe.
 *
 * NOTE: The request data buffer field is not used.
 */
mxm_error_t
mxm_req_probe(mxm_recv_req_t *req);


/**
 * Receive a previously probed message. After a successful call to this function,
 * the message handle is no longer valid.
 *
 * @param req        Request to receive.
 * @param msg        Message to receive into the request.
 */
mxm_error_t
mxm_message_recv(mxm_recv_req_t *req, mxm_message_h msg);


/**
 * Release a probed message.
 *
 * @param msg        Message to release.
 */
mxm_error_t
mxm_message_release(mxm_message_h msg);


/**
 * Check for request completion.
 *
 * @param req        Request to check.
 */
static inline int mxm_req_test(mxm_req_base_t *req)
{
    return req->state == MXM_REQ_COMPLETED;
}


/**
 * Wait for request completion.
 *
 * @param req        Request to wait for.
 *
 * NOTE: Other pending requests may be progressed as well.
 */
static inline void mxm_req_wait(mxm_req_base_t *req)
{
    mxm_wait_t wait;
    wait.req = req;
    wait.state = MXM_REQ_COMPLETED;
    wait.progress_cb = NULL;
    mxm_wait(&wait);
}


/**
 * Try to cancel a previously posted send request.
 * The request will be completed regardless of remote peer actions, possibly
 * with error = MXM_ERR_CANCELED.
 *
 * @param req    Request to cancel.
 *
 * @return MXM_OK if the cancellation process has completed.
 *         MXM_ERR_NO_PROGRESS if the cancellation process has started.
 *         otherwise - the request is not valid for cancellation.
 */
mxm_error_t
mxm_req_cancel_send(mxm_send_req_t *req);


/**
 * Try to cancel a previously posted receive request.
 * The request will be completed regardless of remote peer actions, possibly
 * with error = MXM_ERR_CANCELED.
 *
 * @param req    Request to cancel.
 *
 * @return MXM_OK if the cancellation process has completed.
 *         MXM_ERR_NO_PROGRESS if the cancellation process has started.
 *         otherwise - the request is not valid for cancellation.
 */
mxm_error_t
mxm_req_cancel_recv(mxm_recv_req_t *req);


/**
 * Register a function which will be called whenever MXM is blocking.
 *
 * @param context      MXM context.
 * @param arg          Argument to pass to the callback.
 * @param progress_cb  Callback to register.
 */
mxm_error_t
mxm_progress_register(mxm_h context, mxm_progress_cb_t progress_cb, void *arg);


/**
 * Remove a previously registered progress callback.
 *
 * @param context      MXM context.
 * @param progress_cb  Callback to remove.
 */
mxm_error_t
mxm_progress_unregister(mxm_h context, mxm_progress_cb_t progress_cb);


/**
 * Read MXM configuration.
 *
 * @param ctx_optsp    Filled with a pointer to context options.
 * @param ep_optsp     Filled with a pointer to endpoint options.
 * @param prefix       If non-NULL, prefix for configuration variables. Usually it's
 *                     an upper-case name.
 * @param config_file  Reserved for future use: if non-NULL, path to configuration file.
 * @param flags        Reserved: set to 0.
 *
 * Order of parsing is:
 * 1. Set all values to defaults.
 * 2. Modify according to configuration file.
 * 3. Modify according to environment variables.
 *
 */
mxm_error_t
mxm_config_read_opts(mxm_context_opts_t **ctx_optsp, mxm_ep_opts_t **ep_optsp,
                     const char *prefix, const char *config_file, unsigned flags);


/**
 * Free the memory allocated by mxm_config_read_context_opts().
 *
 * @param opts Options to release.
 */
void
mxm_config_free_context_opts(mxm_context_opts_t *opts);


/**
 * Free the memory allocated by mxm_config_read_ep_opts().
 *
 * @param opts Options to release.
 */
void
mxm_config_free_ep_opts(mxm_ep_opts_t *opts);


/**
 * Print out MXM configuration.
 *
 * @param stream      Stream to print the configuration to.
 * @param ctx_opts    Context options to print.
 * @param ep_opts     Endpoint options to print.
 * @param flags       Control output.
 */
void
mxm_config_print(FILE *stream, mxm_context_opts_t *ctx_opts, mxm_ep_opts_t *ep_opts,
                 unsigned flags);


/**
 * @deprecated
 * @see mxm_config_read_opts.
 *
 * @param optsp Filled with a pointer to context options.
 * @note The returned pointer must be released with mxm_config_free().
 */
mxm_error_t
mxm_config_read_context_opts(mxm_context_opts_t **optsp);


/**
 * @deprecated
 * @see mxm_config_read_opts.
 *
 * @param optsp Filled with a pointer to endpoint options.
 * @note The returned pointer must be released with mxm_config_free().
 */
mxm_error_t
mxm_config_read_ep_opts(mxm_ep_opts_t **optsp);


END_C_DECLS

#endif /* MXM_API_H_ */
