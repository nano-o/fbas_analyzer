use super::*;

/// Makes non-nested quorum sets containing all immediate graph neighbors
pub struct AllNeighborsQsc {
    graph: Graph,
    connected_nodes: NodeIdSet,
    relative_threshold: Option<f64>,
}
impl AllNeighborsQsc {
    pub fn new(graph: Graph, relative_threshold: Option<f64>) -> Self {
        let connected_nodes = graph.get_connected_nodes();
        AllNeighborsQsc {
            graph,
            connected_nodes,
            relative_threshold,
        }
    }
    pub fn new_67p(graph: Graph) -> Self {
        Self::new(graph, None)
    }
    pub fn new_relative(graph: Graph, relative_threshold: f64) -> Self {
        Self::new(graph, Some(relative_threshold))
    }
}
impl QuorumSetConfigurator for AllNeighborsQsc {
    fn configure(&self, node_id: NodeId, fbas: &mut Fbas) -> ChangeEffect {
        let existing_quorum_set = &mut fbas.nodes[node_id].quorum_set;
        if self.connected_nodes.contains(node_id) && *existing_quorum_set == QuorumSet::new_empty()
        {
            let mut validators = self
                .graph
                .outlinks
                .get(node_id)
                .expect("Graph too small for this FBAS!")
                .clone();

            if !validators.contains(&node_id) {
                // we add nodes to their own quorum sets because
                // 1. nodes in the Stellar network often do it.
                // 2. it makes sense for threshold calculation (for achieving global n=3f+1)
                validators.push(node_id);
            }
            validators.sort_unstable(); // for easier comparability

            let threshold = calculate_threshold(validators.len(), self.relative_threshold);

            existing_quorum_set.validators.extend(validators);
            existing_quorum_set.threshold = threshold;
            Change
        } else {
            NoChange
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_neighbors_qsc_can_be_like_super_safe() {
        let n = 10;
        let all_neighbors_qsc = AllNeighborsQsc::new_relative(Graph::new_full_mesh(n), 1.0);
        let super_safe_qsc = SuperSafeQsc::new();

        let actual = simulate!(all_neighbors_qsc, n);
        let expected = simulate!(super_safe_qsc, n);
        assert_eq!(expected, actual);
    }

    #[test]
    fn all_neighbors_qsc_can_be_like_ideal_safe() {
        let n = 10;
        let all_neighbors_qsc = AllNeighborsQsc::new_67p(Graph::new_full_mesh(n));
        let ideal_qsc = IdealQsc::new();

        let actual = simulate!(all_neighbors_qsc, n);
        let expected = simulate!(ideal_qsc, n);
        assert_eq!(expected, actual);
    }
}
