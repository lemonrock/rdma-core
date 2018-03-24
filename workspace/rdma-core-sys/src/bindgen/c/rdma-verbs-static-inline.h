#include <rdma/rdma_verbs.h>

int rust_rdma_seterrno(int ret);

struct ibv_mr * rust_rdma_reg_msgs(struct rdma_cm_id * id, void * addr, size_t length);

struct ibv_mr * rust_rdma_reg_read(struct rdma_cm_id * id, void * addr, size_t length);

struct ibv_mr * rust_rdma_reg_write(struct rdma_cm_id * id, void * addr, size_t length);

int rust_rdma_dereg_mr(struct ibv_mr * mr);

int rust_rdma_post_recvv(struct rdma_cm_id * id, void * context, struct ibv_sge * sgl, int nsge);

int rust_rdma_post_sendv(struct rdma_cm_id * id, void * context, struct ibv_sge * sgl, int nsge, int flags);

int rust_rdma_post_readv(struct rdma_cm_id * id, void * context, struct ibv_sge * sgl, int nsge, int flags, uint64_t remote_addr, uint32_t rkey);

int rust_rdma_post_writev(struct rdma_cm_id * id, void * context, struct ibv_sge * sgl, int nsge, int flags, uint64_t remote_addr, uint32_t rkey);

int rust_rdma_post_recv(struct rdma_cm_id * id, void * context, void * addr, size_t length, struct ibv_mr * mr);

int rust_rdma_post_send(struct rdma_cm_id * id, void * context, void * addr, size_t length, struct ibv_mr * mr, int flags);

int rust_rdma_post_read(struct rdma_cm_id * id, void * context, void * addr, size_t length, struct ibv_mr * mr, int flags, uint64_t remote_addr, uint32_t rkey);

int rust_rdma_post_write(struct rdma_cm_id * id, void * context, void * addr, size_t length, struct ibv_mr * mr, int flags, uint64_t remote_addr, uint32_t rkey);

int rust_rdma_post_ud_send(struct rdma_cm_id * id, void * context, void * addr, size_t length, struct ibv_mr * mr, int flags, struct ibv_ah * ah, uint32_t remote_qpn);

int rust_rdma_get_send_comp(struct rdma_cm_id * id, struct ibv_wc * wc);

int rust_rdma_get_recv_comp(struct rdma_cm_id * id, struct ibv_wc * wc);
