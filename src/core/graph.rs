use crate::Node;
//use crate::{Engine, Node};

#[derive(Clone, Debug, Default)]
pub struct Graph {
    nodes: Vec<Node>,
    inputs: Vec<Node>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>) -> Graph {
        let mut g = Graph {
            nodes,
            inputs: Vec::new(),
        };
        g.construct();
        return g;
    }

    fn collect(&self, visited: &mut Vec<Node>) {
        self.nodes.iter().for_each(|node| {
            if !visited.contains(node) {
                visited.push(node.clone());
            }
        });
        // def _collect(self, visited=None):
        //     visited = visited or []
        //     for node in visited:
        //         if self._id == node._id:
        //             # already visited
        //             return visited
        //     visited.append(self)
        //     # collect all nodes above
        //     for node in self.upstream():
        //         node._collect(visited)
        //     for node, _ in self.downstream():
        //         node._collect(visited)
        //     return visited
    }


    fn construct(&mut self) {
        // construct a graph from a collection
        // of nodes. Graph will be directed,
        // might be cyclic or disconnected.
        //
        // the goal of construction is to
        // find all the depth 0 nodes (e.g.
        // the inputs) as these drive the
        // execution of the graph
        
        // given a tree that looks like:
        //   A -> B -> D -> F
        //    \-> C -> E /
        //  the result will be: [[A], [B, C], [D, E], [F]]
        //
        //  This will be the order we synchronously execute, so that within a
        //  level nodes' execution will be asynchronous but from level to level
        //  they will be synchronous

        // python code
        // # collect all nodes
        // all_nodes = self._collect()
        // # the list of lists of nodes representing layers in the graph
        // nodes = []
        //
        // # we want to collect all the "top" nodes in the graph
        // tops = set(n._id for n in all_nodes if len(n.upstream()) == 0)
        // tops = [n for n in all_nodes if n._id in tops]
        //
        // if tops_only:
        //     return tops
        //
        // # now descend the graph in layers.
        // nodes_seen = set()
        // to_visit = tops
        // while to_visit:
        //     nodes.append([])
        //
        //     next_to_visit = []
        //     for node in to_visit:
        //         if node._id in nodes_seen:
        //             # TODO allow cycles?
        //             continue
        //
        //         nodes[-1].append(node)
        //         nodes_seen.add(node._id)
        //
        //         if node.downstream():
        //             next_to_visit.extend([x[0] for x in node.downstream()])
        //
        //     to_visit = next_to_visit
        //
        //  if not reverse:
        //     nodes.reverse()
        //
        // return nodes


    }

    // pub fn get_inputs(&mut self, engine_inputs: &mut Vec<Node>) {
    //     engine_inputs.append(&mut self.inputs);
    // }
    pub fn get_inputs(&mut self) -> &mut Vec<Node> {
        &mut self.inputs
    }
}

impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        self.nodes == other.nodes
    }
}

impl Eq for Graph {}
