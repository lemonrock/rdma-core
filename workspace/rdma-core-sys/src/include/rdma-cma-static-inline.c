#include <rdma/rdma_cma.h>
#include "rdma-cma-static-inline.h"

struct sockaddr * rust_rdma_get_local_addr(struct rdma_cm_id * id)
{
	return rdma_get_local_addr(id);
}

struct sockaddr * rust_rdma_get_peer_addr(struct rdma_cm_id * id)
{
	return rdma_get_peer_addr(id);
}
