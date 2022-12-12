use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

const INPUT: &str = include_str!("../input/2022/day12.txt");

fn main(){
    let char_matrix = INPUT.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut graph = Graph::new(&char_matrix);
    println!("Part 1: {}", graph.shortest_path());

    //Add a virtual start node
    let virtual_start = graph.nodes.len();
    //Create edges from the start node to all nodes of height 0
    let new_edges = graph.nodes.iter().enumerate()
        .filter(|(_, height)| **height == 0)
        .map(|(index, _)| (virtual_start, index)).collect::<Vec<(usize, usize)>>();
    graph.nodes.push(virtual_start);
    graph.start = virtual_start;
    graph.edges.extend(new_edges);
    println!("Part 2: {}", graph.shortest_path() - 1);
}

struct Graph{
    start: usize,
    end: usize,
    nodes: Vec<usize>,
    edges : Vec<(usize,usize)>
}

impl Graph {
    fn new(char_matrix: &Vec<Vec<char>>) -> Self{
        let mut nodes : Vec<usize> = vec![];
        let (mut start, mut end) = (0, 0);

        let grid_width = char_matrix[0].len();
        for row in char_matrix {
            for c in row {
                let height = match c {
                    'S' => {
                        start = nodes.len();
                        0
                    },
                    'E' => {
                        end = nodes.len();
                        25
                    },
                    c => {
                       *c as usize - 'a' as usize
                    }
                };
                nodes.push(height);
            }
        }
        let mut edges = vec![];
        for i in 0..nodes.len() {
            //left
            if i % grid_width != 0 {
                let left = i - 1;
                if nodes[i] + 1 >= nodes[left]{
                    edges.push((i, left));
                }
            }
            //right
            if i % grid_width != grid_width - 1 {
                let right = i + 1;
                if nodes[i] + 1 >= nodes[right] {
                    edges.push((i, right));
                }
            }
            //up
            if i >= grid_width {
                let up = i - grid_width;
                if nodes[i] + 1 >= nodes[up] {
                    edges.push((i, up));
                }
            }
            //down
            if i < nodes.len() - grid_width {
                let down = i + grid_width;
                if nodes[i] + 1 >= nodes[down] {
                    edges.push((i, down));
                }
            }
        }
        Graph{ start, end, nodes, edges}
    }

    fn shortest_path(&self) -> usize {
        //Dijkstra to find the shortest path for this graph
        let mut distances = vec![usize::MAX; self.nodes.len()];
        distances[self.start] = 0;
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, self.start))); //Reverse to make it a min heap

        while let Some(Reverse((distance, node))) = queue.pop() {
            if node == self.end {
                break;
            }
            for (from, to) in &self.edges {
                if *from == node {
                    let new_distance = distance + 1;
                    if new_distance < distances[*to] {
                        distances[*to] = new_distance;
                        queue.push(Reverse((new_distance, *to)));
                    }
                }
            }
        }
        distances[self.end]
    }
}
