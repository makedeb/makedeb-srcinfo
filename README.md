# makedeb-srcinfo
![PyPI](https://img.shields.io/pypi/v/makedeb-srcinfo?color=blue&label=PyPI&logo=pypi)

`makedeb-srcinfo` is a Python library to aid in the parsing of makedeb-styled SRCINFO files.

## Installation
Install the `makedeb-srcinfo` package from PyPI:

```sh
pip install makedeb-srcinfo
```

## Usage

```python3
from makedeb_srcinfo import SrcinfoParser

with open(".SRCINFO", "r") as file:
    data = file.read()

# Parse a SRCINFO file.
srcinfo = SrcinfoParser(data)

# Get all references of a variable (returned in a tuple).
pkgname = srcinfo.get_variable("pkgname")
arch = srcinfo.get_variable("arch")

# Get all references of a variable plus it's extensions (i.e. 'depends' and 'focal_depends').
# Results are returned in a dict with a tuple of (distro, arch) for the key name:
#   {
#      ("focal", None): ("gimp", "krita"),
#      ("focal", "amd64"): ("gcc", "golang-go")
#   }
depends = srcinfo.get_extended_variable("depends")
```
