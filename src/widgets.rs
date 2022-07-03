use glam::Vec2;
use thunderdome::Index;

use crate::component::Component;
use crate::context::Context;
use crate::layout::Constraints;
use crate::snapshot::Element;

#[derive(Debug)]
pub struct List {
    pub props: ListProps,
    pub index: Index,
}

#[derive(Debug, Clone)]
pub struct ListProps {
    pub direction: Direction,
}

impl ListProps {
    pub fn vertical() -> Self {
        Self {
            direction: Direction::Down,
        }
    }
}

impl Component for List {
    type Props = ListProps;

    fn new(index: Index, props: &Self::Props) -> Self {
        Self {
            props: props.clone(),
            index,
        }
    }

    fn update(&mut self, props: &Self::Props) {
        self.props = props.clone();
    }

    fn size(&self, constraints: Constraints) -> Vec2 {
        Vec2::ZERO
    }
}

#[derive(Debug, Clone)]
pub struct FixedSizeBox {
    pub size: Vec2,
}

impl Component for FixedSizeBox {
    type Props = Self;

    fn new(index: Index, props: &Self::Props) -> Self {
        props.clone()
    }

    fn update(&mut self, props: &Self::Props) {
        *self = props.clone();
    }

    fn size(&self, constraints: Constraints) -> Vec2 {
        constraints.constrain(self.size)
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Down,
    Right,
}

pub fn vertical<F: FnOnce()>(contents: F) {
    let context = Context::active();

    let id = context
        .borrow_mut()
        .snapshot_mut()
        .push(Element::new::<List, _>(ListProps::vertical()));

    contents();

    context.borrow_mut().snapshot_mut().pop(id);
}

pub fn fsbox<S: Into<Vec2>>(size: S) {
    let context = Context::active();

    let size = size.into();
    context
        .borrow_mut()
        .snapshot_mut()
        .insert(Element::new::<FixedSizeBox, FixedSizeBox>(FixedSizeBox {
            size,
        }));
}
