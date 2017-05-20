#include <rdma/rdma_cma.h>

struct sockaddr * rust_rdma_get_local_addr(struct rdma_cm_id * id);

struct sockaddr * rust_rdma_get_peer_addr(struct rdma_cm_id * id);
