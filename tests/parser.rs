use makedeb_srcinfo::SrcInfo;

#[test]
#[rustfmt::skip]
fn valid() {
    let file = include_str!("files/VALID.SRCINFO");
    let srcinfo = SrcInfo::new(&file).unwrap();

    // Checks for extended variables.
    let extended_values = srcinfo.get_extended_values("postrm").unwrap();

    assert!(extended_values.len() == 4);
    assert!(extended_values.contains(&"postrm".to_owned()));
    assert!(extended_values.contains(&"focal_postrm".to_owned()));
    assert!(extended_values.contains(&"postrm_amd64".to_owned()));
    assert!(extended_values.contains(&"focal_postrm_amd64".to_owned()));

    assert!(srcinfo.get_string("postrm").unwrap() == "file");
    assert!(srcinfo.get_string("focal_postrm").unwrap() == "focal_file");
    assert!(srcinfo.get_string("postrm_amd64").unwrap() == "file_amd64");
    assert!(srcinfo.get_string("focal_postrm_amd64").unwrap() == "focal_file_amd64");
}

#[test]
fn no_value() {
    let file = include_str!("files/NO_VALUE.SRCINFO");
    let srcinfo = SrcInfo::new(&file);
    let err = srcinfo.unwrap_err();

    assert!(err.line_num.unwrap() == 4);
}
