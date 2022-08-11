use js_sys::{Promise, Function};
use std::{rc::Rc, cell::RefCell};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

type JsResult<T> = Result<T, JsValue>;

#[derive(Clone)]
pub struct WasmPromise {
    reject: Rc<RefCell<Function>>,
    resolve: Rc<RefCell<Function>>,
    context: Promise
}

impl WasmPromise {

    pub fn new() -> Self {

        let resolve =  Rc::new(RefCell::new(Function::default()));
        let reject =  Rc::new(RefCell::new(Function::default()));

        let res_c = resolve.clone();
        let rej_c = reject.clone();

        Self {
            context: Promise::new(&mut (Box::new( move |res, rej| {
                *res_c.borrow_mut() = res;
                *rej_c.borrow_mut() = rej;
            }) as Box<dyn FnMut(Function, Function)>)),
            resolve,
            reject,
        }
    }

    pub fn resolved(v: JsValue) -> Self {

        let resolve =  Rc::new(RefCell::new(Function::default()));
        let reject =  Rc::new(RefCell::new(Function::default()));

        let promise = Promise::resolve(&v);

        Self {
            resolve,
            reject,
            context: promise
        }
    }

    pub fn reject(&self, a: JsValue) -> JsResult<JsValue> {
        self.reject.borrow().call1(&self.context, &a)
    }

    pub fn resolve(&self, v: JsValue) -> JsResult<JsValue> {
       self.resolve.borrow().call1(&self.context, &v)
    }

    pub async fn into_future(&self) -> JsResult<JsValue> {
        JsFuture::from(self.context.clone()).await
    }
}
