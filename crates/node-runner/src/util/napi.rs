use anyhow::{anyhow, bail, Context};
use napi::bindgen_prelude::napi_register_module_v1;
use napi::sys::{napi_env, napi_value};
use napi::Env;
use nodejs::run_raw;
use std::ffi::c_void;
use std::fmt::Debug;
use std::ptr::null_mut;
use std::sync::mpsc::channel;
use std::sync::{Mutex, Once};

static ONCE: Once = Once::new();
static GUARD: Mutex<()> = Mutex::new(());

unsafe fn run_napi_inner<F: for<'a> FnOnce(Env)>(f: F) -> i32 {
    static mut MODULE_INIT_FN: *mut c_void = null_mut(); // *mut Option<F>

    let mut module_init_fn = Some(f);
    MODULE_INIT_FN = (&mut module_init_fn) as *mut Option<F> as _;

    unsafe extern "C" fn napi_reg_func<F: for<'a> FnOnce(Env)>(
        env: napi_env,
        exports: napi_value,
    ) -> napi_value {
        ONCE.call_once(|| {
            napi_register_module_v1(env, exports);
        });

        let module_init_fn = (MODULE_INIT_FN as *mut Option<F>).as_mut().unwrap();
        let module_init_fn = module_init_fn.take().unwrap();
        MODULE_INIT_FN = null_mut();
        module_init_fn(Env::from_raw(env));

        exports
    }

    run_raw(napi_reg_func::<F> as _)
}

pub fn run_napi<F: for<'a> FnOnce(Env) -> anyhow::Result<R>, R: Debug>(
    func: F,
) -> anyhow::Result<R> {
    let _guard = GUARD.lock().unwrap();
    if ONCE.is_completed() {
        bail!("run_napi can only be called once");
    }

    let (sender, receiver) = channel::<anyhow::Result<R>>();
    let code = unsafe {
        run_napi_inner(move |env| {
            let res = func(env).map_err(|e| anyhow!(e));

            if let Err(e) = sender.send(res) {
                env.throw_error(&e.to_string(), None).unwrap();
            }
        })
    };

    let result = receiver.recv().context("Failed to run Node.js")??;
    if code != 0 {
        Err(anyhow!("Failed to run Node.js. Exit code: {}", code))
    } else {
        Ok(result)
    }
}
