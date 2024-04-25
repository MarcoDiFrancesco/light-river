use crate::classification::alias::FType;
use crate::classification::mondrian_tree::MondrianTree;
use crate::common::{ClassifierOutput, ClassifierTarget, Observation};
use crate::stream::data_stream::Data;
use core::iter::zip;
use ndarray::array;
use ndarray::{arr1, Array1, Array2};
use ndarray::{ArrayBase, Dim, ScalarOperand, ViewRepr};
use num::pow::Pow;
use num::traits::float;
use num::{Float, FromPrimitive};
use rand::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::env::consts;
use std::iter::FlatMap;
use std::ops::{Add, Div, Mul, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use std::rc::Rc;
use std::rc::Weak;
use std::{cmp, mem, usize};

pub struct MondrianForest<F: FType> {
    trees: Vec<MondrianTree<F>>,
    labels: Vec<String>,
}
impl<F: FType> MondrianForest<F> {
    pub fn new(
        window_size: usize,
        n_trees: usize,
        features: &Vec<String>,
        labels: &Vec<String>,
    ) -> Self {
        let tree_default = MondrianTree::new(window_size, features, labels);
        let trees = vec![tree_default; n_trees];
        let labels = labels.clone();
        MondrianForest::<F> { trees, labels }
    }

    /// Note: In Nel215 codebase should work on multiple records, here it's
    /// working only on one.
    ///
    /// Function in River/LightRiver: "learn_one()"
    pub fn partial_fit(&mut self, x: &Array1<F>, y: &String) {
        for tree in &mut self.trees {
            tree.partial_fit(x, y);
            println!("treeee {}", tree);
        }
    }

    pub fn fit(x: &HashMap<String, f32>, y: &String) {
        unimplemented!()
    }

    pub fn predict_proba(&self, x: &Array1<F>) -> Array1<F> {
        // scores shape in nel215: (n_trees, n_samples, n_labels)
        // scores shape here: (n_trees, n_labels). We are doing one shot learning.
        let n_trees = self.trees.len();
        let n_labels = self.labels.len();

        // Initialize an accumulator array for summing probabilities from each tree
        let mut total_probs = Array1::<F>::zeros(n_labels);

        // Sum probabilities from each tree
        for tree in &self.trees {
            let probs = tree.predict_proba(x);
            total_probs += &probs; // Assuming `probs` is an Array1<F>
        }

        // Average the probabilities by the number of trees
        total_probs /= F::from_usize(n_trees).unwrap();

        total_probs
    }
}
