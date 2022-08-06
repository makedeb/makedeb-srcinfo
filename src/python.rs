use crate::SrcInfo as RustSrcInfo;
use pyo3::{create_exception, exceptions::PyException, prelude::*};

// Exceptions
create_exception!(
    makedeb_srcinfo,
    ParserError,
    PyException,
    "A class representing the output of a parsing error."
);

#[pyclass]
struct SrcInfo {
    srcinfo: RustSrcInfo,
}

#[pymethods]
impl SrcInfo {
    /// Parse the `.SRCINFO` file, raising a :class:`~makedeb_srcinfo.ParserError` exception if there was an issue parsing the
    /// file.
    ///
    /// `content` should be a string representing the content of the `.SRCINFO` file.
    #[new]
    fn new(content: String) -> PyResult<Self> {
        match RustSrcInfo::new(&content) {
            Ok(srcinfo) => Ok(SrcInfo { srcinfo }),
            Err(err) => {
                let msg: String;

                if let Some(line_num) = err.line_num {
                    msg = format!("[Line {}] {}", line_num, err.msg);
                } else {
                    msg = err.msg;
                }

                let py_err = ParserError::new_err(msg);
                Err(py_err)
            }
        }
    }

    /// Get a value for anything that's a string variable in a PKGBUILD.
    ///
    /// **Note** that you'll need to use :func:`~makedeb_srcinfo.SrcInfo.get_array` if you want to get the `pkgname` variable, since that has the ability to be more than one item.
    ///
    /// This function also accepts extended variables (i.e. `focal_postrm`), though only variables that can be
    /// extended by makedeb are supported.
    ///
    /// Returns the the value of the variable if it can be found, otherwise :class:`None` is returned.
    pub fn get_string(&self, key: &str) -> Option<String> {
        self.srcinfo.get_string(key).cloned()
    }

    /// Get a value for anything that's an array variable in a PKGBUILD.
    ///
    /// This function also accepts extended variables (i.e. `focal_depends`), though only variables that can be
    /// extended by makedeb are supported.
    ///
    /// Returns a list of values if the variable can be found, otherwise :class:`None` is returned.
    pub fn get_array(&self, key: &str) -> Option<Vec<String>> {
        self.srcinfo.get_array(key).cloned()
    }

    /// Get the extended names (as well as the key itself) for a variable. Use this if you need a variable as well as any                          
    /// same variable that contains distribution and architecture extensions.
    ///
    /// If `key` isn't a key makedeb supports for variable name extensions, this will return :class:`None`, regardless of if the base key is in the `.SRCINFO` file or not.
    ///
    /// This returns a list of strings that can be then passed into :func:`~makedeb_srcinfo.SrcInfo.get_string` and
    /// :func:`~makedeb_srcinfo.SrcInfo.get_array`.
    pub fn get_extended_values(&self, key: &str) -> Option<Vec<String>> {
        self.srcinfo.get_extended_values(key)
    }
}

#[pymodule]
fn makedeb_srcinfo(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<SrcInfo>()?;
    Ok(())
}
