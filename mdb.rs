#![feature(globs)]
#![crate_id = "rmdb"]
#![crate_type = "rlib"]
#![crate_type = "staticlib"] // for now
#![allow(non_camel_case_types)]
#![allow(dead_code)] // TODO: remove it once wrapper complete

extern crate libc;

#[link(name = "mdb")]
extern {}

pub mod mdb {
    use libc::{c_int, c_uint, size_t, c_char, c_void, c_uchar, c_ushort, off_t};
    use libc::types::os::common::posix01::pthread_t;

    pub use self::consts::*;
    use self::os::*;

    pub mod consts {
        use libc::{c_int, c_uint};

        // Return codes
        pub static MDB_SUCCESS: c_int = 0;
        pub static MDB_KEYEXIST: c_int = -30799;
        pub static MDB_NOTFOUND: c_int = -30798;
        pub static MDB_PAGE_NOTFOUND: c_int = -30797;
        pub static MDB_CORRUPTED: c_int = -30796;
        pub static MDB_PANIC: c_int = -30795;
        pub static MDB_VERSION_MISMATCH: c_int = -30794;
        pub static MDB_INVALID: c_int = -30793;
        pub static MDB_MAP_FULL: c_int = -30792;
        pub static MDB_DBS_FULL: c_int = -30791;
        pub static MDB_READERS_FULL: c_int = -30790;
        pub static MDB_TLS_FULL: c_int = -30789;
        pub static MDB_TXN_FULL: c_int = -30788;
        pub static MDB_CURSOR_FULL: c_int = -30787;
        pub static MDB_PAGE_FULL: c_int = -30786;
        pub static MDB_MAP_RESIZED: c_int = -30785;
        pub static MDB_INCOMPATIBLE: c_int = -30784;
        pub static MDB_BAD_RSLOT: c_int = -30783;
        pub static MDB_BAD_TXN: c_int = -30782;
        pub static MDB_BAD_VALSIZE: c_int = -30781;

        // Write flags
        pub static MDB_NOOVERWRITE: c_int = 0x10;
        pub static MDB_NODUPDATA: c_int = 0x20;
        pub static MDB_CURRENT: c_int = 0x40;
        pub static MDB_RESERVE: c_int = 0x10000;
        pub static MDB_APPEND: c_int = 0x20000;
        pub static MDB_APPENDDUP: c_int = 0x40000;
        pub static MDB_MULTIPLE: c_int = 0x80000;

        // Database flags
        pub static MDB_REVERSEKEY: c_uint = 0x02;
        pub static MDB_DUPSORT: c_uint = 0x04;
        pub static MDB_INTEGERKEY: c_uint = 0x08;
        pub static MDB_DUPFIXED: c_uint = 0x10;
        pub static MDB_INTEGERDUP: c_uint = 0x20;
        pub static MDB_REVERSEDUP: c_uint =  0x40;
        pub static MDB_CREATE: c_uint = 0x40000;

        // Environment flags
         pub static MDB_FIXEDMAP: c_uint =  0x01;
        pub static MDB_NOSUBDIR: c_uint = 0x4000;
        pub static MDB_NOSYNC: c_uint = 0x10000;
        pub static MDB_RDONLY: c_uint = 0x20000;
        pub static MDB_NOMETASYNC: c_uint = 0x40000;
        pub static MDB_WRITEMAP: c_uint = 0x80000;
        pub static MDB_MAPASYNC: c_uint = 0x100000;
        pub static MDB_NOTLS: c_uint = 0x200000;
        pub static MDB_NOLOCK: c_uint =  0x400000;
        pub static MDB_NORDAHEAD: c_uint = 0x800000;
        pub static MDB_NOMEMINIT: c_uint =  0x1000000;
    }

    #[cfg(target_os = "macos")]
    #[cfg(target_os = "ios")]
    #[cfg(target_os = "linux")]
    #[cfg(target_os = "freebsd")]
    #[cfg(target_os = "android")]
    mod os {
        use libc::{c_ulong, c_int, mode_t, pid_t};
        pub use self::mutex::pthread_mutex_t;

        #[cfg(target_os = "macos")]
        pub type pthread_key_t = c_ulong;
        #[cfg(not(target_os = "macos"))]
        pub type pthread_key_t = c_uint;

        pub type mdb_mode_t = mode_t;
        pub type mdb_filehandle_t = c_int;
        pub type MDB_PID_T = pid_t;


        // TODO: avoid duplication of pthread_mutex_t declaration
        // It should be somehow extracted from std::unstable::mutex
        #[cfg(target_os = "freebsd")]
        mod mutex {
            use libc;
            pub type pthread_mutex_t = *libc::c_void;
        }

        #[cfg(target_os = "macos")]
        #[cfg(target_os = "ios")]
        mod mutex {
            use libc;

            #[cfg(target_arch = "x86_64")]
            static __PTHREAD_MUTEX_SIZE__: uint = 56;
            #[cfg(target_arch = "x86")]
            static __PTHREAD_MUTEX_SIZE__: uint = 40;
            #[cfg(target_arch = "arm")]
            static __PTHREAD_MUTEX_SIZE__: uint = 40;

            pub struct pthread_mutex_t {
            __sig: libc::c_long,
            __opaque: [u8, ..__PTHREAD_MUTEX_SIZE__],
            }
        }

        #[cfg(target_os = "linux")]
        mod os {
            use libc;

            // minus 8 because we have an 'align' field
            #[cfg(target_arch = "x86_64")]
            static __SIZEOF_PTHREAD_MUTEX_T: uint = 40 - 8;
            #[cfg(target_arch = "x86")]
            static __SIZEOF_PTHREAD_MUTEX_T: uint = 24 - 8;
            #[cfg(target_arch = "arm")]
            static __SIZEOF_PTHREAD_MUTEX_T: uint = 24 - 8;
            #[cfg(target_arch = "mips")]
            static __SIZEOF_PTHREAD_MUTEX_T: uint = 24 - 8;

            pub struct pthread_mutex_t {
                __align: libc::c_longlong,
                size: [u8, ..__SIZEOF_PTHREAD_MUTEX_T],
            }

        }
        #[cfg(target_os = "android")]
        mod os {
            use libc;

            pub struct pthread_mutex_t { value: libc::c_int }
            pub struct pthread_cond_t { value: libc::c_int }
        }
    }

    #[cfg(target_os = "windows")]
    mod os {
        use libc::{c_int, c_void};

        pub type mdb_mode_t = c_int;
        pub type mdb_filehandle_t = *c_void;
        pub type pthread_key_t = u32;
        pub type pthread_mutex_t = c_int;
    }

    type pgno_t = MDB_ID;
    type txnid_t = MDB_ID;
    type indx_t = u16;

    type MDB_dbi = c_uint;
    type MDB_ID = size_t;
    type MDB_IDL = *MDB_ID;


    type MDB_rel_func = fn(*MDB_val, *c_void, *c_void, *c_void);
    type MDB_msg_func = fn(*c_char, *c_void) -> c_int;
    type MDB_cmp_func = fn(*MDB_val, *MDB_val) -> c_int;

    type HANDLE = c_int;

    struct MDB_ID2 {
        mid: MDB_ID,
        mptr: *c_void
    }

    type MDB_ID2L = *MDB_ID2;

    struct MDB_val {
        mv_size: size_t,
        mv_data: *c_void,
    }

    struct MDB_rxbody {
        mrb_txnid: txnid_t,
        mrb_pid: MDB_PID_T,
        mrb_tid: pthread_t
    }

    /*
    enum MDB_reader_mru {
        mrx: MDB_rxbody,
        pad: char[] // PADDING
    }

    struct MDB_reader {
        mru: MDB_reader_mru
    }
    */

    struct MDB_txbody {
        mtb_magic: u32,
        mtb_format: u32,
        mtb_mutx: pthread_mutex_t,
        mtb_txnid: txnid_t,
        mtb_numreaders: c_uint
    }

    /*
    enum MDB_txninfo_mt1 {
        mtb: MDB_txbody,
        pad: c_char[] // PADDING
    }

    enum MDB_txninfo_mt2{
        mt2_wmutex: pthread_mutex_t,
        pad: c_char[] // PADDING
    }

    struct MDB_txninfo {
        mt1: MDB_txninfo_mt1,
        mt2: MDB_txninfo_mt2,
        mti_readers: [MDB_reader, ..1]
    }
    */

    struct MDB_pgstate {
        mf_pghead: *pgno_t,
        mf_pglast: txnid_t
    }

    /*
    enum MDB_page_p {
        p_pgno: pgno_t,
        p_next: *c_void
    }

    enum MDB_page_pb {
        struct {
            pb_lower: indx_t,
            pb_upper: indx_t
        },
        pb_pages: u32
    }
    */

    struct MDB_page {
        mp_p: size_t,
        mp_pad: u16,
        mp_flags: u16,
        mp_pb: u32,/*MDB_page_pb,*/
        mp_ptrs: [indx_t, ..1]
    }

    struct MDB_meta {
        mm_magic: u32,
        mm_version: u32,
        mm_address: *c_void,
        mm_mapsize: size_t,
        mm_dbs: [MDB_db, ..2],
        mm_last_pg: pgno_t,
        mm_txnid: txnid_t
    }

    struct MDB_env {
        me_fd: HANDLE,
        me_lfd: HANDLE,
        me_mfd: HANDLE,
        me_flags: u32,
        me_psize: c_uint,
        me_os_psize: c_uint,
        me_maxreaders: c_uint,
        me_numreaders: c_uint,
        me_numdbs: MDB_dbi,
        me_maxdbs: MDB_dbi,
        me_pid: MDB_PID_T,
        me_path: *c_char,
        me_map: *c_char,
        me_txns: *c_void,//MDB_txninfo,
        me_metas: [*MDB_meta, ..2],
        me_pbuf: *c_void,
        me_txn: *MDB_txn,
        me_mapsize: size_t,
        me_size: off_t,
        me_maxpg: pgno_t,
        me_dbxs: *MDB_dbx,
        me_dbflags: *u16,
        me_txkey: pthread_key_t,
        me_pgstate: MDB_pgstate,
        me_dpages: *MDB_page,
        me_free_pgs: MDB_IDL,
        me_dirty_list: MDB_ID2L,
        me_maxfree_1pg: c_int,
        me_nodemax: c_uint
    }

    struct MDB_db {
        md_pad: u32,
        md_flags: u16,
        md_depth: u16,
        md_branch_pages: pgno_t,
        md_leaf_pages: pgno_t,
        md_overflow_pages: pgno_t,
        md_entries: size_t,
        md_root: pgno_t
    }

    struct MDB_dbx {
        md_name: MDB_val,
        md_cmp: *MDB_cmp_func,
        md_dcmp: *MDB_cmp_func,
        md_rel: *MDB_rel_func,
        md_relctx: c_void
    }

    struct MDB_txn {
        mt_parent: *MDB_txn,
        mt_child: *MDB_txn,
        mt_next_pgno: pgno_t,
        mt_txnid: txnid_t,
        mt_env: *MDB_env,
        mt_free_pgs: MDB_IDL,
        mt_spill_pgs: MDB_IDL,
        mt_u: *c_void, /*enum {
            dirty_list: MDB_ID2L,
            reader: *MDB_reader
        },*/
        mt_dbxs: *MDB_dbx,
        mt_dbs: *MDB_db,
        mt_cursors: **MDB_cursor,
        mt_dbflags: *c_uchar,
        mt_numdbs: MDB_dbi,
        mt_flags: c_uint,
        mt_dirty_room: c_uint
    }

    struct MDB_stat {
        ms_psize: c_uint,
        ms_depth: c_uint,
        ms_branch_pages: size_t,
        ms_leaf_pages: size_t,
        ms_overflow_pages: size_t,
        ms_entries: size_t
    }

    struct MDB_envinfo {
        me_mapaddr: *c_void,
        me_mapsize: size_t,
        me_last_pgno: size_t,
        me_last_txnid: size_t,
        me_maxreaders: c_uint,
        me_numreaders: c_uint
    }

    #[repr(C)]
    enum MDB_cursor_op {
        MDB_FIRST,
        MDB_FIRST_DUP,
        MDB_GET_BOTH,
        MDB_GET_BOTH_RANGE,
        MDB_GET_CURRENT,
        MDB_GET_MULTIPLE,
        MDB_LAST,
        MDB_LAST_DUP,
        MDB_NEXT,
        MDB_NEXT_DUP,
        MDB_NEXT_MULTIPLE,
        MDB_NEXT_NODUP,
        MDB_PREV,
        MDB_PREV_DUP,
        MDB_PREV_NODUP,
        MDB_SET,
        MDB_SET_KEY,
        MDB_SET_RANGE

    }

    struct MDB_cursor {
        mc_next: *MDB_cursor,
        mc_backup: *MDB_cursor,
        mc_xcursor: *MDB_xcursor,
        mc_txn: *MDB_txn,
        mc_dbi: MDB_dbi,
        mc_db: *MDB_db,
        mc_dbx: *MDB_dbx,
        mc_dbflag: *c_uchar,
        mc_snum: c_ushort,
        mc_top: c_ushort,
        mc_flags: c_uint,
        mp_pg: *MDB_page,
        mc_ki: indx_t
    }

    struct MDB_xcursor {
        mx_cursor: MDB_cursor,
        mx_db: MDB_db,
        mx_dbx: MDB_dbx,
        mx_dbflag: c_uchar,
    }

    extern "C" {
        fn mdb_version(major: *c_int, minor: *c_int, patch: *c_int) -> *c_char;
        fn mdb_stderror(err: c_int) -> *c_char;
        fn mdb_env_create(env: **MDB_env) -> c_int;
        fn mdb_env_open(env: *MDB_env, path: *c_char, flags: c_uint, mode: mdb_mode_t) -> c_int;
        fn mdb_env_copy(env: *MDB_env, path: *c_char) -> c_int;
        fn mdb_env_copyfd(env: *MDB_env, fd: mdb_filehandle_t) -> c_int;
        fn mdb_env_stat(env: *MDB_env, stat: *MDB_stat) -> c_int;
        fn mdb_env_info(env: *MDB_env, info: *MDB_envinfo) -> c_int;
        fn mdb_env_sync(env: *MDB_env, force: c_int) -> c_int;
        fn mdb_env_close(env: *MDB_env);
        fn mdb_env_set_flags(env: *MDB_env, flags: c_uint, onoff: c_int) -> c_int;
        fn mdb_env_get_flags(env: *MDB_env, flags: c_uint) -> c_int;
        fn mdb_env_get_path(env: *MDB_env, path: **c_char) -> c_int;
        fn mdb_env_get_fd(env: *MDB_env, fd: *mdb_filehandle_t) -> c_int;
        fn mdb_env_set_mapsize(env: *MDB_env, size: size_t) -> c_int;
        fn mdb_env_set_maxreaders(env: *MDB_env, readers: c_uint) -> c_int;
        fn mdb_env_get_maxreaders(env: *MDB_env, readers: *c_uint) -> c_int;
        fn mdb_env_set_maxdbs(env: *MDB_env, dbs: MDB_dbi) -> c_int;
        fn mdb_env_get_maxkeysize(env: *MDB_env) -> c_int;
        fn mdb_txn_begin(env: *MDB_env, parent: *MDB_txn, flags: c_uint, txn: **MDB_txn) -> c_int;
        fn mdb_txn_env(txn: *MDB_txn) -> *MDB_env;
        fn mdb_txn_commit(txn: *MDB_txn) -> c_int;
        fn mdb_txn_abort(txn: *MDB_txn);
        fn mdb_txn_reset(txn: *MDB_txn);
        fn mdb_txn_renew(txn: *MDB_txn) -> c_int;
        fn mdb_dbi_open(txn: *MDB_txn, name: **c_char, flags: c_uint, dbi: *MDB_dbi) -> c_int;
        fn mdb_stat(txn: *MDB_txn, dbi: MDB_dbi, stat: *MDB_stat) -> c_int;
        fn mdb_dbi_flags(txn: *MDB_txn, dbi: MDB_dbi, flags: *c_uint) -> c_int;
        fn mdb_dbi_close(txn: *MDB_txn, dbi: MDB_dbi);
        fn mdb_drop(txn: *MDB_txn, dbi: MDB_dbi, del: c_int) -> c_int;
        fn mdb_set_compare(txn: *MDB_txn, dbi: MDB_dbi, cmp: MDB_cmp_func) -> c_int;
        fn mdb_set_dupsort(txn: *MDB_txn, dbi: MDB_dbi, cmp: MDB_cmp_func) -> c_int;
        fn mdb_set_relfunc(txn: *MDB_txn, dbi: MDB_dbi, rel: MDB_rel_func) -> c_int;
        fn mdb_set_relctx(txn: *MDB_txn, dbi: MDB_dbi, ctx: *c_void) -> c_int;
        fn mdb_get(txn: *MDB_txn, dbi: MDB_dbi, key: *MDB_val, data: *MDB_val) -> c_int;
        fn mdb_put(txn: *MDB_txn, dbi: MDB_dbi, key: *MDB_val, data: *MDB_val, flags: c_uint) -> c_int;
        fn mdb_del(txn: *MDB_txn, dbi: MDB_dbi, key: *MDB_val, data: *MDB_val) -> c_int;
        fn mdb_cursor_open(txn: *MDB_txn, dbi: MDB_dbi, cursor: **MDB_cursor) -> c_int;
        fn mdb_cursor_close(cursor: *MDB_cursor) -> c_int;
        fn mdb_cursor_renew(txn: *MDB_txn, cursor: *MDB_cursor) -> c_int;
        fn mdb_cursor_txn(cursor: *MDB_cursor) -> *MDB_txn;
        fn mdb_cursor_dbi(cursor: *MDB_cursor) -> *MDB_dbi;
        fn mdb_cursor_get(cursor: *MDB_cursor, key: *MDB_val, data: *MDB_val, op: MDB_cursor_op) -> c_int;
        fn mdb_cursor_put(cursor: *MDB_cursor, key: *MDB_val, data: *MDB_val, flags: c_uint) -> c_int;
        fn mdb_cursor_del(cursor: *MDB_cursor, flags: c_uint) -> c_int;
        fn mdb_cursor_count(cursor: *MDB_cursor, countp: *size_t) -> c_int;
        fn mdb_cmp(txn: *MDB_txn, dbi: MDB_dbi, a: *MDB_val, b: *MDB_val) -> c_int;
        fn mdb_dcmp(txn: *MDB_txn, dbi: MDB_dbi, a: *MDB_val, b: *MDB_val) -> c_int;
        fn mdb_reader_list(env: *MDB_env, func: MDB_msg_func, ctx: *c_void) -> c_int;
        fn mdb_reader_check(env: *MDB_env, dead: *c_int) -> c_int;
    }
}
