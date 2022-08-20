use makedeb_srcinfo::SplitPackage;

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
