use js_sys::{Promise, Function};
use std::rc::Rc;
use wasm_bindgen::{JsValue, __rt::WasmRefCell};
use wasm_bindgen_futures::JsFuture;

type JsResult<T> = Result<T, JsValue>;

#[derive(Clone)]
pub struct WasmPromise {
    cbs: Rc<WasmRefCell<(Function, Function)>>,
    context: Promise
}

impl WasmPromise {

    pub fn new() -> Self {

        let cbs =  Rc::new(WasmRefCell::new((Function::default(), Function::default())));
        let cbs_c = cbs.clone();

        Self {
            context: Promise::new(&mut (Box::new( move |res, rej| {
                *cbs_c.borrow_mut() = (res, rej);
            }) as Box<dyn FnMut(Function, Function)>)),
            cbs
        }
    }

    pub fn resolved(v: JsValue) -> Self {

        let cbs =  Rc::new(WasmRefCell::new((Function::default(), Function::default())));   
        let promise = Promise::resolve(&v);

        Self {
            context: promise,
            cbs 
        }
    }

    pub fn set_promise(&mut self, p: Promise) {
        self.context = p;
    }

    pub fn reject(&self, a: JsValue) -> JsResult<JsValue> {
        self.cbs.borrow().1.call1(&self.context, &a)
    }

    pub fn resolve(&self, v: JsValue) -> JsResult<JsValue> {
       self.cbs.borrow().0.call1(&self.context, &v)
    }

    pub async fn into_future(&self) -> JsResult<JsValue> {
        JsFuture::from(self.context.clone()).await
    }
}
