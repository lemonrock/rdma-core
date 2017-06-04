/**
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


#ifndef __MXM_VERSION_H__
#define __MXM_VERSION_H__

#define MXM_VERNO_MAJOR 3
#define MXM_VERNO_MINOR 6
#define MXM_VERNO_REV   "9eb222f"
#define MXM_VERNO_MICRO 3098
#define MXM_VERNO_STRING "3.6.3098"

#define MXM_MINOR_BIT   (16UL)
#define MXM_MAJOR_BIT   (24UL)
#define MXM_API         ((3L<<MXM_MAJOR_BIT)|(6L << MXM_MINOR_BIT))

#define MXM_VERSION(major, minor) (((major)<<MXM_MAJOR_BIT)|((minor)<<MXM_MINOR_BIT))

#endif
