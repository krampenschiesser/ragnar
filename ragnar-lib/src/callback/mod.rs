mod native_callback;
mod local_callback;
mod app_callback;

pub use native_callback::{NativeCallback, NativeCallbackWrapper};
pub use local_callback::{LocalCallback, LocalCallbackWrapper};
pub use app_callback::{AppCallback, AppCallbackWrapper};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash,serde::Serialize,serde::Deserialize)]
pub struct CallbackId(u64);

#[derive(Debug, Clone, Copy)]
pub struct TypedInputCallbackRef<In> {
    pub id: CallbackId,
    pub _phantom: std::marker::PhantomData<In>,
}

#[derive(Debug, Clone, Copy)]
pub struct TypedCallbackRef<In, Out> {
    pub id: CallbackId,
    pub _in: std::marker::PhantomData<In>,
    pub _out: std::marker::PhantomData<Out>,
}

impl<In, Out> From<TypedCallbackRef<In, Out>> for TypedInputCallbackRef<In> {
    fn from(r: TypedCallbackRef<In, Out>) -> Self {
        Self {
            id: r.id,
            _phantom: r._in,
        }
    }
}
