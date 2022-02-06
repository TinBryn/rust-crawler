use std::collections::{hash_map::Entry, HashMap, HashSet};

use crate::{page_node::PageNode, uri::Uri};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Graph {
    nodes: HashMap<Uri, PageNode>,
    outgoing_links: HashMap<Uri, HashSet<Uri>>,
    incoming_links: HashMap<Uri, HashSet<Uri>>,
}

impl Graph {
    pub fn add_node(&mut self, uri: Uri) -> Option<&mut PageNode> {
        match self.nodes.entry(uri) {
            Entry::Vacant(e) => Some(e.insert(PageNode::default().enqueue())),
            _ => None,
        }
    }

    pub fn add_neighbor(&mut self, from: Uri, to: Uri) {
        self.outgoing_links
            .entry(from.clone())
            .or_default()
            .insert(to.clone());
        self.incoming_links.entry(to).or_default().insert(from);
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn parents(&self, uri: &Uri) -> Option<impl Iterator<Item = &Uri>> {
        self.incoming_links.get(uri).map(|s| s.iter())
    }

    pub fn get(&self, uri: &Uri) -> Option<&PageNode> {
        self.nodes.get(uri)
    }

    pub fn get_mut(&mut self, uri: &Uri) -> Option<&mut PageNode> {
        self.nodes.get_mut(uri)
    }
}
