use std::any::{Any, TypeId};
use std::marker::PhantomData;

use crate::arg::Arg;

pub trait System {
    fn arg_type(&self) -> Vec<TypeId>;
    fn arg_count(&self) -> usize;

    fn call(&self, args: Vec<&dyn Any>);
}

pub struct Arg0System<F: Fn()>(F);

impl<F: Fn()> System for Arg0System<F> {
    fn arg_type(&self) -> Vec<TypeId> {
        vec![]
    }

    fn arg_count(&self) -> usize {
        0
    }

    fn call(&self, _: Vec<&dyn Any>) {
        (self.0)();
    }
}

pub struct Arg1System<F: Fn(&A), A: Arg>(F, PhantomData<A>);

impl<F: Fn(&A), A: Arg> System for Arg1System<F, A> {
    fn arg_type(&self) -> Vec<TypeId> {
        vec![TypeId::of::<A>()]
    }

    fn arg_count(&self) -> usize {
        1
    }

    fn call(&self, args: Vec<&dyn Any>) {
        let arg = args[0].downcast_ref::<A>().unwrap();
        (self.0)(arg);
    }
}

pub struct Arg2System<F: Fn(&A1, &A2), A1: Arg, A2: Arg>(F, PhantomData<A1>, PhantomData<A2>);

impl<F: Fn(&A1, &A2), A1: Arg, A2: Arg> System for Arg2System<F, A1, A2> {
    fn arg_type(&self) -> Vec<TypeId> {
        vec![TypeId::of::<A1>(), TypeId::of::<A2>()]
    }

    fn arg_count(&self) -> usize {
        2
    }

    fn call(&self, args: Vec<&dyn Any>) {
        let a1 = args[0].downcast_ref::<A1>().unwrap();
        let a2 = args[1].downcast_ref::<A2>().unwrap();
        (self.0)(a1, a2);
    }
}

pub struct Arg3System<F: Fn(&A1, &A2, &A3), A1: Arg, A2: Arg, A3: Arg>(
    F,
    PhantomData<A1>,
    PhantomData<A2>,
    PhantomData<A3>,
);

impl<F: Fn(&A1, &A2, &A3), A1: Arg, A2: Arg, A3: Arg> System for Arg3System<F, A1, A2, A3> {
    fn arg_type(&self) -> Vec<TypeId> {
        vec![TypeId::of::<A1>(), TypeId::of::<A2>(), TypeId::of::<A3>()]
    }

    fn arg_count(&self) -> usize {
        3
    }

    fn call(&self, args: Vec<&dyn Any>) {
        let a1 = args[0].downcast_ref::<A1>().unwrap();
        let a2 = args[1].downcast_ref::<A2>().unwrap();
        let a3 = args[2].downcast_ref::<A3>().unwrap();
        (self.0)(a1, a2, a3);
    }
}

impl<F: Fn()> From<F> for Arg0System<F> {
    fn from(value: F) -> Self {
        Arg0System(value)
    }
}

impl<F: Fn(&A), A: Arg> From<F> for Arg1System<F, A> {
    fn from(value: F) -> Self {
        Arg1System(value, PhantomData)
    }
}

impl<F: Fn(&A1, &A2), A1: Arg, A2: Arg> From<F> for Arg2System<F, A1, A2> {
    fn from(value: F) -> Self {
        Arg2System(value, PhantomData, PhantomData)
    }
}

impl<F: Fn(&A1, &A2, &A3), A1: Arg, A2: Arg, A3: Arg> From<F> for Arg3System<F, A1, A2, A3> {
    fn from(value: F) -> Self {
        Arg3System(value, PhantomData, PhantomData, PhantomData)
    }
}
