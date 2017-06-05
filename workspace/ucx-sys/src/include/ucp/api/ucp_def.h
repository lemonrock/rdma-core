/*
* Copyright (C) Mellanox Technologies Ltd. 2001-2014.  ALL RIGHTS RESERVED.
* Copyright (C) UT-Battelle, LLC. 2014-2015. ALL RIGHTS RESERVED.
* Copyright (C) IBM 2015. ALL RIGHTS RESERVED.
*
* See file LICENSE for terms.
*/

#ifndef UCP_DEF_H_
#define UCP_DEF_H_

#include <ucs/type/status.h>
#include <stddef.h>
#include <stdint.h>


/**
 * @ingroup UCP_CONTEXT
 * @brief Forward declarations
 */
typedef struct ucp_tag_recv_info         ucp_tag_recv_info_t;


/**
 * @ingroup UCP_CONTEXT
 * @brief UCP Application Context
 *
 * UCP application context (or just a context) is an opaque handle that holds a
 * UCP communication instance's global information.  It represents a single UCP
 * communication instance.  The communication instance could be an OS process
 * (an application) that uses UCP library.  This global information includes
 * communication resources, endpoints, memory, temporary file storage, and
 * other communication information directly associated with a specific UCP
 * instance.  The context also acts as an isolation mechanism, allowing
 * resources associated with the context to manage multiple concurrent
 * communication instances. For example, users using both MPI and OpenSHMEM
 * sessions simultaneously can isolate their communication by allocating and
 * using separate contexts for each of them. Alternatively, users can share the
 * communication resources (memory, network resource context, etc.) between
 * them by using the same application context. A message sent or a RMA
 * operation performed in one application context cannot be received in any
 * other application context.
 */
typedef struct ucp_context               *ucp_context_h;


/**
 * @ingroup UCP_CONFIG
 * @brief UCP configuration descriptor
 *
 * This descriptor defines the configuration for @ref ucp_context_h
 * "UCP application context". The configuration is loaded from the run-time
 * environment (using configuration files of environment variables)
 * using @ref ucp_config_read "ucp_config_read" routine and can be printed
 * using @ref ucp_config_print "ucp_config_print" routine. In addition,
 * application is responsible to release the descriptor using
 * @ref ucp_config_release "ucp_config_release" routine.
 *
 * @todo This structure will be modified through a dedicated function.
 */
typedef struct ucp_config                ucp_config_t;


/**
 * @ingroup UCP_ENDPOINT
 * @brief UCP Endpoint
 *
 * The endpoint handle is an opaque object that is used to address a remote
 * @ref ucp_worker_h "worker". It typically provides a description of source,
 * destination, or both. All UCP communication routines address a destination
 * with the endpoint handle. The endpoint handle is associated with only one
 * @ref ucp_context_h "UCP context". UCP provides the @ref ucp_ep_create
 * "endpoint create" routine to create the endpoint handle and the @ref
 * ucp_ep_destroy "destroy" routine to destroy the endpoint handle.
 */
typedef struct ucp_ep                    *ucp_ep_h;


/**
 * @ingroup UCP_WORKER
 * @brief UCP worker address
 *
 * The address handle is an opaque object that is used as an identifier for a
 * @ref ucp_worker_h "worker" instance.
 */
typedef struct ucp_address               ucp_address_t;


/**
 * @ingroup UCP_ENDPOINT
 * @brief Error handling mode for the UCP endpoint.
 * 
 * Specifies error handling mode for the UCP endpoint.
 */
typedef enum {
    UCP_ERR_HANDLING_MODE_NONE,             /**< No guarantees about error
                                             *   reporting, imposes minimal
                                             *   overhead from a performance
                                             *   perspective */
    UCP_ERR_HANDLING_MODE_PEER              /**< Guarantees that send requests
                                             *   are always completed
                                             *   (successfully or error) even in
                                             *   case of remote failure, disables
                                             *   protocols and APIs which may
                                             *   cause a hang or undefined
                                             *   behavior in case of peer failure,
                                             *   may affect performance and
                                             *   memory footprint */
} ucp_err_handling_mode_t;


/**
 * @ingroup UCP_MEM
 * @brief UCP Remote memory handle
 *
 * Remote memory handle is an opaque object representing remote memory access
 * information. Typically, the handle includes a memory access key and other
 * network hardware specific information, which are input to remote memory
 * access operations, such as PUT, GET, and ATOMIC. The object is
 * communicated to remote peers to enable an access to the memory region.
 */
typedef struct ucp_rkey                  *ucp_rkey_h;


/**
 * @ingroup UCP_MEM
 * @brief UCP Memory handle
 *
 * Memory handle is an opaque object representing a memory region allocated
 * through UCP library, which is optimized for remote memory access
 * operations (zero-copy operations).  The memory handle is a self-contained
 * object, which includes the information required to access the memory region
 * locally, while @ref ucp_rkey_h "remote key" is used to access it
 * remotely. The memory could be registered to one or multiple network resources
 * that are supported by UCP, such as InfiniBand, Gemini, and others.
 */
typedef struct ucp_mem                   *ucp_mem_h;


/**
 * @ingroup UCP_MEM
 * @brief Attributes of the @ref ucp_mem_h "UCP Memory handle", filled by
 *        @ref ucp_mem_query function.
 */
typedef struct ucp_mem_attr {
   /**
     * Mask of valid fields in this structure, using bits from @ref ucp_mem_attr_field.
     * Fields not specified in this mask would be ignored.
     * Provides ABI compatibility with respect to adding new fields.
     */
    uint64_t                field_mask;

    /**
     * Address of the memory segment.
     */
     void                   *address;

    /**
     * Size of the memory segment.
     */
     size_t                 length;
} ucp_mem_attr_t;


/**
 * @ingroup UCP_MEM
 * @brief UCP Memory handle attributes field mask.
 *
 * The enumeration allows specifying which fields in @ref ucp_mem_attr_t are
 * present. It is used for the enablement of backward compatibility support.
 */
enum ucp_mem_attr_field {
    UCP_MEM_ATTR_FIELD_ADDRESS = UCS_BIT(0), /**< Virtual address */
    UCP_MEM_ATTR_FIELD_LENGTH  = UCS_BIT(1)  /**< The size of memory region */
};


/**
 * @ingroup UCP_WORKER
 * @brief UCP Worker
 *
 * UCP worker is an opaque object representing the communication context.  The
 * worker represents an instance of a local communication resource and progress
 * engine associated with it. Progress engine is a construct that is
 * responsible for asynchronous and independent progress of communication
 * directives. The progress engine could be implement in hardware or software.
 * The worker object abstract an instance of network resources such as a host
 * channel adapter port, network interface, or multiple resources such as
 * multiple network interfaces or communication ports. It could also represent
 * virtual communication resources that are defined across multiple devices.
 * Although the worker can represent multiple network resources, it is
 * associated with a single @ref ucp_context_h "UCX application context".
 * All communication functions require a context to perform the operation on
 * the dedicated hardware resource(s) and an @ref ucp_ep_h "endpoint" to address the
 * destination.
 *
 * @note Worker are parallel "threading points" that an upper layer may use to
 * optimize concurrent communications.
 */
 typedef struct ucp_worker                *ucp_worker_h;


/**
 * @ingroup UCP_COMM
 * @brief UCP Tag Identifier
 *
 * UCP tag identifier is a 64bit object used for message identification.
 * UCP tag send and receive operations use the object for an implementation
 * tag matching semantics (derivative of MPI tag matching semantics).
 */
typedef uint64_t                         ucp_tag_t;


/**
 * @ingroup UCP_COMM
 * @brief UCP Message descriptor.
 *
 * UCP Message descriptor is an opaque handle for a message returned by
 * @ref ucp_tag_probe_nb. This handle can be passed to @ref ucp_tag_msg_recv_nb
 * in order to receive the message data to a specific buffer.
 */
typedef struct ucp_recv_desc             *ucp_tag_message_h;


/**
 * @ingroup UCP_COMM
 * @brief UCP Datatype Identifier
 *
 * UCP datatype identifier is a 64bit object used for datatype identification.
 * Predefined UCP identifiers are defined by @ref ucp_dt_type.
 */
typedef uint64_t                         ucp_datatype_t;


/**
 * @ingroup UCP_CONTEXT
 * @brief Request initialization callback.
 *
 * This callback routine is responsible for the request initialization.
 *
 * @param [in]  request   Request handle to initialize.
 */
typedef void (*ucp_request_init_callback_t)(void *request);


/**
 * @ingroup UCP_CONTEXT
 * @brief Request cleanup callback.
 *
 * This callback routine is responsible for cleanup of the memory
 * associated with the request.
 *
 * @param [in]  request   Request handle to cleanup.
 */
typedef void (*ucp_request_cleanup_callback_t)(void *request);


/**
 * @ingroup UCP_COMM
 * @brief Completion callback for non-blocking sends.
 *
 * This callback routine is invoked whenever the @ref ucp_tag_send_nb
 * "send operation" is completed. It is important to note that the call-back is
 * only invoked in a case when the operation cannot be completed in place.
 *
 * @param [in]  request   The completed send request.
 * @param [in]  status    Completion status. If the send operation was completed
 *                        successfully UCX_OK is returned. If send operation was
 *                        canceled UCS_ERR_CANCELED is returned.
 *                        Otherwise, an @ref ucs_status_t "error status" is
 *                        returned.
 */
typedef void (*ucp_send_callback_t)(void *request, ucs_status_t status);


 /**
 * @ingroup UCP_COMM
 * @brief Callback to process peer failure.
 *
 * This callback routine is invoked when transport level error detected.
 *
 * @param [in]  arg      User argument to be passed to the callback.
 * @param [in]  ep       Endpoint to handle transport level error. Upon return
 *                       from the callback, this @a ep is no longer usable and
 *                       all subsequent operations on this @a ep will fail with
 *                       the error code passed in @a status.
 * @param [in]  status   @ref ucs_status_t "error status".
 */
typedef void (*ucp_err_handler_cb_t)(void *arg, ucp_ep_h ep, ucs_status_t status);


 /**
 * @ingroup UCP_COMM
 * @brief UCP endpoint error handling context.
 * 
 * This structure should be initialized in @ref ucp_ep_params_t to handle peer failure
 */
typedef struct ucp_err_handler {
    ucp_err_handler_cb_t cb;       /**< Error handler callback */
    void                 *arg;     /**< User defined argument */
} ucp_err_handler_t;


/**
 * @ingroup UCP_COMM
 * @brief Completion callback for non-blocking tag receives.
 *
 * This callback routine is invoked whenever the @ref ucp_tag_recv_nb
 * "receive operation" is completed and the data is ready in the receive buffer.
 *
 * @param [in]  request   The completed receive request.
 * @param [in]  status    Completion status. If the send operation was completed
 *                        successfully UCX_OK is returned. If send operation was
 *                        canceled UCS_ERR_CANCELED is returned. If the data can
 *                        not fit into the receive buffer the
 *                        @ref UCS_ERR_MESSAGE_TRUNCATED error code is returned.
 *                        Otherwise, an @ref ucs_status_t "error status" is
 *                        returned.
 * @param [in]  info      @ref ucp_tag_recv_info_t "Completion information"
 *                        The @a info descriptor is Valid only if the status is
 *                        UCS_OK.
 */
typedef void (*ucp_tag_recv_callback_t)(void *request, ucs_status_t status,
                                        ucp_tag_recv_info_t *info);

/**
 * @ingroup UCP_WORKER
 * @brief UCP worker wakeup events mask.
 *
 * The enumeration allows specifying which events are expected on wakeup, though
 * empty events are possible.
 */
typedef enum ucp_wakeup_event_types {
    UCP_WAKEUP_RMA         = UCS_BIT(0), /**< Remote memory access send completion */
    UCP_WAKEUP_AMO         = UCS_BIT(1), /**< Atomic operation send completion */
    UCP_WAKEUP_TAG_SEND    = UCS_BIT(2), /**< Tag send completion  */
    UCP_WAKEUP_TAG_RECV    = UCS_BIT(3)  /**< Tag receive completion */
} ucp_wakeup_event_t;

#endif
