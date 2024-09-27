use super::{Lookup, LookupDisplay};
use crate::data::{Node, Nodes};

impl From<Nodes> for Lookup<Node> {
    fn from(mut nodes: Nodes) -> Self {
        let mut lookup = Lookup::<Node>::new();
        nodes.nodes.drain().for_each(|(id, node)| {
            let name = node.name.clone();
            lookup.add(node).with_name(&name).with_id(&id);
        });
        lookup
    }
}

impl LookupDisplay for Node {
    fn display() -> &'static str {
        "nodes"
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
