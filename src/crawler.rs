use std::{
    collections::{HashSet, VecDeque},
    sync::{Arc, Mutex}, thread, time::Duration,
};

use crate::{graph::Graph, page_node::PageNode, uri::Uri};

#[derive(Debug)]
pub struct Crawler {
    max_threads: usize,
    verbosity: bool,
    graph: Graph,
}

impl Crawler {
    pub fn new(max_threads: usize) -> Self {
        Self {
            max_threads,
            verbosity: false,
            graph: Default::default(),
        }
    }

    pub fn set_verbosity(&mut self, verbosity: bool) {
        self.verbosity = verbosity;
    }

    pub fn crawl(&mut self, initial_url: &str, display_results: bool) -> &Graph {
        let initial_uri: Uri = initial_url.parse().unwrap();
        let mut domain = initial_uri;
        domain.path.clear();
        domain.query.clear();
        domain.fragment.clear();

        let data = Inner {
            num_threads: 0,
            domain,
            queue: Default::default(),
            currently_being_explored: Default::default(),
            errors: Default::default(),
            graph: Default::default(),
        };

        todo!()
    }
}

#[derive(Debug)]
struct CrawlData {
    max_threads: usize,
    verbosity: bool,
    inner: Arc<Mutex<Inner>>,
}

impl CrawlData {
    fn get_node_sync(&mut self, uri: &Uri) -> Option<PageNode> {
        let inner = self.inner.lock().unwrap();
        inner.graph.get(uri).cloned()
    }
}

#[derive(Debug)]
struct Inner {
    num_threads: usize,
    domain: Uri,
    queue: VecDeque<Uri>,
    currently_being_explored: HashSet<Uri>,
    errors: Vec<String>,
    graph: Graph,
}

impl Inner {
    fn enqueue(&mut self, uri: Uri, max_threads: usize) {
        if self.graph.add_node(uri.clone()).is_none() {
            return;
        }

        println!("Enqueueing {:?} (queue has size {})", uri, self.queue.len());
        if self.num_threads < max_threads {
            self.num_threads += 1;
            self.spawn_crawling_thread(uri);
        } else {
            self.queue.push_back(uri);
        }
    }

    fn spawn_crawling_thread(&mut self, uri: Uri) {
        self.currently_being_explored.insert(uri);

        thread::sleep(Duration::from_millis(100));

        let t = thread::spawn(|| {
            
        });

        
        todo!()
    }
}
