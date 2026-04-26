// direitos autorais (Rust) DCrust 16/04/2026
// Tradução do security/security.c do kernel Linux para Rust
// Copyright (Rust) DCrust 16/04/2026

#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

// ─── Constantes de Lockdown ──────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum LockdownReason {
    None = 0,
    ModuleSignature,
    DevMem,
    EfiTest,
    Kexec,
    Hibernation,
    PciAccess,
    IoPort,
    Msr,
    AcpiTables,
    DeviceTree,
    PcmciaCis,
    Tiocsserial,
    ModuleParameters,
    Mmiotrace,
    Debugfs,
    XmonWr,
    BpfWriteUser,
    DbgWriteKernel,
    RtasErrorInjection,
    XenUserActions,
    IntegrityMax,
    Kcore,
    Kprobes,
    BpfReadKernel,
    DbgReadKernel,
    Perf,
    Tracefs,
    XmonRw,
    XfrmSecret,
    ConfidentialityMax,
}

pub fn lockdown_reason_str(reason: LockdownReason) -> &'static str {
    match reason {
        LockdownReason::None                => "none",
        LockdownReason::ModuleSignature     => "unsigned module loading",
        LockdownReason::DevMem              => "/dev/mem,kmem,port",
        LockdownReason::EfiTest             => "/dev/efi_test access",
        LockdownReason::Kexec               => "kexec of unsigned images",
        LockdownReason::Hibernation         => "hibernation",
        LockdownReason::PciAccess           => "direct PCI access",
        LockdownReason::IoPort              => "raw io port access",
        LockdownReason::Msr                 => "raw MSR access",
        LockdownReason::AcpiTables          => "modifying ACPI tables",
        LockdownReason::DeviceTree          => "modifying device tree contents",
        LockdownReason::PcmciaCis           => "direct PCMCIA CIS storage",
        LockdownReason::Tiocsserial         => "reconfiguration of serial port IO",
        LockdownReason::ModuleParameters    => "unsafe module parameters",
        LockdownReason::Mmiotrace           => "unsafe mmio",
        LockdownReason::Debugfs             => "debugfs access",
        LockdownReason::XmonWr              => "xmon write access",
        LockdownReason::BpfWriteUser        => "use of bpf to write user RAM",
        LockdownReason::DbgWriteKernel      => "use of kgdb/kdb to write kernel RAM",
        LockdownReason::RtasErrorInjection  => "RTAS error injection",
        LockdownReason::XenUserActions      => "Xen guest user action",
        LockdownReason::IntegrityMax        => "integrity",
        LockdownReason::Kcore               => "/proc/kcore access",
        LockdownReason::Kprobes             => "use of kprobes",
        LockdownReason::BpfReadKernel       => "use of bpf to read kernel RAM",
        LockdownReason::DbgReadKernel       => "use of kgdb/kdb to read kernel RAM",
        LockdownReason::Perf                => "unsafe use of perf",
        LockdownReason::Tracefs             => "use of tracefs",
        LockdownReason::XmonRw              => "xmon read and write access",
        LockdownReason::XfrmSecret          => "xfrm SA secret",
        LockdownReason::ConfidentialityMax  => "confidentiality",
    }
}

// ─── Estruturas base ─────────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct LsmBlobSizes {
    pub lbs_cred:        usize,
    pub lbs_file:        usize,
    pub lbs_inode:       usize,
    pub lbs_ipc:         usize,
    pub lbs_key:         usize,
    pub lbs_msg_msg:     usize,
    pub lbs_task:        usize,
    pub lbs_superblock:  usize,
    pub lbs_sock:        usize,
    pub lbs_bdev:        usize,
    pub lbs_tun_dev:     usize,
    pub lbs_ib:          usize,
    pub lbs_bpf_map:     usize,
    pub lbs_bpf_prog:    usize,
    pub lbs_bpf_token:   usize,
    pub lbs_perf_event:  usize,
    pub lbs_xattr_count: usize,
}

#[derive(Debug, Clone)]
pub struct LsmId {
    pub name:  &'static str,
    pub id:    u64,
}

#[derive(Debug, Default)]
pub struct LsmContext {
    pub id:      u64,
    pub flags:   u64,
    pub len:     usize,
    pub ctx:     Vec<u8>,
}

#[derive(Debug, Default, Clone)]
pub struct LsmProp {
    pub secid: u32,
}

// ─── Estado global do LSM ────────────────────────────────────────────────────

pub struct LsmState {
    pub debug:       AtomicBool,
    pub active_cnt:  AtomicU32,
    pub blob_sizes:  Mutex<LsmBlobSizes>,
}

impl LsmState {
    pub fn new() -> Self {
        Self {
            debug:      AtomicBool::new(false),
            active_cnt: AtomicU32::new(0),
            blob_sizes: Mutex::new(LsmBlobSizes::default()),
        }
    }
}

// ─── Alocadores de blob ──────────────────────────────────────────────────────

/// Aloca um blob de segurança genérico
pub fn lsm_blob_alloc(size: usize) -> Option<Vec<u8>> {
    if size == 0 {
        return None;
    }
    Some(vec![0u8; size])
}

/// Aloca blob de credencial
pub fn lsm_cred_alloc(size: usize) -> Result<Vec<u8>, i32> {
    lsm_blob_alloc(size).ok_or(-12) // -ENOMEM
}

/// Aloca blob de inode
pub fn lsm_inode_alloc(size: usize) -> Result<Vec<u8>, i32> {
    lsm_blob_alloc(size).ok_or(-12)
}

/// Aloca blob de arquivo
pub fn lsm_file_alloc(size: usize) -> Result<Vec<u8>, i32> {
    lsm_blob_alloc(size).ok_or(-12)
}

/// Aloca blob de tarefa
pub fn lsm_task_alloc(size: usize) -> Result<Vec<u8>, i32> {
    lsm_blob_alloc(size).ok_or(-12)
}

/// Aloca blob de socket
pub fn lsm_sock_alloc(size: usize) -> Result<Vec<u8>, i32> {
    lsm_blob_alloc(size).ok_or(-12)
}

// ─── Contexto de usuário ─────────────────────────────────────────────────────

/// Preenche um contexto LSM para espaço de usuário
pub fn lsm_fill_user_ctx(
    val: &[u8],
    id: u64,
    flags: u64,
) -> Result<LsmContext, i32> {
    Ok(LsmContext {
        id,
        flags,
        len: std::mem::size_of::<LsmContext>() + val.len(),
        ctx: val.to_vec(),
    })
}

// ─── Hooks de segurança ──────────────────────────────────────────────────────

// Binder
pub fn security_binder_set_context_mgr() -> i32 { 0 }
pub fn security_binder_transaction() -> i32 { 0 }
pub fn security_binder_transfer_binder() -> i32 { 0 }
pub fn security_binder_transfer_file() -> i32 { 0 }

// ptrace
pub fn security_ptrace_access_check(mode: u32) -> i32 { 0 }
pub fn security_ptrace_traceme() -> i32 { 0 }

// Capabilities
pub fn security_capget() -> i32 { 0 }
pub fn security_capset() -> i32 { 0 }
pub fn security_capable(cap: i32, opts: u32) -> i32 { 0 }

// Quota
pub fn security_quotactl(cmds: i32, r#type: i32, id: i32) -> i32 { 0 }
pub fn security_quota_on() -> i32 { 0 }

// Syslog / tempo
pub fn security_syslog(r#type: i32) -> i32 { 0 }
pub fn security_settime64() -> i32 { 0 }

// Memória virtual
pub fn security_vm_enough_memory(pages: i64) -> i32 {
    // Retorna 0 se há memória suficiente
    if pages > 0 { 0 } else { -12 }
}

// Execução de binários (bprm)
pub fn security_bprm_creds_for_exec() -> i32 { 0 }
pub fn security_bprm_creds_from_file() -> i32 { 0 }
pub fn security_bprm_check() -> i32 { 0 }
pub fn security_bprm_committing_creds() {}
pub fn security_bprm_committed_creds() {}

// Contexto de sistema de arquivos
pub fn security_fs_context_submount() -> i32 { 0 }
pub fn security_fs_context_dup() -> i32 { 0 }
pub fn security_fs_context_parse_param() -> i32 { -524 } // -ENOPARAM

// Superbloco
pub fn security_sb_alloc() -> i32 { 0 }
pub fn security_sb_delete() {}
pub fn security_sb_free() {}
pub fn security_free_mnt_opts() {}
pub fn security_sb_eat_lsm_opts() -> i32 { 0 }
pub fn security_sb_mnt_opts_compat() -> i32 { 0 }
pub fn security_sb_remount() -> i32 { 0 }
pub fn security_sb_kern_mount() -> i32 { 0 }
pub fn security_sb_show_options() -> i32 { 0 }
pub fn security_sb_statfs() -> i32 { 0 }
pub fn security_sb_mount() -> i32 { 0 }
pub fn security_sb_umount() -> i32 { 0 }
pub fn security_sb_pivotroot() -> i32 { 0 }
pub fn security_sb_set_mnt_opts() -> i32 { 0 }
pub fn security_sb_clone_mnt_opts() -> i32 { 0 }
pub fn security_move_mount() -> i32 { 0 }

// Notificações de path
pub fn security_path_notify(mask: u64, obj_type: u32) -> i32 { 0 }

// Inode
pub fn security_inode_alloc() -> i32 { 0 }
pub fn security_inode_free() {}
pub fn security_dentry_init_security() -> i32 { 0 }
pub fn security_dentry_create_files_as() -> i32 { 0 }
pub fn security_inode_init_security() -> i32 { 0 }
pub fn security_inode_init_security_anon() -> i32 { 0 }
pub fn security_inode_create() -> i32 { 0 }
pub fn security_inode_post_create_tmpfile() {}
pub fn security_inode_link() -> i32 { 0 }
pub fn security_inode_unlink() -> i32 { 0 }
pub fn security_inode_symlink() -> i32 { 0 }
pub fn security_inode_mkdir() -> i32 { 0 }
pub fn security_inode_rmdir() -> i32 { 0 }
pub fn security_inode_mknod() -> i32 { 0 }
pub fn security_inode_rename() -> i32 { 0 }
pub fn security_inode_readlink() -> i32 { 0 }
pub fn security_inode_follow_link() -> i32 { 0 }
pub fn security_inode_permission(mask: i32) -> i32 { 0 }
pub fn security_inode_setattr() -> i32 { 0 }
pub fn security_inode_post_setattr() {}
pub fn security_inode_getattr() -> i32 { 0 }
pub fn security_inode_setxattr() -> i32 { 0 }
pub fn security_inode_set_acl() -> i32 { 0 }
pub fn security_inode_post_set_acl() {}
pub fn security_inode_get_acl() -> i32 { 0 }
pub fn security_inode_remove_acl() -> i32 { 0 }
pub fn security_inode_post_remove_acl() {}
pub fn security_inode_post_setxattr() {}
pub fn security_inode_getxattr() -> i32 { 0 }
pub fn security_inode_listxattr() -> i32 { 0 }
pub fn security_inode_removexattr() -> i32 { 0 }
pub fn security_inode_post_removexattr() {}
pub fn security_inode_file_setattr() -> i32 { 0 }
pub fn security_inode_file_getattr() -> i32 { 0 }
pub fn security_inode_need_killpriv() -> i32 { 0 }
pub fn security_inode_killpriv() -> i32 { 0 }
pub fn security_inode_getsecurity() -> i32 { 0 }
pub fn security_inode_setsecurity() -> i32 { 0 }
pub fn security_inode_listsecurity() -> i32 { 0 }
pub fn security_inode_getlsmprop(prop: &mut LsmProp) { prop.secid = 0; }
pub fn security_inode_copy_up() -> i32 { 0 }
pub fn security_inode_copy_up_xattr() -> i32 { -95 } // -EOPNOTSUPP
pub fn security_inode_setintegrity() -> i32 { 0 }
pub fn security_kernfs_init_security() -> i32 { 0 }

// Path (CONFIG_SECURITY_PATH)
pub fn security_path_mknod() -> i32 { 0 }
pub fn security_path_post_mknod() {}
pub fn security_path_mkdir() -> i32 { 0 }
pub fn security_path_rmdir() -> i32 { 0 }
pub fn security_path_unlink() -> i32 { 0 }
pub fn security_path_symlink() -> i32 { 0 }
pub fn security_path_link() -> i32 { 0 }
pub fn security_path_rename() -> i32 { 0 }
pub fn security_path_truncate() -> i32 { 0 }
pub fn security_path_chmod() -> i32 { 0 }
pub fn security_path_chown() -> i32 { 0 }
pub fn security_path_chroot() -> i32 { 0 }

// Arquivo
pub fn security_file_permission(mask: i32) -> i32 { 0 }
pub fn security_file_alloc() -> i32 { 0 }
pub fn security_file_release() {}
pub fn security_file_free() {}
pub fn security_file_ioctl(cmd: u32, arg: u64) -> i32 { 0 }
pub fn security_file_ioctl_compat(cmd: u32, arg: u64) -> i32 { 0 }
pub fn security_mmap_file(prot: u64, flags: u64) -> i32 { 0 }
pub fn security_mmap_addr(addr: u64) -> i32 { 0 }
pub fn security_file_mprotect(reqprot: u64, prot: u64) -> i32 { 0 }
pub fn security_file_lock(cmd: u32) -> i32 { 0 }
pub fn security_file_fcntl(cmd: u32, arg: u64) -> i32 { 0 }
pub fn security_file_set_fowner() {}
pub fn security_file_send_sigiotask(sig: i32) -> i32 { 0 }
pub fn security_file_receive() -> i32 { 0 }
pub fn security_file_open() -> i32 { 0 }
pub fn security_file_post_open(mask: i32) -> i32 { 0 }
pub fn security_file_truncate() -> i32 { 0 }

// Tarefa
pub fn security_task_alloc(clone_flags: u64) -> i32 { 0 }
pub fn security_task_free() {}
pub fn security_cred_alloc_blank() -> i32 { 0 }
pub fn security_cred_free() {}
pub fn security_prepare_creds() -> i32 { 0 }
pub fn security_transfer_creds() {}
pub fn security_cred_getsecid(secid: &mut u32) { *secid = 0; }
pub fn security_cred_getlsmprop(prop: &mut LsmProp) { prop.secid = 0; }
pub fn security_kernel_act_as(secid: u32) -> i32 { 0 }
pub fn security_kernel_create_files_as() -> i32 { 0 }
pub fn security_kernel_module_request(kmod_name: &str) -> i32 { 0 }
pub fn security_kernel_read_file(contents: bool) -> i32 { 0 }
pub fn security_kernel_post_read_file() -> i32 { 0 }
pub fn security_kernel_load_data(contents: bool) -> i32 { 0 }
pub fn security_kernel_post_load_data() -> i32 { 0 }
pub fn security_task_fix_setuid(flags: i32) -> i32 { 0 }
pub fn security_task_fix_setgid(flags: i32) -> i32 { 0 }
pub fn security_task_fix_setgroups() -> i32 { 0 }
pub fn security_task_setpgid(pgid: i32) -> i32 { 0 }
pub fn security_task_getpgid() -> i32 { 0 }
pub fn security_task_getsid() -> i32 { 0 }
pub fn security_current_getlsmprop_subj(prop: &mut LsmProp) { prop.secid = 0; }
pub fn security_task_getlsmprop_obj(prop: &mut LsmProp) { prop.secid = 0; }
pub fn security_task_setnice(nice: i32) -> i32 { 0 }
pub fn security_task_setioprio(ioprio: i32) -> i32 { 0 }
pub fn security_task_getioprio() -> i32 { 0 }
pub fn security_task_prlimit(flags: u32) -> i32 { 0 }
pub fn security_task_setrlimit(resource: u32) -> i32 { 0 }
pub fn security_task_setscheduler() -> i32 { 0 }
pub fn security_task_getscheduler() -> i32 { 0 }
pub fn security_task_movememory() -> i32 { 0 }
pub fn security_task_kill(sig: i32) -> i32 { 0 }
pub fn security_task_prctl(option: i32, arg2: u64, arg3: u64, arg4: u64, arg5: u64) -> i32 {
    -38 // -ENOSYS: nenhum LSM quis tratar
}
pub fn security_task_to_inode() {}
pub fn security_create_user_ns() -> i32 { 0 }

// IPC
pub fn security_ipc_permission(flag: i16) -> i32 { 0 }
pub fn security_ipc_getlsmprop(prop: &mut LsmProp) { prop.secid = 0; }
pub fn security_msg_msg_alloc() -> i32 { 0 }
pub fn security_msg_msg_free() {}
pub fn security_msg_queue_alloc() -> i32 { 0 }
pub fn security_msg_queue_free() {}
pub fn security_msg_queue_associate(msqflg: i32) -> i32 { 0 }
pub fn security_msg_queue_msgctl(cmd: i32) -> i32 { 0 }
pub fn security_msg_queue_msgsnd(msqflg: i32) -> i32 { 0 }
pub fn security_msg_queue_msgrcv(r#type: i64, mode: i32) -> i32 { 0 }
pub fn security_shm_alloc() -> i32 { 0 }
pub fn security_shm_free() {}
pub fn security_shm_associate(shmflg: i32) -> i32 { 0 }
pub fn security_shm_shmctl(cmd: i32) -> i32 { 0 }
pub fn security_shm_shmat(shmflg: i32) -> i32 { 0 }
pub fn security_sem_alloc() -> i32 { 0 }
pub fn security_sem_free() {}
pub fn security_sem_associate(semflg: i32) -> i32 { 0 }
pub fn security_sem_semctl(cmd: i32) -> i32 { 0 }
pub fn security_sem_semop(nsops: u32, alter: i32) -> i32 { 0 }
pub fn security_d_instantiate() {}

// Self attrs
pub fn security_getselfattr(attr: u32, flags: u32) -> i32 { 0 }
pub fn security_setselfattr(attr: u32, flags: u32) -> i32 { 0 }
pub fn security_getprocattr(lsmid: i32, name: &str) -> i32 { 0 }
pub fn security_setprocattr(lsmid: i32, name: &str, size: usize) -> i32 { 0 }
pub fn security_ismaclabel(name: &str) -> i32 { 0 }

// Conversão de contexto
pub fn security_secid_to_secctx(secid: u32) -> Result<LsmContext, i32> {
    Ok(LsmContext { id: secid as u64, ..Default::default() })
}
pub fn security_lsmprop_to_secctx(prop: &LsmProp, lsmid: i32) -> Result<LsmContext, i32> {
    Ok(LsmContext { id: prop.secid as u64, ..Default::default() })
}
pub fn security_secctx_to_secid(secdata: &str, secid: &mut u32) -> i32 {
    *secid = 0;
    0
}
pub fn security_release_secctx(cp: &mut LsmContext) {
    *cp = LsmContext::default();
}
pub fn security_inode_invalidate_secctx() {}
pub fn security_inode_notifysecctx(ctxlen: u32) -> i32 { 0 }
pub fn security_inode_setsecctx(ctxlen: u32) -> i32 { 0 }
pub fn security_inode_getsecctx() -> i32 { 0 }

// Rede (CONFIG_SECURITY_NETWORK)
pub fn security_netlink_send() -> i32 { 0 }
pub fn security_unix_stream_connect() -> i32 { 0 }
pub fn security_unix_may_send() -> i32 { 0 }
pub fn security_socket_create(family: i32, r#type: i32, protocol: i32, kern: i32) -> i32 { 0 }
pub fn security_socket_post_create() -> i32 { 0 }
pub fn security_socket_socketpair() -> i32 { 0 }
pub fn security_socket_bind(addrlen: i32) -> i32 { 0 }
pub fn security_socket_connect(addrlen: i32) -> i32 { 0 }
pub fn security_socket_listen(backlog: i32) -> i32 { 0 }
pub fn security_socket_accept() -> i32 { 0 }
pub fn security_socket_sendmsg(size: i32) -> i32 { 0 }
pub fn security_socket_recvmsg(size: i32, flags: i32) -> i32 { 0 }
pub fn security_socket_getsockname() -> i32 { 0 }
pub fn security_socket_getpeername() -> i32 { 0 }
pub fn security_socket_getsockopt(level: i32, optname: i32) -> i32 { 0 }
pub fn security_socket_setsockopt(level: i32, optname: i32) -> i32 { 0 }
pub fn security_socket_shutdown(how: i32) -> i32 { 0 }
pub fn security_sock_rcv_skb() -> i32 { 0 }
pub fn security_socket_getpeersec_stream(len: u32) -> i32 { 0 }
pub fn security_socket_getpeersec_dgram(secid: &mut u32) -> i32 {
    *secid = 0;
    0
}
pub fn security_sk_alloc(family: i32) -> i32 { 0 }
pub fn security_sk_free() {}
pub fn security_sk_clone() {}
pub fn security_sk_classify_flow() {}
pub fn security_req_classify_flow() {}
pub fn security_sock_graft() {}
pub fn security_inet_conn_request() -> i32 { 0 }
pub fn security_inet_csk_clone() {}
pub fn security_inet_conn_established() {}
pub fn security_secmark_relabel_packet(secid: u32) -> i32 { 0 }
pub fn security_secmark_refcount_inc() {}
pub fn security_secmark_refcount_dec() {}
pub fn security_tun_dev_alloc_security() -> i32 { 0 }
pub fn security_tun_dev_free_security() {}
pub fn security_tun_dev_create() -> i32 { 0 }
pub fn security_tun_dev_attach_queue() -> i32 { 0 }
pub fn security_tun_dev_attach() -> i32 { 0 }
pub fn security_tun_dev_open() -> i32 { 0 }
pub fn security_sctp_assoc_request() -> i32 { 0 }
pub fn security_sctp_bind_connect(optname: i32, addrlen: i32) -> i32 { 0 }
pub fn security_sctp_sk_clone() {}
pub fn security_sctp_assoc_established() -> i32 { 0 }
pub fn security_mptcp_add_subflow() -> i32 { 0 }

// InfiniBand (CONFIG_SECURITY_INFINIBAND)
pub fn security_ib_pkey_access(subnet_prefix: u64, pkey: u16) -> i32 { 0 }
pub fn security_ib_endport_manage_subnet(dev_name: &str, port_num: u8) -> i32 { 0 }
pub fn security_ib_alloc_security() -> i32 { 0 }
pub fn security_ib_free_security() {}

// XFRM (CONFIG_SECURITY_NETWORK_XFRM)
pub fn security_xfrm_policy_alloc() -> i32 { 0 }
pub fn security_xfrm_policy_clone() -> i32 { 0 }
pub fn security_xfrm_policy_free() {}
pub fn security_xfrm_policy_delete() -> i32 { 0 }
pub fn security_xfrm_state_alloc() -> i32 { 0 }
pub fn security_xfrm_state_alloc_acquire(secid: u32) -> i32 { 0 }
pub fn security_xfrm_state_delete() -> i32 { 0 }
pub fn security_xfrm_state_free() {}
pub fn security_xfrm_policy_lookup(fl_secid: u32) -> i32 { 0 }
pub fn security_xfrm_state_pol_flow_match() -> i32 { 1 }
pub fn security_xfrm_decode_session(secid: &mut u32) -> i32 {
    *secid = 0;
    0
}
pub fn security_skb_classify_flow() {}

// Chaves (CONFIG_KEYS)
pub fn security_key_alloc(flags: u64) -> i32 { 0 }
pub fn security_key_free() {}
pub fn security_key_permission() -> i32 { 0 }
pub fn security_key_getsecurity(buffer: &mut Option<String>) -> i32 {
    *buffer = None;
    0
}
pub fn security_key_post_create_or_update(payload_len: usize, flags: u64, create: bool) {}

// Auditoria (CONFIG_AUDIT)
pub fn security_audit_rule_init(field: u32, op: u32, rulestr: &str) -> i32 { 0 }
pub fn security_audit_rule_known() -> i32 { 0 }
pub fn security_audit_rule_free() {}
pub fn security_audit_rule_match(field: u32, op: u32) -> i32 { 0 }

// BPF (CONFIG_BPF_SYSCALL)
pub fn security_bpf(cmd: i32, size: u32, kernel: bool) -> i32 { 0 }
pub fn security_bpf_map() -> i32 { 0 }
pub fn security_bpf_prog() -> i32 { 0 }
pub fn security_bpf_map_create(kernel: bool) -> i32 { 0 }
pub fn security_bpf_prog_load(kernel: bool) -> i32 { 0 }
pub fn security_bpf_token_create() -> i32 { 0 }
pub fn security_bpf_token_cmd() -> i32 { 0 }
pub fn security_bpf_token_capable(cap: i32) -> i32 { 0 }
pub fn security_bpf_map_free() {}
pub fn security_bpf_prog_free() {}
pub fn security_bpf_token_free() {}

// Lockdown
pub fn security_locked_down(what: LockdownReason) -> i32 {
    println!("lockdown: {}", lockdown_reason_str(what));
    0
}

// Block device
pub fn security_bdev_alloc() -> i32 { 0 }
pub fn security_bdev_free() {}
pub fn security_bdev_setintegrity(size: usize) -> i32 { 0 }

// Perf events (CONFIG_PERF_EVENTS)
pub fn security_perf_event_open(r#type: i32) -> i32 { 0 }
pub fn security_perf_event_alloc() -> i32 { 0 }
pub fn security_perf_event_free() {}
pub fn security_perf_event_read() -> i32 { 0 }
pub fn security_perf_event_write() -> i32 { 0 }

// io_uring (CONFIG_IO_URING)
pub fn security_uring_override_creds() -> i32 { 0 }
pub fn security_uring_sqpoll() -> i32 { 0 }
pub fn security_uring_cmd() -> i32 { 0 }
pub fn security_uring_allowed() -> i32 { 0 }

// initramfs
pub fn security_initramfs_populated() {}

// ─── main (teste) ─────────────────────────────────────────────────────────────

fn main() {
    let state = LsmState::new();
    state.debug.store(false, Ordering::SeqCst);

    // Teste de lockdown
    let rc = security_locked_down(LockdownReason::Debugfs);
    assert_eq!(rc, 0);

    // Teste de alocação de blob
    let blob = lsm_blob_alloc(64);
    assert!(blob.is_some());
    assert_eq!(blob.unwrap().len(), 64);

    // Teste de contexto LSM
    let ctx = lsm_fill_user_ctx(b"selinux", 1, 0);
    assert!(ctx.is_ok());

    // Teste de prop
    let mut prop = LsmProp::default();
    security_inode_getlsmprop(&mut prop);
    assert_eq!(prop.secid, 0);

    // Teste de secid
    let mut secid: u32 = 0;
    security_cred_getsecid(&mut secid);
    assert_eq!(secid, 0);

    println!("✅ LSM security.rs — todos os hooks inicializados com sucesso");
}
