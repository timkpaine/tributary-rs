use crate::core::{Optional, Graph, Node};

use chrono::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Engine {
    realtime: bool,
    clock: DateTime<Utc>,
    inputs: Vec<Node>,

    // args
    start_time: Optional<DateTime<Utc>>,
    end_time: Optional<DateTime<Utc>>,
}

impl Engine {
    pub fn new(
        realtime: bool,
        start_time: Optional<DateTime<Utc>>,
        end_time: Optional<DateTime<Utc>>,
    ) -> Engine {
        Engine {
            realtime,
            clock: start_time.unwrap_or_else(Utc::now),
            inputs: Vec::new(),
            start_time,
            end_time,
        }
    }

    pub fn set_inputs(&mut self, nodes: Vec<Node>) {
        self.inputs = nodes;
    }

    pub fn attach_nodes(&mut self, nodes: Vec<Node>) {
        // build graph by loading user provided node/nodes
        let mut graph = Graph::new(nodes);

        // let the graph logic determine inputs
        // then grab and attach to engine
        self.inputs.append(graph.get_inputs());
        // graph.get_inputs(&mut self.inputs);

        // nodes.iter().for_each(|&node| {
        // });
    }

    pub fn now(&self) -> DateTime<Utc> {
        self.clock
    }

    pub fn run(&mut self) {
        // TODO
    }
}

/**********************************/
#[cfg(test)]
// use std::default::default;
#[cfg(test)]
mod engine_tests {
    use super::*;

    #[test]
    fn test_construction() {
        let now = Utc::now();

        // now defaults to start time
        let e = Engine::new(true, Some(now), None);
        assert_eq!(e.now(), now);

        // default uses now, older than previous
        let e = Engine::new(true, None, None);
        assert_ne!(e.now(), now);
    }
}
