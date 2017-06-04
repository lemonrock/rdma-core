/**
* Copyright (C) Mellanox Technologies Ltd. 2001-2012.  ALL RIGHTS RESERVED.
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


#ifndef MXM_WAIT_H_
#define MXM_WAIT_H_

#include <mxm/api/mxm_def.h>

BEGIN_C_DECLS


/**
 * MXM progress callback.
 *
 * @param user_data  User-defined argument.
 */
typedef void (*mxm_progress_cb_t)(void *arg);


/**
 * MXM wait context.
 */
typedef struct mxm_wait mxm_wait_t;
struct mxm_wait {
    mxm_req_base_t         *req;            /* Request we wait for */
    mxm_req_state_t        state;           /* State to wait for */

    /* @deprecated
     * use mxm_progress_register / mxm_progress_unregister
     */
    mxm_progress_cb_t      progress_cb;     /* Callback to call while polling */
    void                   *progress_arg;   /* User-defined argument for the callback */
};


END_C_DECLS

#endif
