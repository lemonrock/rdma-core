# ucx-sys

This is a MIT licensed rust crate. It provides a FFI binding to the UCX library. It is likely that it works better with the `libibverbs` library variant supplied by Mellanox.

## May not be supported but look useful
```C

/**
 * @ingroup UCP_MEM
 * @brief If possible translate remote address into local address which can be
 *        accessed using direct load and store operations.
 *
 * This routine returns a local memory address for the remote address such that
 * application can use the local address for direct memory load and store
 * operations. If the underlying hardware does not support this capability
 * this routine will return a corresponding error.
 *
 * @param [in]  ep              Endpoint handle that was used for rkey object
 *                              creation and is used for the remote memory address.
 * @param [in]  remote_addr     Remote address to translate.
 * @param [out] rkey            Remote key handle for the remote address.
 * @param [out] local_addr_p    Local memory address that can by accessed
 *                              directly using memory load and store operations.
 *
 * @return Error code as defined by @ref ucs_status_t
 */
ucs_status_t ucp_rmem_ptr(ucp_ep_h ep, void *remote_addr, ucp_rkey_h rkey,
                          void **local_addr_p);
```


## Static inline functions and macros not wrapped at this time
```C
/**
 * @ingroup UCP_DATATYPE
 * @brief Generate an identifier for contiguous data type.
 *
 * This macro creates an identifier for contiguous datatype that is defined by
 * the size of the basic element.
 *
 * @param [in]  _elem_size    Size of the basic element of the type.
 *
 * @return Data-type identifier.
 */
#define ucp_dt_make_contig(_elem_size) \
    (((ucp_datatype_t)(_elem_size) << UCP_DATATYPE_SHIFT) | UCP_DATATYPE_CONTIG)


/**
 * @ingroup UCP_DATATYPE
 * @brief Generate an identifier for Scatter-gather IOV data type.
 *
 * This macro creates an identifier for datatype of scatter-gather list
 * with multiple pointers
 *
 * @return Data-type identifier.
 */
#define ucp_dt_make_iov() (UCP_DATATYPE_IOV)


/**
 * @ingroup UCP_CONTEXT
 * @brief UCP context initialization.
 *
 * This routine creates and initializes a @ref ucp_context_h
 * "UCP application context".
 *
 * @warning This routine must be called before any other UCP function
 * call in the application.
 *
 * This routine checks API version compatibility, then discovers the available
 * network interfaces, and initializes the network resources required for
 * discovering of the network and memory related devices.
 *  This routine is responsible for initialization all information required for
 * a particular application scope, for example, MPI application, OpenSHMEM
 * application, etc.
 *
 * @note
 * @li Higher level protocols can add additional communication isolation, as
 * MPI does with it's communicator object. A single communication context may
 * be used to support multiple MPI communicators.
 * @li The context can be used to isolate the communication that corresponds to
 * different protocols. For example, if MPI and OpenSHMEM are using UCP to
 * isolate the MPI communication from the OpenSHMEM communication, users should
 * use different application context for each of the communication libraries.
 *
 * @param [in]  config        UCP configuration descriptor allocated through
 *                            @ref ucp_config_read "ucp_config_read()" routine.
 * @param [in]  params        User defined @ref ucp_params_t configurations for the
 *                            @ref ucp_context_h "UCP application context".
 * @param [out] context_p     Initialized @ref ucp_context_h
 *                            "UCP application context".
 *
 * @return Error code as defined by @ref ucs_status_t
 */
static inline ucs_status_t ucp_init(const ucp_params_t *params,
                                    const ucp_config_t *config,
                                    ucp_context_h *context_p)
{
    return ucp_init_version(UCP_API_MAJOR, UCP_API_MINOR, params, config,
                            context_p);
}

```
