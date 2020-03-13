use std::any::Any;
use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct TypedInputCallbackRef<In> {
    pub id: u64,
    pub _phantom: std::marker::PhantomData<In>,
}

pub struct Callback<In: 'static, Out: 'static> {
    pub id: u64,
    pub callback: Box<dyn Fn(&In) -> Out>,
}

impl<In, Out> Callback<In, Out> {
    pub fn get_input_ref(&self) -> TypedInputCallbackRef<In> {
        TypedInputCallbackRef {
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

pub struct CallbackWrapper {
    id: u64,
    callback: Box<dyn Fn(Box<dyn Any>) -> Option<Box<dyn Any>>>,
}

impl<In: 'static, Out: 'static> Into<CallbackWrapper> for Callback<In, Out> {
    fn into(self) -> CallbackWrapper {
        let id = self.id;
        let x = move |any: Box<dyn Any>| {
            if let Some(event) = any.downcast_ref::<In>() {
                let res: Out = (self.callback)(event);
                let bo = Box::new(res) as Box<dyn Any>;
                Some(bo)
            } else {
                None
            }
        };
        CallbackWrapper {
            id,
            callback: Box::new(x),
        }
    }
}
