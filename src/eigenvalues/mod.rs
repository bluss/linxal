//! Contains methods for solving eigenvalues, including general and
//! symmetric/Hermitian eigenvalue problems.
//!
//! The eigenvalue problem is to find solution $\left(x, \lambda\right)$ to the problem:
//!
//! $$A \cdot x = \lambda \cdot x$$
//!
//! for a square matrix \\(A\\).

pub mod general;
pub mod symmetric;
pub mod types;

pub use self::general::{Eigen};
pub use self::types::{EigenError};
pub use self::symmetric::{SymEigen};
