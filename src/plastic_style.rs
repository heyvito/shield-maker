use crate::badge::{Renderer, Badger, GradientStop};
use crate::xml;
use crate::xml::Pusher;

pub(crate) struct Plastic {}

impl Badger for Plastic {
    fn vertical_margin(&self) -> f32 { -10.0 }

    fn height(&self) -> f32 { 18.0 }

    fn shadow(&self) -> bool { true }

    fn render(&self, parent: &Renderer) -> Vec<xml::Node> {
        let gradient = xml::Node::with_name_and("linearGradient", |n| {
            n.add_attrs(&[
                ("id", "s"),
                ("x2", "0"),
                ("y2", "100%"),
            ]);
            let stops = vec![
                GradientStop { offset: "0", stop_color: "#fff", stop_opacity: ".7" },
                GradientStop { offset: ".1", stop_color: "#aaa", stop_opacity: ".1" },
                GradientStop { offset: ".9", stop_color: "#000", stop_opacity: ".3" },
                GradientStop { offset: "1", stop_color: "#000", stop_opacity: ".5" },
            ];
            for stop in stops {
                n.push_node_named("stop", |n| stop.into_attributes(n));
            }
        });

        let clip_path = parent.make_clip_path_element(4.0);

        let background_group = parent.make_background_group_element(true, &[("clip-path", "url(#r)")]);
        let foreground_group_element = parent.make_foreground_group_element();

        vec![gradient, clip_path, background_group, foreground_group_element]
    }
}
