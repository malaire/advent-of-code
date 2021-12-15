use ndarray::Array2;
use petgraph::{algo::dijkstra, Graph};

use malaire_aoc::{read_byte_array, run};

static INPUT_A: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

static INPUT_X: &str = include_str!("input");

fn main() {
    run(0, solve_1, INPUT_A, 40);
    run(1, solve_1, INPUT_X, 702);

    run(0, solve_2, INPUT_A, 315);
    run(2, solve_2, INPUT_X, 2955);
}

fn solve_1(input: &str) -> usize {
    find_shortest(read_byte_array(input).map(|x| (x - b'0') as usize))
}

fn solve_2(input: &str) -> usize {
    let block = read_byte_array(input).map(|x| (x - b'0') as usize);

    let mut risks = Array2::zeros((block.nrows() * 5, block.ncols() * 5));
    for (index, risk) in risks.indexed_iter_mut() {
        let delta = (index.0 / block.nrows()) + (index.1 / block.ncols());
        *risk = (block[(index.0 % block.nrows(), index.1 % block.ncols())] + delta - 1) % 9 + 1;
    }

    find_shortest(risks)
}

fn find_shortest(risks: Array2<usize>) -> usize {
    let mut graph = Graph::new();
    let nodes = risks.map(|_| graph.add_node(()));

    for (index, &risk) in risks.indexed_iter() {
        if index.0 > 0 {
            let prev = (index.0 - 1, index.1);
            graph.add_edge(nodes[prev], nodes[index], risk);
            graph.add_edge(nodes[index], nodes[prev], risks[prev]);
        }

        if index.1 > 0 {
            let prev = (index.0, index.1 - 1);
            graph.add_edge(nodes[prev], nodes[index], risk);
            graph.add_edge(nodes[index], nodes[prev], risks[prev]);
        }
    }

    *dijkstra(&graph, nodes[(0, 0)], None, |e| *e.weight())
        .get(&nodes[(nodes.nrows() - 1, nodes.ncols() - 1)])
        .unwrap()
}
