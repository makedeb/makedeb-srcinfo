//! This library provides a standardized way for clients to parse makedeb-styled `.SRCINFO` files.
//! These are the files found on the [MPR](https://mpr.makedeb.org) that provide a method to know
//! the contents of a PKGBUILD file without having to `source` (and thefore execute) it.
//!
//! Most clients won't need to use any of the `SRCINFO_*` constants, but instead should use the
//! [`SrcInfo`] struct to read a `.SRCINFO` file.
use regex::Regex;
use std::collections::HashMap;

// Python bindings.
mod python;

/// A list of items that should always be strings (i.e. a maximum of one can be present) in a `.SRCINFO` file.
pub const SRCINFO_STRINGS: [&str; 10] = [
    "pkgbase", "pkgdesc", "pkgver", "pkgrel", "epoch", "url", "preinst", "postinst", "prerm",
    "postrm",
];

/// A list of items that should always be arrays (i.e. any amount can be present) in a `.SRCINFO` file.
pub const SRCINFO_ARRAYS: [&str; 19] = [
    "pkgname",
    "arch",
    "license",
    "depends",
    "makedepends",
    "checkdepends",
    "optdepends",
    "conflicts",
    "provides",
    "replaces",
    "source",
    "control_fields",
    "md5sums",
    "sha1sums",
    "sha224sums",
    "sha256sums",
    "sha384sums",
    "sha512sums",
    "b2sums",
];

/// A list of items that can be extended (e.g. prefixed with `focal_` or suffixed with `_amd64`) in
/// a `.SRCINFO` file.
pub const SRCINFO_EXTENDED: [&str; 20] = [
    // Strings
    "preinst",
    "postinst",
    "prerm",
    "postrm",
    // Arrays
    "depends",
    "makedepends",
    "checkdepends",
    "optdepends",
    "conflicts",
    "provides",
    "replaces",
    "source",
    "control_fields",
    "md5sums",
    "sha1sums",
    "sha224sums",
    "sha256sums",
    "sha384sums",
    "sha512sums",
    "b2sums",
];

/// A list of items that must always be present inside of a `.SRCINFO` file.
pub const SRCINFO_REQUIRED: [&str; 5] = ["pkgbase", "pkgname", "pkgver", "pkgrel", "arch"];

/// A struct representing the output of a parsing error.
#[derive(Debug)]
pub struct ParserError {
    /// A message describing the parsing error.
    pub msg: String,
    /// The line number the error occured on. This will always be the [`Some`] variant unless there
    /// was an issue with the file as a whole, in which case the [`None`] variant will be returned.
    pub line_num: Option<usize>,
}

type ParseMap = HashMap<String, Vec<String>>;

#[derive(Debug)]
pub struct SrcInfo {
    map: ParseMap,
}

impl SrcInfo {
    /// Parse the `.SRCINFO` file, returning a [`ParserError`] if there was an issue parsing the
    /// file.
    ///
    /// `content` should be a string representing the content of the `.SRCINFO` file.
    pub fn new(content: &str) -> Result<Self, ParserError> {
        let mut map: ParseMap = HashMap::new();

        for (_index, _line) in content.lines().enumerate() {
            let mut line = _line.to_owned();

            // We'll use the index for error reporting. Line numbers start at one in a file while
            // indexes start at zero, so increment the index by one.
            let index = _index + 1;

            // Arch Linux .SRCINFO files sometimes contain comment lines while makedeb's do not, so
            // we want to ignore those.
            if line.starts_with('#') {
                continue;
            }

            // Arch Linux .SRCINFO files also contain some blank lines which are lacking in
            // makedeb's style, so ignore those too.
            if line.is_empty() {
                continue;
            }

            // Older .SRCINFO files contain tabs in some lines. We still want to parse those lines
            // and the only problem is the tab, so just remove it.
            line = line.replace('\t', "");

            // Split the line between its key and value.
            let _parts = line.split(" = ");

            if _parts.clone().count() < 2 {
                return Err(ParserError {
                    msg: "No ' = ' delimiter found.".to_string(),
                    line_num: Some(index),
                });
            }

            let parts: Vec<&str> = _parts.collect();
            let key = parts[0].to_string();
            let value = parts[1..].join(" = ");

            if let Some(values) = map.get_mut(&key) {
                values.push(value);
            } else {
                map.insert(key, vec![value]);
            }
        }

        // Make sure we have all required keys present.
        for item in SRCINFO_REQUIRED {
            if !map.contains_key(&item.to_owned()) {
                return Err(ParserError {
                    msg: format!("Required key '{}' not found.", item),
                    line_num: None,
                });
            }
        }

        // Make sure any item that's supposed to be a string only has one item present.
        // TODO: Also do this for any SRCINFO_STRINGS also in SRCINFO_EXTENDED.
        for item in SRCINFO_STRINGS {
            if let Some(values) = map.get(&item.to_owned()) {
                if values.len() > 1 {
                    return Err(ParserError {
                        msg: format!(
                            "Key '{}' is present more than once when it is not allowed to.",
                            item
                        ),
                        line_num: None,
                    });
                }
            }
        }

        Ok(Self { map })
    }

    /// Convert an extended string to it's base form.
    /// This returns "" if the string isn't a valid key for a `.SRCINFO` file. While this could
    /// return a [`None`] variant, this makes it easier to integrate in other places it's used
    /// in this lib.
    ///
    /// This function is also not public (!) so we can have trash design decisions like this.
    fn get_base_key(item: &str) -> &str {
        let mut keys = SRCINFO_STRINGS.to_vec();
        keys.append(&mut SRCINFO_ARRAYS.to_vec());

        if keys.contains(&item) {
            return item;
        }

        for key in keys {
            let re_key = format!("^{0}_|_{0}_|_{0}$", key);
            let re = Regex::new(&re_key).unwrap();

            if re.is_match(item) {
                return key;
            }
        }

        ""
    }

    /// Get a value for anything that's a string variable in a PKGBUILD.
    ///
    /// **Note** that you'll need to use [`SrcInfo::get_array`] if you want to get the `pkgname` variable, since that has the
    /// ability to be more than one item.
    ///
    /// This function also accepts extended variables (i.e. `focal_postrm`), though only variables that can be
    /// extended by makedeb are supported.
    ///
    /// Returns the [`Some`] variant if the variable can be found, otherwise the [`None`] variant is returned.
    pub fn get_string(&self, key: &str) -> Option<&String> {
        if !SRCINFO_STRINGS.contains(&SrcInfo::get_base_key(key)) {
            return None;
        }

        if let Some(values) = self.map.get(&key.to_owned()) {
            Some(&values[0])
        } else {
            None
        }
    }

    /// Get a value for anything that's an array variable in a PKGBUILD.
    ///
    /// This function also accepts extended variables (i.e. `focal_depends`), though only variables that can be
    /// extended by makedeb are supported.
    ///
    /// Returns the [`Some`] variant if the variable can be found, otherwise the [`None`] variant is returned.
    pub fn get_array(&self, key: &str) -> Option<&Vec<String>> {
        if !SRCINFO_ARRAYS.contains(&SrcInfo::get_base_key(key)) {
            return None;
        }

        self.map.get(&key.to_owned())
    }

    /// Get the extended names (as well as the key itself) for a variable. Use this if you need a variable as well as any
    /// same variable that contains distribution and architecture extensions.
    ///
    /// If `key` isn't a key makedeb supports for variable name extensions, this will return the [`None`] variant, regardless
    /// of if the base key is in the `.SRCINFO` file or not.
    ///
    /// This returns a vector of strings that can be then passed into [`SrcInfo.get_string`] and
    /// [`SrcInfo.get_array`].
    pub fn get_extended_values(&self, key: &str) -> Option<Vec<String>> {
        if !SRCINFO_EXTENDED.contains(&key) {
            return None;
        }

        let mut matches: Vec<String> = Vec::new();
        let re = Regex::new(&format!(".*_{0}$|.*_{0}_.*|^{0}.*|^{0}$", key)).unwrap();

        for item in self.map.keys() {
            if re.is_match(item) {
                matches.push(item.clone());
            }
        }

        // If no items are in our vector, then no variants of the key were in the `.SRCINFO` file,
        // and we want to let the client know no matches were found.
        if matches.is_empty() {
            None
        } else {
            Some(matches)
        }
    }
}
