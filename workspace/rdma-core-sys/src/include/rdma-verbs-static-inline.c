#include <rdma/rdma_verbs.h>
#include "rdma-verbs-static-inline.h"

int rust_rdma_seterrno(int ret)
{
	return rdma_seterrno(ret);
}

struct ibv_mr * rust_rdma_reg_msgs(struct rdma_cm_id * id, void * addr, size_t length)
{
	return rdma_reg_msgs(id, addr, length);
}

struct ibv_mr * rust_rdma_reg_read(struct rdma_cm_id * id, void * addr, size_t length)
{
	return rdma_reg_read(id, addr, length);
}

struct ibv_mr * rust_rdma_reg_write(struct rdma_cm_id * id, void * addr, size_t length)
{
	return rdma_reg_write(id, addr, length);
}

int rust_rdma_dereg_mr(struct ibv_mr * mr)
{
	return rdma_dereg_mr(mr);
}

int rust_rdma_post_recvv(struct rdma_cm_id * id, void * context, struct ibv_sge * sgl, int nsge)
{
	return rdma_post_recvv(id, context, sgl, nsge);
}

int rust_rdma_post_sendv(struct rdma_cm_id * id, void * context, struct ibv_sge * sgl, int nsge, int flags)
{
	return rdma_post_sendv(id, context, sgl, nsge, flags);
}

int rust_rdma_post_readv(struct rdma_cm_id * id, void * context, struct ibv_sge * sgl, int nsge, int flags, uint64_t remote_addr, uint32_t rkey)
{
	return rdma_post_readv(id, context, sgl, nsge, flags, remote_addr, rkey);
}

int rust_rdma_post_writev(struct rdma_cm_id * id, void * context, struct ibv_sge * sgl, int nsge, int flags, uint64_t remote_addr, uint32_t rkey)
{
	return rdma_post_writev(id, context, sgl, nsge, flags, remote_addr, rkey);
}

int rust_rdma_post_recv(struct rdma_cm_id * id, void * context, void * addr, size_t length, struct ibv_mr * mr)
{
	return rdma_post_recv(id, context, addr, length, mr);
}

int rust_rdma_post_send(struct rdma_cm_id * id, void * context, void * addr, size_t length, struct ibv_mr * mr, int flags)
{
	return rdma_post_send(id, context, addr, length, mr, flags);
}

int rust_rdma_post_read(struct rdma_cm_id * id, void * context, void * addr, size_t length, struct ibv_mr * mr, int flags, uint64_t remote_addr, uint32_t rkey)
{
	return rdma_post_read(id, context, addr, length, mr, flags, remote_addr, rkey);
}

int rust_rdma_post_write(struct rdma_cm_id * id, void * context, void * addr, size_t length, struct ibv_mr * mr, int flags, uint64_t remote_addr, uint32_t rkey)
{
	return rdma_post_write(id, context, addr, length, mr, flags, remote_addr, rkey);
}

int rust_rdma_post_ud_send(struct rdma_cm_id * id, void * context, void * addr, size_t length, struct ibv_mr * mr, int flags, struct ibv_ah * ah, uint32_t remote_qpn)
{
	return rdma_post_ud_send(id, context, addr, length, mr, flags, ah, remote_qpn);
}

int rust_rdma_get_send_comp(struct rdma_cm_id * id, struct ibv_wc * wc)
{
	return rdma_get_send_comp(id, wc);
}

int rust_rdma_get_recv_comp(struct rdma_cm_id * id, struct ibv_wc * wc)
{
	return rdma_get_recv_comp(id, wc);
}
