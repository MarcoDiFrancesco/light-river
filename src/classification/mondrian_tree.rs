// use ndarray::Array1;
use num::pow::Pow;
use num::traits::float;
use rand::prelude::*;

use num::{Float, FromPrimitive};
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::env::consts;
use std::iter::FlatMap;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use std::rc::Rc;
use std::{mem, usize};

use crate::common::{ClassifierOutput, ClassifierTarget, Observation};
use crate::stream::data_stream::Data;

trait FType:
    Float + FromPrimitive + AddAssign + SubAssign + MulAssign + DivAssign + std::fmt::Debug
{
}
impl<T> FType for T where
    T: Float + FromPrimitive + AddAssign + SubAssign + MulAssign + DivAssign + std::fmt::Debug
{
}

/// Stats assocociated to one node
/// Vecotor are the labels
struct Stats {
    sum: Vec<f64>,
    sq_sum: Vec<f64>,
    count: Vec<i32>,
}
impl Stats {
    fn create_result() {
        unimplemented!()
    }
    fn add() {
        unimplemented!()
    }
    fn merge() {
        unimplemented!()
    }
    fn predict_proba() {
        unimplemented!()
    }
}

/// Node struct
#[derive(Debug, Clone)]
struct Node<F> {
    parent: Option<usize>,
    tau: F, // Time parameter: updated during 'node creation' or 'node update'
    is_leaf: bool,
    min_list: Vec<F>, // Lists representing the minimum and maximum values of the data points contained in the current node
    max_list: Vec<F>,
    delta: F, // Dimension in which a split occurs (?)
    xi: F,    // Split point along the dimension specified by delta
    left: Option<usize>,
    right: Option<usize>,
    // stats: Stats, // Ignoring stats for now since it should be a fixed-size array, vector should not work since we are using fixed-size arrays in Trees, but try it out and see what comes out
}
impl<F: FType> Node<F> {
    pub fn update_leaf(&self) {
        unimplemented!()
    }
    pub fn update_internal(&self) {
        unimplemented!()
    }
    pub fn get_parent_tau(&self, parent: Option<&Node<F>>) -> F {
        panic!(
            "Not implemented, adds a lot of complexity for no reason. Just extract tau directly."
        )
        // match self.parent {
        //     Some(_) => parent.tau,
        //     None => F::from_f32(0.0).unwrap(),
        // }
    }
}

struct Trees<F: FType> {
    nodes: Vec<Node<F>>,
}
impl<F: FType> Trees<F> {
    fn new(
        n_trees: usize,
        height: usize,
        features: &Vec<String>,
        rng: &mut ThreadRng,
        n_nodes: usize,
    ) -> Self {
        if n_trees != 1 {
            unimplemented!("Only implemented for 1 tree. This code has to be heavily restructured for multiple trees.");
        }

        // e.g. [0.0, 0.0, 0.0, ...]
        let min_list: Vec<F> = features.iter().map(|_| F::from_f64(0.0).unwrap()).collect();
        let max_list: Vec<F> = features.iter().map(|_| F::from_f64(0.0).unwrap()).collect();

        let node_default = Node::<F> {
            parent: None,
            tau: F::from_f64(0.33).unwrap(),
            is_leaf: false,
            min_list,
            max_list,
            delta: F::from_f64(0.123).unwrap(),
            xi: F::from_f64(0.456).unwrap(),
            left: None,
            right: None,
            // stats: Stats::new,
        };
        let mut nodes = vec![node_default; n_nodes];

        // For each node assign indicies of: parent, and left/right child
        for i in 0..n_nodes {
            let left_idx = 2 * i + 1;
            let right_idx = 2 * i + 2;

            if (left_idx < n_nodes) && (right_idx < n_nodes) {
                nodes[i].left = Some(left_idx);
                nodes[i].right = Some(right_idx);
                nodes[left_idx].parent = Some(i);
                nodes[right_idx].parent = Some(i);
            } else {
                nodes[i].is_leaf = true;
            }
        }

        Trees { nodes }
    }
}

pub struct MondrianTree<F: FType> {
    window_size: usize,
    n_trees: usize,
    height: usize,
    features: Vec<String>,
    rng: ThreadRng,
    n_nodes: usize,
    trees: Trees<F>,
    first_learn: bool,
    // pos_val: ClassifierTarget,
}
impl<F: FType> MondrianTree<F> {
    pub fn new(
        window_size: usize,
        n_trees: usize,
        height: usize,
        features: &Vec<String>,
        // pos_val: ClassifierTarget,
    ) -> Self {
        let features_clone = features.clone();
        let mut rng = rand::thread_rng();
        // #nodes = 2 ^ height - 1
        let n_nodes = usize::pow(2, height.try_into().unwrap()) - 1;
        // TODO: this is only 1 tree, implement later for mulpile ones
        let mut trees = Trees::new(n_trees, height, &features, &mut rng, n_nodes);
        MondrianTree::<F> {
            window_size,
            n_trees,
            height,
            features: features_clone,
            rng,
            n_nodes,
            trees,
            first_learn: false,
            // pos_val,
        }
    }

    fn create_leaf(&self) {
        unimplemented!()
    }

    /// Note: In Nel215 codebase should work on multiple records, here it's
    /// working only on one, so it's the same as "predict()".
    pub fn predict_proba(
        &mut self,
        // x: &HashMap<String, f32>,
        x: &Vec<F>,
        y: &ClassifierTarget,
    ) -> ClassifierOutput<F> {
        self.predict(x, 0, 1.0)
    }

    fn extend_mondrian_block(&self) {
        println!("WARNING: extend_mondrian_block not implemented")
    }

    /// Note: In Nel215 codebase should work on multiple records, here it's
    /// working only on one.
    ///
    /// Function in River/LightRiver: "learn_one()"
    pub fn partial_fit(&mut self, x: &HashMap<String, f32>, y: &ClassifierTarget) {
        // No need to check if node is root, the full tree is already built
        self.extend_mondrian_block();
    }

    fn fit(&self) {
        unimplemented!()
    }

    /// Function in River/LightRiver: "score_one()"
    ///
    /// Recursive function to predict probabilities.
    /// - `x`: Input data.
    /// - `node_idx`: Current node index in the tree.
    /// - `p_not_separated_yet`: Probability that `x` has not been separated by any split in the tree up to this node.
    fn predict(
        &self,
        x: &Vec<F>,
        node_idx: usize,
        p_not_separated_yet: f32,
    ) -> ClassifierOutput<F> {
        let node = &self.trees.nodes[node_idx];
        println!("Node: {:?}", node);

        // Step 1: Calculate the time delta from the parent node.
        // If node is root its time is 0
        let parent_tau: F = match node.parent {
            Some(_) => self.trees.nodes[node.parent.unwrap()].tau,
            None => F::from_f32(0.0).unwrap(),
        };
        let d = node.tau - parent_tau;
        println!("Time delta {:?}", d);

        // Step 2: Compute the distance `eta` of `x` from the node's data boundaries.
        // let max_list_arr = Array1::<F>::from_vec(node.max_list);
        // let x_arr = Array1::<F>::from_vec(x);
        // let eta_tmp = node.max_list - x;

        // println!("eta_tmp: {eta_tmp}");

        // let eta = x.iter().map(|(k, v)| {
        //     let max_dist = F::max(node.max_list.get(k).unwrap_or(&F::zero()) - *v, F::zero());
        //     let min_dist = F::max(*v - node.min_list.get(k).unwrap_or(&F::zero()), F::zero());
        //     max_dist + min_dist
        // }).sum();

        // // Step 3: Calculate the probability `p` of not being separated by new splits.
        // let p = F::one() - (-d * eta).exp();

        // // Step 4: Generate a result for the current node using its statistics.
        // let result = node.stat.create_result(x, p_not_separated_yet * p);

        // // Step 5: If the node is a leaf, calculate the final weight and return the merged result.
        // if node.is_leaf {
        //     let w = p_not_separated_yet * (F::one() - p);
        //     return result.merge(node.stat.create_result(x, w));
        // }

        // // Step 6: Determine the appropriate child node based on the split condition and recurse.
        // let child_idx = if x.get(&node.delta) <= Some(&node.xi) {
        //     node.left.expect("Left child node index missing")
        // } else {
        //     node.right.expect("Right child node index missing")
        // };
        // let child_result = self._predict(x, child_idx, p_not_separated_yet * (F::one() - p));

        // // Step 7: Merge the results from the current node and its child.
        // result.merge(child_result)

        ClassifierOutput::Probabilities(HashMap::from([(
            ClassifierTarget::from("target-example"),
            F::one(),
        )]))
    }

    fn get_params(&self) {
        unimplemented!()
    }
}
