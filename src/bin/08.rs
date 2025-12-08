use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
pub struct Coord {
    x: u64,
    y: u64,
    z: u64,
}

impl From<&str> for Coord {
    fn from(value: &str) -> Self {
        let mut split = value.split(",");
        let x = split.next().unwrap().parse::<u64>().unwrap();
        let y = split.next().unwrap().parse::<u64>().unwrap();
        let z = split.next().unwrap().parse::<u64>().unwrap();
        Coord { x, y, z }
    }
}

impl Coord {
    pub fn abs_diff(&self, other: &Coord) -> u64 {
        let distance_x = self.x.abs_diff(other.x);
        let distance_y = self.y.abs_diff(other.y);
        let distance_z = self.z.abs_diff(other.z);
        distance_x + distance_y + distance_z
    }
}

// Index graph
pub type NodeIx = usize;
pub type EdgeIx = usize;
pub struct Node {
    data: Coord,
    edge: Option<EdgeIx>,
}

#[derive(Clone)]
pub struct Edge {
    source: NodeIx,
    target: NodeIx,
    next_edge: Option<EdgeIx>,
    len: u64,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.len.eq(&other.len)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.len.partial_cmp(&other.len)
    }

    fn lt(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .is_some_and(std::cmp::Ordering::is_lt)
    }

    fn le(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .is_some_and(std::cmp::Ordering::is_le)
    }

    fn gt(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .is_some_and(std::cmp::Ordering::is_gt)
    }

    fn ge(&self, other: &Self) -> bool {
        self.partial_cmp(other)
            .is_some_and(std::cmp::Ordering::is_ge)
    }
}

pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn add_edge(&mut self, source: NodeIx, target: NodeIx, len: u64) {
        let edge_ix = self.edges.len();
        let node = &mut self.nodes[source];
        self.edges.push(Edge {
            source,
            target,
            next_edge: node.edge,
            len,
        });
        node.edge = Some(edge_ix);
    }

    pub fn add_node(&mut self, data: Coord) -> NodeIx {
        let ix = self.nodes.len();
        self.nodes.push(Node { data, edge: None });
        ix
    }

    pub fn successors<'graph>(&'graph self, source: NodeIx) -> Successors<'graph> {
        let first = self.nodes[source].edge;
        Successors {
            graph: self,
            curr_edge_ix: first,
        }
    }

    fn closest_n(&self, count: usize) -> Vec<(NodeIx, NodeIx)> {
        let mut sorted = self.edges.to_vec();
        sorted.sort_by(|a, b| a.len.cmp(&b.len));
        let shortest_x: Vec<(NodeIx, NodeIx)> = self
            .edges
            .iter()
            .take(count)
            .map(|e| (e.source, e.target))
            .collect();
        shortest_x
    }
}

fn make_circuits(pairs: &[(NodeIx, NodeIx)]) -> HashMap<usize, HashSet<usize>> {
    let mut map: HashMap<NodeIx, HashSet<NodeIx>> = HashMap::new();
    for (a, b) in pairs {
        let a_occupied = &map.contains_key(a);
        let b_occupied = &map.contains_key(b);
        match (a_occupied, b_occupied) {
            (true, true) => panic!("something wrong here!"),
            (true, false) => {
                map.entry(*a).and_modify(|x| {
                    x.insert(*b);
                });
            }
            (false, true) => {
                map.entry(*b).and_modify(|x| {
                    x.insert(*a);
                });
            }
            (false, false) => {
                let mut set: HashSet<usize> = HashSet::new();
                set.insert(*b);
                map.insert(*a, set);
            }
        }
    }
    map
}

pub struct Successors<'graph> {
    graph: &'graph Graph,
    curr_edge_ix: Option<EdgeIx>,
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIx;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_edge_ix {
            None => None,
            Some(edge_nm) => {
                let edge = &self.graph.edges[edge_nm];
                self.curr_edge_ix = edge.next_edge;
                Some(edge.target)
            }
        }
    }
}

fn all_pairs(coords: &[Coord]) -> Vec<(Coord, Coord)> {
    let len = coords.len();
    let mut res: Vec<(Coord, Coord)> = Vec::new();
    for a in 0..len {
        if a < (len - 1) {
            // ignore for last element
            for b in a + 1..len {
                let ca = coords[a];
                let cb = coords[b];
                res.push((ca, cb));
            }
        }
    }
    res
}

fn create_graph(input: &str) -> Graph {
    let mut graph: Graph = Graph {
        nodes: vec![],
        edges: vec![],
    };
    let all_nodes: Vec<Coord> = input.lines().map(Coord::from).collect();
    let mut node_ids: HashMap<Coord, usize> = HashMap::new();
    for n in &all_nodes {
        let id = graph.add_node(*n);
        node_ids.insert(*n, id);
    }
    let all_pair: Vec<(Coord, Coord)> = all_pairs(&all_nodes);
    for (a, b) in all_pair {
        let len = a.abs_diff(&b);
        let a_ix = node_ids.get(&a).unwrap();
        let b_ix = node_ids.get(&b).unwrap();
        graph.add_edge(*a_ix, *b_ix, len);
    }
    graph
}

fn pt1_impl(input: &str, count: usize) -> Option<u64> {
    let graph = create_graph(input);
    let shortest_n = graph.closest_n(count);
    let circuits = make_circuits(&shortest_n);
    dbg!(&circuits);
    let res: Vec<usize> = circuits.values().map(|s| s.len()).collect();
    let product: usize = res.iter().product();
    Some(product as u64)
}

pub fn pt1_example(input: &str) -> Option<u64> {
    pt1_impl(input, 10)
}

pub fn pt1_actual(input: &str) -> Option<u64> {
    pt1_impl(input, 1000)
}

pub fn part_one(input: &str) -> Option<u64> {
    pt1_actual(input)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = pt1_example(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_coord_add_new() {
        let mut map = CoordMap::new();
        let a = Coord::new(0, 1, 2);
        let b = Coord::new(2, 1, 0);
        map.add_new(a);
        assert_eq!(1, map.current_id);
        map.add_new(b);
        assert_eq!(2, map.current_id);
        assert_eq!(map.get_id(&a), Some(&0));
        assert_eq!(map.get_id(&b), Some(&1));
    }

    #[test]
    fn test_coord_add_existing() {
        let mut map = CoordMap::new();
        let a = Coord::new(0, 1, 2);
        let b = Coord::new(2, 1, 0);
        map.add_new(a);
        map.add_existing(b, 0);
        assert_eq!(map.get_id(&a), Some(&0));
        assert_eq!(map.get_id(&b), Some(&0));
    }
}
