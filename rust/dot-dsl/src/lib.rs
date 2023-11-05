pub mod graph {
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    fn with_attribs(
        cur: &HashMap<String, String>,
        attr: &[(&str, &str)],
    ) -> HashMap<String, String> {
        let mut new_attribs = cur.clone();
        for (key, val) in attr.iter() {
            new_attribs.insert(key.to_string(), val.to_string());
        }
        new_attribs
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == name)
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_attrs(mut self, attr: &[(&str, &str)]) -> Self {
            self.attrs = with_attribs(&self.attrs, attr);
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            self.edges = edges.to_vec();
            self
        }
    }

    pub mod graph_items {

        pub mod edge {
            use crate::graph::with_attribs;
            use std::collections::HashMap;

            #[derive(Eq, PartialEq, Debug, Clone)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        from: from.to_string(),
                        to: to.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| &**s)
                }

                pub fn with_attrs(mut self, attribs: &[(&str, &str)]) -> Self {
                    self.attrs = with_attribs(&self.attrs, attribs);
                    self
                }
            }
        }

        pub mod node {
            use crate::graph::with_attribs;
            use std::collections::HashMap;

            #[derive(Eq, PartialEq, Debug, Clone)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attribs: &[(&str, &str)]) -> Self {
                    self.attrs = with_attribs(&self.attrs, attribs);
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| &**s)
                }
            }
        }
    }
}
