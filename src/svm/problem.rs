
use vectors::SimdOptimized;
use svm::SVM;
use util;


/// A single problem we should classify.
#[derive(Debug)]
pub struct Problem {
    /// A vector of `num_attributes` features.
    pub features: Vec<f32>,

    /// Kernel values. A vector for each class.
    pub kernel_values: SimdOptimized<f64>,

    /// All votes for a given class label.
    pub vote: Vec<u32>,

    /// Decision values.
    pub decision_values: Vec<f64>,

    /// Computed label. This is what we update eventually.
    pub label: u32,
}




impl Problem {
    /// Creates a new problem with the given parameters.
    pub fn with_dimension(total_sv: usize, num_classes: usize, num_attributes: usize) -> Problem {
        let num_decision_values = num_classes * (num_classes - 1) / 2;
        let preferred_attributes = util::prefered_simd_size(num_attributes);

        Problem {
            features: vec![Default::default(); preferred_attributes],
            kernel_values: SimdOptimized::with_dimension(num_classes, total_sv, Default::default()),
            decision_values: vec![Default::default(); num_decision_values],
            vote: vec![Default::default(); num_classes],
            label: 0,
        }
    }

    /// Creates a new problem for the given SVM.
    pub fn from_svm<T>(svm: &SVM<T>) -> Problem {
        Problem::with_dimension(
            svm.num_total_sv,
            svm.classes.len(),
            svm.num_attributes,
        )
    }
}




