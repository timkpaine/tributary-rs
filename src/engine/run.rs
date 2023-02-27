use crate::core::*;
use crate::engine::*;

use chrono::prelude::*;

pub fn run(
    to_run: RunTarget,
    real_time: Optional<bool>,
    start_time: Optional<DateTime<Utc>>,
    end_time: Optional<DateTime<Utc>>,
) {
    let realtime = real_time.unwrap_or(true);

    let nodes = match to_run {
        RunTarget::Node(node) => {
            vec![node]
        }
        RunTarget::Nodes(the_nodes) => the_nodes,
    };

    let mut engine = Engine::new(realtime, start_time, end_time);
    engine.attach_nodes(nodes);
    engine.run();
}
