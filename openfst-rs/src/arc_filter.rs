use crate::arc::Arc;
use std::collections::HashSet;
use std::hash::Hash;

/// A trait for filtering arcs during FST traversal.
pub trait ArcFilter<A: Arc> {
    fn call(&self, arc: &A) -> bool;
}

/// True for all arcs.
#[derive(Debug, Clone, Default)]
pub struct AnyArcFilter;

impl<A: Arc> ArcFilter<A> for AnyArcFilter {
    #[inline(always)]
    fn call(&self, _arc: &A) -> bool {
        true
    }
}

/// True for (input and output) epsilon arcs.
/// Note: In OpenFst, epsilon is represented by the label 0.
#[derive(Debug, Clone, Default)]
pub struct EpsilonArcFilter;

impl<A: Arc<Label = i32>> ArcFilter<A> for EpsilonArcFilter {
    #[inline(always)]
    fn call(&self, arc: &A) -> bool {
        arc.ilabel() == 0 && arc.olabel() == 0
    }
}

/// True for input epsilon arcs.
#[derive(Debug, Clone, Default)]
pub struct InputEpsilonArcFilter;

impl<A: Arc<Label = i32>> ArcFilter<A> for InputEpsilonArcFilter {
    #[inline(always)]
    fn call(&self, arc: &A) -> bool {
        arc.ilabel() == 0
    }
}

/// True for output epsilon arcs.
#[derive(Debug, Clone, Default)]
pub struct OutputEpsilonArcFilter;

impl<A: Arc<Label = i32>> ArcFilter<A> for OutputEpsilonArcFilter {
    #[inline(always)]
    fn call(&self, arc: &A) -> bool {
        arc.olabel() == 0
    }
}

/// True if the specified label matches (or doesn't match) depending on `keep_match`.
#[derive(Debug, Clone)]
pub struct LabelArcFilter<L> {
    label: L,
    match_input: bool,
    keep_match: bool,
}

impl<L> LabelArcFilter<L> {
    /// Matches the specified input label and keeps the match.
    pub fn new(label: L) -> Self {
        Self {
            label,
            match_input: true,
            keep_match: true,
        }
    }

    /// Full constructor specifying all options.
    pub fn with_options(label: L, match_input: bool, keep_match: bool) -> Self {
        Self {
            label,
            match_input,
            keep_match,
        }
    }
}

impl<A: Arc> ArcFilter<A> for LabelArcFilter<A::Label> {
    #[inline]
    fn call(&self, arc: &A) -> bool {
        let match_found = if self.match_input {
            arc.ilabel() == self.label
        } else {
            arc.olabel() == self.label
        };

        if self.keep_match {
            match_found
        } else {
            !match_found
        }
    }
}

/// True if any of the specified labels match (or don't match) depending on `keep_match`.
#[derive(Debug, Clone)]
pub struct MultiLabelArcFilter<L> {
    labels: HashSet<L>,
    match_input: bool,
    keep_match: bool,
}

impl<L: Hash + Eq> Default for MultiLabelArcFilter<L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<L: Hash + Eq> MultiLabelArcFilter<L> {
    /// Matches the specified input labels and keeps the match.
    pub fn new() -> Self {
        Self {
            labels: HashSet::new(),
            match_input: true,
            keep_match: true,
        }
    }

    /// Full constructor specifying all options.
    pub fn with_options(match_input: bool, keep_match: bool) -> Self {
        Self {
            labels: HashSet::new(),
            match_input,
            keep_match,
        }
    }

    /// Adds a label to the filter set.
    pub fn add_label(&mut self, label: L) {
        self.labels.insert(label);
    }
}

impl<A: Arc> ArcFilter<A> for MultiLabelArcFilter<A::Label>
where
    A::Label: Hash + Eq,
{
    #[inline]
    fn call(&self, arc: &A) -> bool {
        let target_label = if self.match_input {
            arc.ilabel()
        } else {
            arc.olabel()
        };

        let match_found = self.labels.contains(&target_label);

        if self.keep_match {
            match_found
        } else {
            !match_found
        }
    }
}
