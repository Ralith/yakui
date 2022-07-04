#![allow(clippy::transmute_ptr_to_ref)] // thanks, mopa

use std::any::{type_name, Any, TypeId};
use std::fmt;

use glam::Vec2;
use thunderdome::Index;

use crate::dom::{Dom, LayoutDom};
use crate::layout::Constraints;

pub trait Props: Any + fmt::Debug {}
impl<T> Props for T where T: Any + fmt::Debug {}

pub trait ErasedProps: Any {
    fn as_debug(&self) -> &dyn fmt::Debug;
}

impl<T> ErasedProps for T
where
    T: Props,
{
    fn as_debug(&self) -> &dyn fmt::Debug {
        self
    }
}

mopmopafy!(ErasedProps);

pub trait Component: Any + fmt::Debug {
    type Props: Props;

    fn new(index: Index, props: &Self::Props) -> Self;
    fn update(&mut self, props: &Self::Props);
    fn size(&self, dom: &Dom, layout: &mut LayoutDom, constraints: Constraints) -> Vec2;
}

pub fn new<T>(index: Index, props: &dyn ErasedProps) -> Box<dyn ErasedComponent>
where
    T: Component,
{
    let props = props.downcast_ref::<T::Props>().unwrap_or_else(|| {
        panic!(
            "Component {} expects props of type {} (ID {:?}), got ID {:?}",
            type_name::<T>(),
            type_name::<T::Props>(),
            TypeId::of::<T::Props>(),
            props.type_id(),
        )
    });

    let value: T = T::new(index, props);
    let boxed: Box<dyn ErasedComponent> = Box::new(value);
    boxed
}

pub trait ErasedComponent: Any {
    fn update(&mut self, props: &dyn ErasedProps);
    fn size(&self, dom: &Dom, layout: &mut LayoutDom, constraints: Constraints) -> Vec2;

    fn as_debug(&self) -> &dyn fmt::Debug;
}

impl<T> ErasedComponent for T
where
    T: Component,
{
    fn update(&mut self, props: &dyn ErasedProps) {
        let props = props
            .downcast_ref::<T::Props>()
            .unwrap_or_else(|| panic!("Type mixup: unexpected {}", type_name::<T::Props>()));

        <T as Component>::update(self, props);
    }

    fn size(&self, dom: &Dom, layout: &mut LayoutDom, constraints: Constraints) -> Vec2 {
        <T as Component>::size(self, dom, layout, constraints)
    }

    fn as_debug(&self) -> &dyn fmt::Debug {
        self
    }
}

mopmopafy!(ErasedComponent);

// Placeholder component used internally.
#[derive(Debug)]
pub struct DummyComponent;

impl Component for DummyComponent {
    type Props = ();

    fn new(_index: Index, _props: &Self::Props) -> Self {
        Self
    }

    fn update(&mut self, _props: &Self::Props) {}

    fn size(&self, _dom: &Dom, _layout: &mut LayoutDom, _constraints: Constraints) -> Vec2 {
        Vec2::ZERO
    }
}
