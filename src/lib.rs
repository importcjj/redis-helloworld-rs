extern crate libc;

use libc::{ c_int, size_t};

#[repr(C)]
pub struct RedisModuleCtx;

#[repr(C)]
pub struct RedisModuleString;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    Ok = 0,
    Err = 1,
}

pub type RedisModuleCmdFunc =
    extern "C" fn(ctx: *mut RedisModuleCtx, argv: *mut *mut RedisModuleString, argc: c_int) -> Status;

// #[allow(improper_ctypes)]
// #[link(name = "redismodule", kind = "static")]
extern "C" {
    pub fn Export_RedisModule_Init(
        ctx: *mut RedisModuleCtx,
        modulename: *const u8,
        module_version: c_int,
        api_version: c_int,
    ) -> Status;

    static RedisModule_CreateCommand:
        extern "C" fn(
        ctx: *mut RedisModuleCtx,
        name: *const u8,
        cmdfunc: Option<RedisModuleCmdFunc>,
        strflags: *const u8,
        firstkey: c_int,
        lastkey: c_int,
        keystep: c_int,
    ) -> Status;

    static RedisModule_ReplyWithString:
        extern "C" fn(
            ctx: *mut RedisModuleCtx,
            str: *mut RedisModuleString,
        ) -> Status;

    static RedisModule_CreateString:
        extern "C" fn(ctx: *mut RedisModuleCtx, ptr: *const u8, len: size_t)
        -> *mut RedisModuleString;
}

pub extern "C" fn HelloworldCommand(
    ctx: *mut RedisModuleCtx,
    argv: *mut *mut RedisModuleString,
    argc: c_int,
) -> Status {
    unsafe {
        let s: *mut RedisModuleString = RedisModule_CreateString(ctx, "hello, world!".as_ptr(), 13);
        RedisModule_ReplyWithString(ctx, s);
    }
    
    Status::Ok
}

#[no_mangle]
pub extern "C" fn RedisModule_OnLoad(
    ctx: *mut RedisModuleCtx,
    argv: *mut *mut RedisModuleString,
    argc: c_int,
) -> Status {
    println!("{}", "hello, module was load!");
    unsafe {
        let MODULE_VERSION: c_int = 1;
        let REDISMODULE_APIVER_1: c_int = 1;
        let MODULE_NAME: &str = "hellworld";

        if Export_RedisModule_Init(
            ctx,
            MODULE_NAME.as_ptr(),
            MODULE_VERSION,
            REDISMODULE_APIVER_1,
        ) == Status::Err
        {
            return Status::Err;
        }

        if RedisModule_CreateCommand(
            ctx,
            format!("{}\0", "helloworld").as_ptr(),
            Some(HelloworldCommand),
            "\0".as_ptr(),
            0,
            0,
            0,
        ) == Status::Err
        {
            return Status::Err;
        }
    }

    Status::Ok
}
