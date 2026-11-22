//! Integration tests for sigmaos-devtools.

use sigmaos_devtools::*;

#[test]
fn test_chroot_plan() {
    let plan = plan_chroot_build(&ChrootSpec::default()).unwrap();
    assert!(!plan.mounts.is_empty());
}

#[test]
fn test_commit_plan() {
    let p = plan_commit("PKGBUILD", "extra", "msg", true).unwrap();
    assert!(p.steps.contains(&CommitStep::Signoff));
}

#[test]
fn test_db_update_plan() {
    let staged = vec![RepoPackage { filename: "a-1.pkg.tar.zst".into(), name: "a".into(), version: "1".into() }];
    let plan = plan_db_update(RepoName::Core, &staged, &[]);
    assert_eq!(plan.added.len(), 1);
}
