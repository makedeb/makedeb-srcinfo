use makedeb_srcinfo::{SplitDependency, SplitPackage};

#[test]
fn split_version() {
    let ver1 = SplitPackage::new("pkg1");
    let ver2 = SplitPackage::new("pkg2=1.0");
    let ver3 = SplitPackage::new("pkg3>=1.0=1.3");

    assert!(ver1.pkgname == "pkg1");
    assert!(ver1.operator.is_none());
    assert!(ver1.version.is_none());

    assert!(ver2.pkgname == "pkg2");
    assert!(ver2.operator.as_ref().unwrap() == "=");
    assert!(ver2.version.as_ref().unwrap() == "1.0");

    assert!(ver3.pkgname == "pkg3");
    assert!(ver3.operator.as_ref().unwrap() == ">=");
    assert!(ver3.version.as_ref().unwrap() == "1.0=1.3");
}

#[test]
fn split_dependency() {
    let ver1 = SplitDependency::new("pkg1");
    let ver2 = SplitDependency::new("pkg2=1.0|pkg4");
    let ver3 = SplitDependency::new("pkg3>=1.0=1.3|pkg5=5|pkg6");

    assert_eq!(ver1.as_control(), "pkg1");
    assert_eq!(ver2.as_control(), "pkg2 (= 1.0) | pkg4");
    assert_eq!(ver3.as_control(), "pkg3 (>= 1.0=1.3) | pkg5 (= 5) | pkg6");
}
