use std::{
    collections::{HashSet, VecDeque},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use crate::{graph::Graph, page_node::PageNode, uri::Uri};

/**
 * This is the main entry point for crawling a website
 * ---
 *
 * This just stores the controlling metadata (max threads and verbosity) and the results of
 * the site crawl, the intermediate data throughout the crawl will be given to a temporary
 * object and communated with via channels
 */
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
}

enum WorkerInput {
    Task { id: usize, uri: Uri },
    End,
}

enum WorkerOutput {
    Results {
        node: PageNode,
        urls: Vec<String>,
        id: usize,
    },
}

struct Worker {
    thread: thread::JoinHandle<()>,
    input: Sender<WorkerInput>,
}

impl Worker {
    fn spawn(results: Sender<WorkerOutput>) -> Self {
        let (input, tasks) = channel();

        let thread = thread::spawn(move || {
            while let Ok(task) = tasks.recv() {
                match task {
                    WorkerInput::Task { id, uri } => {
                        let response = get_response_sync(&uri);
                        let node = PageNode::default();
                        let urls = vec![];

                        let result = WorkerOutput::Results { node, urls, id };

                        results.send(result).unwrap();
                    }
                    WorkerInput::End => break,
                }
            }
        });

        Self { input, thread }
    }
}

struct Response {}

fn get_response_sync(uri: &Uri) -> Response {
    todo!()
}
