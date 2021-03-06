//! Common traits, structures, and macros for most user-end applications

pub use svd::general::SVD;
pub use svd::types::{SVDError, SVDSolution, SingularValue};
pub use eigenvalues::general::Eigen;
pub use eigenvalues::types::EigenError;
pub use eigenvalues::symmetric::SymEigen;
pub use types::{Symmetric, Error, c32, c64, Magnitude};
pub use solve_linear::general::SolveLinear;
pub use solve_linear::symmetric::SymmetricSolveLinear;
pub use least_squares::LeastSquares;

pub use util::external::*;
