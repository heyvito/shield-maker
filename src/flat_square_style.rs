use crate::badge::{Renderer, Badger};
use crate::xml;

pub(crate) struct FlatSquare {}

impl Badger for FlatSquare {
    fn vertical_margin(&self) -> f32 { 0.0 }

    fn height(&self) -> f32 { 20.0 }

    fn shadow(&self) -> bool { false }

    fn render(&self, parent: &Renderer) -> Vec<xml::Node> {
        let background_group = parent.make_background_group_element(false, &[("shape-rendering", "crispEdges")]);
        let foreground_group_element = parent.make_foreground_group_element();

        vec![background_group, foreground_group_element]
    }
}
