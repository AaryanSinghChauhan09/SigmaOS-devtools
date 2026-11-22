//! SigmaOS-devtools component.
//!
//! Rust implementation of `archlinux/devtools`: Arch Linux developer
//! tools (makechrootpkg, commitpkg, extra-x86_64-build, db-update,
//! ldidd, ...).
//!
//! This port implements the orchestration cores: chroot build
//! environments (bind mounts, clean chroots), repo database updates,
//! and package commit/sign/push workflows — the logic upstream's
//! shell/Python scripts implement.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod chroot;
pub mod commit;
pub mod dbupdate;
pub mod error;

pub use chroot::{BuildPlan, ChrootSpec, ChrootStep, plan_chroot_build};
pub use commit::{CommitPlan, CommitStep, plan_commit};
pub use dbupdate::{DbUpdatePlan, RepoName, RepoPackage, plan_db_update};
pub use error::DevtoolsError;
