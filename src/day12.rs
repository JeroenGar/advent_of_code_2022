use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::Instant;

const INPUT: &str = include_str!("../input/2022/day12.txt");

fn main() {
    let start = Instant::now();
    let char_matrix = INPUT.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut graph = Graph::new(&char_matrix);
    println!("Part 1: {}", graph.dijkstra());
    //Add a virtual start node and edges to all possible start locations
    let virtual_start = 0;
    let virtual_edges = graph.node_heights.iter().enumerate()
        .filter(|(_, height)| **height == 0)
        .map(|(index, _)| index).collect::<Vec<usize>>();
    graph.start = graph.node_heights.len();
    graph.node_heights.push(virtual_start);
    graph.edges.push(virtual_edges);
    println!("Part 2: {}", graph.dijkstra() - 1); // -1 to account for the virtual start node
    println!("Time: {:?}", start.elapsed());
}

struct Graph {
    start: usize,
    end: usize,
    node_heights: Vec<usize>,
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new(char_matrix: &Vec<Vec<char>>) -> Self {
        let mut node_heights: Vec<usize> = vec![];
        let (mut start, mut end) = (0, 0);
        let grid_width = char_matrix[0].len();
        for row in char_matrix {
            for c in row {
                let height = match c {
                    'S' => 0,
                    'E' => 25,
                    c => *c as usize - 'a' as usize
                };
                match c {
                    'S' => start = node_heights.len(),
                    'E' => end = node_heights.len(),
                    _ => {}
                }
                node_heights.push(height);
            }
        }
        let edges: Vec<Vec<usize>> = (0..node_heights.len()).map(|i| {
            let mut neighbors = vec![];
            if i % grid_width != 0 {
                neighbors.push(i - 1); //left
            }
            if i % grid_width != grid_width - 1 {
                neighbors.push(i + 1); //right
            }
            if i >= grid_width {
                neighbors.push(i - grid_width);  //up
            }
            if i < node_heights.len() - grid_width {
                neighbors.push(i + grid_width); //down
            }
            neighbors.into_iter()
                .filter(|&n| node_heights[i] + 1 >= node_heights[n]) //reachable
                .collect()
        }).collect();

        Graph { start, end, node_heights, edges }
    }

    fn dijkstra(&self) -> usize {
        let mut distances = vec![usize::MAX; self.node_heights.len()];
        distances[self.start] = 0;
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, self.start))); //Reverse to make it a min heap

        while let Some(Reverse((distance, node))) = queue.pop() {
            if node == self.end {
                break;
            }
            for to in &self.edges[node] {
                let new_distance = distance + 1;
                if new_distance < distances[*to] {
                    distances[*to] = new_distance;
                    queue.push(Reverse((new_distance, *to)));
                }
            }
        }
        distances[self.end]
    }
}
