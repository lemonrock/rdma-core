use ::libc::pthread_cond_t;
use ::libc::pthread_mutex_t;
use ::libc::sockaddr;
use ::libc::sockaddr_in;
use ::libc::sockaddr_in6;
use ::libc::sockaddr_storage;
use ::libc::socklen_t;
use ::libc::timespec;


// Defined officially in linux/types.h; created here to make it easier to generate this code
pub type __u8 = u8;
pub type __u64 = u64;
pub type __be16 = u16;
pub type __be32 = u32;
pub type __be64 = u64;
