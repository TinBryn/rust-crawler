#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeStatus {
    Success,
    Failure,
    None,
    Enqueued,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestType {
    Get,
    Head,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageNode {
    pub request_type: RequestType,
    pub node_status: NodeStatus,
    pub error: String,
    pub response_code: i32,
}

impl PageNode {
    pub fn enqueue(mut self) -> Self {
        self.node_status = NodeStatus::Enqueued;
        self
    }
}

impl Default for PageNode {
    fn default() -> Self {
        Self {
            request_type: RequestType::Get,
            node_status: NodeStatus::None,
            error: String::default(),
            response_code: 0,
        }
    }
}
