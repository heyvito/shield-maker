use std::fmt::Display;

pub(crate) enum Element {
    Text(String),
    Raw(String),
    Node(Node),
}

pub(crate) struct Node {
    name: String,
    attributes: Option<Vec<(String, String)>>,
    content: Option<Vec<Element>>,
}

impl Node {
    pub(crate) fn with_name(name: &str) -> Node {
        Node {
            name: name.into(),
            attributes: None,
            content: None,
        }
    }

    pub(crate) fn with_attributes(name: &str, attributes: &[(&str, &str)]) -> Node {
        let mut node = Node::with_name(name);
        for (k, v) in attributes {
            node.add_attr(*k, *v);
        }

        node
    }

    pub(crate) fn with_name_and<T>(name: &str, and: T) -> Node where T: FnOnce(&mut Node) {
        let mut node = Node::with_name(name);
        and(&mut node);
        node
    }

    pub(crate) fn add_attr<V: Display + ?Sized>(&mut self, name: &str, value: &V) {
        self.attributes.get_or_insert_with(Vec::new).push((name.into(), format!("{}", value)));
    }

    pub(crate) fn add_attrs<V: Display + ?Sized>(&mut self, attrs: &[(&str, &V)]) {
        for (k, v) in attrs {
            self.add_attr(*k, *v);
        }
    }
}

pub(crate) struct Document {
    elements: Vec<Element>,
}

impl Document {
    pub(crate) fn new() -> Document {
        Document {
            elements: Vec::new(),
        }
    }
}

pub(crate) trait Pusher {
    fn push_element(&mut self, elem: Element);

    fn push_text(&mut self, text: &str) {
        self.push_element(Element::Text(text.into()))
    }

    fn push_raw(&mut self, content: &str) {
        self.push_element(Element::Raw(content.into()))
    }

    fn push_node(&mut self, node: Node) {
        self.push_element(Element::Node(node))
    }

    fn push_nodes(&mut self, nodes: Vec<Node>) {
        for n in nodes {
            self.push_node(n);
        }
    }

    fn push_node_and<T>(&mut self, node: Node, and: T) where T: FnOnce(&mut Node) {
        let mut node = node;
        and(&mut node);
        self.push_node(node);
    }

    fn push_node_named<T>(&mut self, name: &str, and: T) where T: FnOnce(&mut Node) {
        self.push_node_and(Node::with_name(name), and);
    }
}

impl Pusher for Node {
    fn push_element(&mut self, elem: Element) {
        self.content.get_or_insert_with(Vec::new).push(elem);
    }
}

impl Pusher for Document {
    fn push_element(&mut self, elem: Element) {
        self.elements.push(elem);
    }
}

pub(crate) struct Renderer {
    inner: Vec<u8>,
}

impl Renderer {
    fn write_escaped(&mut self, data: &str) {
        self.write_raw(&self.escape_xml(data));
    }

    fn write_raw(&mut self, data: &str) {
        self.inner.extend_from_slice(data.as_bytes());
    }

    fn write_attr(&mut self, name: &str, value: &str) {
        let escaped = self.escape_xml(value);
        self.inner.reserve(1 + name.len() + 2 + escaped.len() + 1);
        self.write_raw(" ");
        self.write_raw(name);
        self.write_raw("=\"");
        self.write_raw(&escaped);
        self.write_raw("\"");
    }

    fn escape_xml(&self, text: &str) -> String {
        text
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }

    fn write_node(&mut self, node: &Node) {
        self.write_raw("<");
        self.write_raw(&node.name);
        if let Some(attrs) = node.attributes.as_ref() {
            for (name, value) in attrs {
                self.write_attr(name, value);
            }
        }

        if let Some(elms) = node.content.as_ref() {
            self.write_raw(">");
            for elem in elms {
                self.trampoline(elem);
            }
            self.write_raw(&format!("</{}>", node.name))
        } else {
            self.write_raw("/>");
        }
    }

    fn trampoline(&mut self, el: &Element) {
        match el {
            Element::Text(txt) => self.write_escaped(txt),
            Element::Raw(raw) => self.write_raw(raw),
            Element::Node(node) => self.write_node(node),
        }
    }

    pub(crate) fn render(doc: &Document) -> String {
        let mut writer = Renderer { inner: vec![] };
        for elm in doc.elements.iter() {
            writer.trampoline(elm);
        };
        String::from_utf8(writer.inner).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::xml::*;

    #[test]
    fn writes_simple_element() {
        let person = Node::with_attributes("person", &[
            ("name", "Paul Appleseed"),
            ("email", "paul@example.org"),
        ]);
        let mut doc = Document::new();
        doc.push_node(person);

        let str = Renderer::render(&doc);
        assert_eq!(&str, "<person name=\"Paul Appleseed\" email=\"paul@example.org\"/>");
    }

    #[test]
    fn writes_nested_elements() {
        let mut person = Node::with_attributes("person", &[
            ("name", "Paul Appleseed"),
            ("email", "paul@example.org"),
        ]);

        person.push_node_and(Node::with_name("Todo"), |node| {
            node.push_node_and(Node::with_name("Task"),
                               |n| n.push_text("Water plants"));
            node.push_node_and(Node::with_name("Task"),
                               |n| n.push_text("Pet dog"));
            node.push_node_and(Node::with_name("Task"),
                               |n| n.push_text("Use Rust"));
        });

        person.push_node_and(Node::with_name("danger"),
                             |n| n.push_raw("some raw content!"));

        let mut doc = Document::new();
        doc.push_node(person);

        let str = Renderer::render(&doc);
        assert_eq!(&str, "<person name=\"Paul Appleseed\" email=\"paul@example.org\"><Todo><Task>Water plants</Task><Task>Pet dog</Task><Task>Use Rust</Task></Todo><danger>some raw content!</danger></person>");
    }
}
