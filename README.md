# makedeb-srcinfo
`makedeb-srcinfo` is a library for Rust and Python to parse makedeb-style `.SRCINFO` files.

The library itself is written in Rust, and bindings for Python are made possible via [PyO3](https://pyo3.rs) and [Maturin](https://maturin.rs).

[![Build status](https://img.shields.io/drone/build/makedeb/makedeb-srcinfo?logo=drone&label=deploy&server=https%3A%2F%2Fdrone.hunterwittenborn.com)](https://drone.hunterwittenborn.com/makedeb/makedeb-srcinfo/latest)

[![Crates.io](https://img.shields.io/crates/v/makedeb-srcinfo?logo=rust)](https://crates.io/crates/makedeb-srcinfo)
[![Rust docs](https://img.shields.io/docsrs/makedeb-srcinfo?label=rust%20docs&logo=rust)](https://docs.rs/makedeb-srcinfo)

[![PyPI](https://img.shields.io/pypi/v/makedeb-srcinfo?logo=pypi&logoColor=white)](https://pypi.org/project/makedeb-srcinfo/)

## Usage
### Installation
You'll first need to install the library before using it. Installation instructions will change depending on if you're using the Rust or Python library:

#### Rust
```sh
cargo add makedeb-srcinfo
```

#### Python
```python3
pip install makedeb-srcinfo
```

### Using the library
The Rust and Python libraries are designed to look quite similar to each other, both interfaces use the same function/class names, and should only differ in how the languages themselves are designed.

#### Rust
See the documentation on [Docs.rs](https://docs.rs/makedeb-srcinfo) for full usage instructions.

```rust
use makedeb_srcinfo::SrcInfo;
use std::fs;

fn main() {
    let file = fs::read_to_string(".SRCINFO").unwrap();
    let srcinfo = SrcInfo::new(&file).unwrap();
    
    // Get the package base.
    println!("The package base for this package is {}.", srcinfo.get_string("pkgbase").unwrap());

    // Get any dependencies.
    match srcinfo.get_array("makedepends") {
        Some(makedepends) => {
            println!("Found {} build dependencies:", makedepends.len());

            for dep in makedepends {
                println!("- {}", dep);
            };
        },
        None => {
            println!("Found no dependencies.");
        }
    }
}
```

#### Python
Note
Help is currently needed to get Python documentation automatically published on new releases. Please see https://github.com/makedeb/makedeb-srcinfo/issues/3 if you'd like to help.

```python3
#!/usr/bin/env python3
from makedeb_srcinfo import SrcInfo

with open(".SRCINFO") as file:
    data = file.read()

srcinfo = SrcInfo(data)

# Get the package base.
pkgbase = srcinfo.get_string("pkgbase")
print(f"The package base for this package is {pkgbase}.")

# Get any dependencies.
makedepends = srcinfo.get_array("makedepends")

if len(makedepends) == 0:
    print("Found no build dependencies.")
else:
    print(f"Found {len(makedepends)} build dependencies:")

    for dep in makedepends:
        print(f"- {dep}")
```
