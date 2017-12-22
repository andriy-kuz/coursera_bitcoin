extern crate consensus_from_trust;
extern crate rand;

use consensus_from_trust::node::*;
use consensus_from_trust::malicious_node::*;
use consensus_from_trust::compliant_node::*;
use consensus_from_trust::transaction::*;
use consensus_from_trust::candidate::*;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // There are four required command line arguments: p_graph (.1, .2, .3),
    // p_malicious (.15, .30, .45), p_txDistribution (.01, .05, .10),
    // and numRounds (10, 20). You should try to test your CompliantNode
    // code for all 3x3x3x2 = 54 combinations.
    let args: Vec<String> = env::args().collect();

    const NODES_COUNT: usize = 100;
    // parameter for random graph: prob. that an edge will exist
    let p_graph = args[1].parse::<f64>().expect("Wrong 'graph' parameter");
    // prob. that a node will be set to be malicious
    let p_malicious = args[2].parse::<f64>().expect("Wrong 'malicious' parameter");
    // probability of assigning an initial transaction to each node
    let p_tx_distribution = args[3]
        .parse::<f64>()
        .expect("Wrong 'tx_distribution' parameter");
    // number of simulation rounds your nodes will run for
    let rounds_count = args[4]
        .parse::<i32>()
        .expect("Wrong 'rounds_count' parameter");

    // pick which nodes are malicious and which are compliant
    let mut nodes: Vec<Box<Node>> = Vec::with_capacity(NODES_COUNT);
    {
        let between = Range::new(0., 1.);
        let mut rng = rand::thread_rng();
        for _ in 0..NODES_COUNT {
            if between.ind_sample(&mut rng) < p_malicious {
                nodes.push(Box::new(MaliciousNode::new(
                    p_graph,
                    p_malicious,
                    p_tx_distribution,
                    rounds_count,
                )))
            } else {
                nodes.push(Box::new(CompliantNode::new(
                    p_graph,
                    p_malicious,
                    p_tx_distribution,
                    rounds_count,
                )))
            }
        }
    }
    // initialize random follow graph
    let mut followees: [[bool; NODES_COUNT]; NODES_COUNT] = [[false; NODES_COUNT]; NODES_COUNT];
    {
        let between = Range::new(0., 1.);
        let mut rng = rand::thread_rng();

        for i in 0..NODES_COUNT {
            for j in 0..NODES_COUNT {
                if i == j {
                    continue;
                }

                if between.ind_sample(&mut rng) < p_graph {
                    followees[i][j] = true;
                }
            }
        }
    }
    // notify all nodes of their followees
    for i in 0..NODES_COUNT {
        // TODO: pass reference to slice
        nodes[i].set_followees(followees[i].to_vec());
    }
    // initialize a set of 500 valid Transactions with random ids
    const TXS_COUNT: usize = 500;
    let mut txs_valid: Vec<usize> = Vec::with_capacity(TXS_COUNT);
    {
        let mut rng = rand::thread_rng();
        for _ in 0..TXS_COUNT {
            let r = rng.gen::<usize>();
            txs_valid.push(r);
        }
    }
    // distribute the 500 Transactions throughout the nodes, to initialize
    // the starting state of Transactions each node has heard. The distribution
    // is random with probability p_txDistribution for each Transaction-Node pair.
    {
        let between = Range::new(0., 1.);
        let mut rng = rand::thread_rng();
        for i in 0..NODES_COUNT {
            let mut txs_pending = HashSet::new();

            for tx_id in &txs_valid {
                if between.ind_sample(&mut rng) < p_tx_distribution {
                    txs_pending.insert(Transaction::new(*tx_id));
                }
            }
            nodes[i].set_pending_txs(txs_pending);
        }
    }
    // Simulate for rounds_count times
    for _ in 0..rounds_count {
        // gather all the proposals into a map. The key is the index of the node receiving
        // proposals. The value is an ArrayList containing 1x2 Integer arrays. The first
        // element of each array is the id of the transaction being proposed and the second
        // element is the index # of the node proposing the transaction.
        let mut all_proposals = HashMap::new();

        for node in 0..NODES_COUNT {
            let proposals = nodes[node].send_to_followers();
            for tx in &proposals {
                if txs_valid.contains(&tx.id()) == false {
                    continue;
                }

                for i in 0..NODES_COUNT {
                    if followees[i][node] == false {
                        continue;
                    }

                    if all_proposals.contains_key(&i) == false {
                        all_proposals.insert(i, HashSet::new());
                    }
                    let candidate = Candidate::new(tx.clone(), node);
                    if let Some(ref mut value) = all_proposals.get_mut(&i) {
                        value.insert(candidate);
                    }
                }
            }
        }
        // Distribute the Proposals to their intended recipients as Candidates
        for node in 0..NODES_COUNT {
            if let Some(value) = all_proposals.remove(&node) {
                nodes[node].reveive_from_followees(value)
            }
        }
    }

    // print results
    for node in 0..NODES_COUNT {
        let txs = nodes[node].send_to_followers();
        println!("Transaction ids that Node {} believes consensus on:", node);
        for tx in txs {
            println!("{}", tx.id());
        }
        println!("***");
        println!("***");
    }
}
