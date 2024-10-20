#![feature(cfg_target_has_atomic)]
#![feature(core_io_borrowed_buf)]
#![feature(duration_constants)]
#![feature(io_error_uncategorized)]
#![feature(ip)]
#![feature(maybe_uninit_uninit_array)]
#![feature(once_cell_try)]
#![feature(read_buf)]
#![feature(tcp_linger)]
#![feature(try_blocks)]

extern crate eyra;

mod stdtests {
    mod common;
    mod net;
    mod sys_common;

    mod env;
    mod fs;
    mod integration_create_dir_all_bare;
    mod integration_env;
    mod integration_thread;
    mod kernel_copy;
    mod lazy;
    mod net_addr;
    mod net_ip;
    mod net_tcp;
    mod net_udp;
    mod panic;
    mod process;
    mod process_common;
    mod process_unix;
    mod sync_barrier;
    mod sync_condvar;
    mod sync_lazy_lock;
    mod sync_mpsc;
    mod sync_mpsc_sync;
    mod sync_mutex;
    mod sync_once;
    mod sync_once_lock;
    mod sync_rwlock;
    mod thread;
    mod thread_local;
    mod time;
}
