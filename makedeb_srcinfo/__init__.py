class ParsingError(Exception):
    pass

class InputError:
    pass

class SrcinfoParser:
    """
    Class to parse a SRCINFO file.

    This takes in one parameter, which should be a 'str' representation of the SRCINFO file.
    """

    _strings = (
        "pkgbase",
        "pkgdesc",
        "pkgver",
        "pkgrel",
        "epoch",
        "url"
    )

    _arrays = (
        "pkgname",
        "arch",
        "license"
    )

    _extended_strings = (
        "preinst",
        "postinst",
        "prerm",
        "postrm"
    )

    _extended_arrays = (
        "depends",
        "makedepends",
        "checkdepends",
        "optdepends",
        "conflicts",
        "provides",
        "replaces",
        "source",
        "control_fields"
    )

    _extended = (_extended_strings + _extended_arrays)

    _required = (
        "pkgbase",
        "pkgname",
        "pkgver",
        "pkgrel",
        "arch"
    )
    
    def __init__(self, srcinfo_content):
        if type(srcinfo_content) is not str:
            raise TypeError("Expected type 'str'. Pass the content of the SRCINFO file.")

        # Read each line for further processing.
        self._srcinfo_data = {}
        lines = srcinfo_content.splitlines()

        for line_number in range(len(lines)):
            line = lines[line_number]
            
            # Older SRCINFO files contain tabs in some lines.
            line = line.lstrip("\t")

            # Older SRCINFO files also contained blank lines. We want to ignore those.
            if line == "":
                continue
            
            # Continue with processing.
            parts = line.split(" = ")

            if len(parts) == 1:
                raise ParsingError(f"Line {line_number}: Couldn't find ' = ' delimiter.")

            # Line passed parsing test.
            key = parts[0]
            value = " = ".join(parts[1:])

            if self._srcinfo_data.get(key) is None:
                self._srcinfo_data[key] = [value]
            else:
                self._srcinfo_data[key] += [value]

        # Check that we have all required variables.
        for key in self._required:
            if key not in self._srcinfo_data:
                raise ParsingError(f"Couldn't find required '{key}' variable.")

        # Check that string values only appear once.
        for key in self._strings:
            value = self._srcinfo_data.get(key)

            if value is not None and len(value) != 1:
                raise ParsingError(f"Variable '{key}' appeared more than once when it is not allowed to.")

        for base_key in self._extended_strings:
            srcinfo_keys = self._get_variable_with_extensions(base_key)

            for key in srcinfo_keys:
                if len(self._srcinfo_data[key]) != 1:
                    raise ParsingError(f"Variable '{key}' appeared more than once when it is not allowed to.")

    def _get_variable_with_extensions(self, variable):
        if variable not in self._extended:
            raise TypeError(f"Variable '{variable}' doesn't support extensions.")

        matches = []

        for key in self._srcinfo_data:
            if variable in key:
                matches += [key]

        return tuple(set(matches))

    def get_variable(self, variable):
        """
        Get all values for the given key. Data is returned in a tuple, with all matches listed inside as strings.
        """
        return tuple(
            self._srcinfo_data.get(variable, ())
        )

    def get_extended_variable(self, variable):
        """
        Get all values for the given key, including those that are simply the variable with distribution/architecture extensions.

        Data is returned as a dictionary. Each key is in the format 'distro, arch', with each having a value of a tuple containing the matches.

        E.g.:
        {("focal", "amd64"): ("gimp", "krita", "gcc")}

        'distro' and 'arch' will be set to None for any variables that don't contain those relevant extensions.

        Raises a ParsingError when the given key isn't supported with extensions.
        """

        returned_matches = {}
        matches = self._get_variable_with_extensions(variable)

        for match in matches:
            distro = None
            arch = None

            parts = match.split(variable)

            for part in parts:
                if part.endswith("_"):
                    distro = part.rstrip("_")
                elif part.startswith("_"):
                    arch = part.lstrip("_")

            returned_matches[distro, arch] = self.get_variable(match)

        return returned_matches

    def construct_extended_variable_name(self, distro, var, arch):
        if distro and arch:
            return f"{distro}_{var}_{arch}"
        elif distro:
            return f"{distro}_{var}"
        elif arch:
            return f"{var}_{arch}"
        else:
            return var

    def split_dep_description(self, dep):
        parts = dep.split(": ")
        dep = parts[0]
        desc = ": ".join(parts[1:])

        return (dep, desc)

    def split_dep_condition(self, dep):
        conditions = ("<=", ">=", "=", "<", ">")
        found_condition = None

        for condition in conditions:
            if condition in dep:
                found_condition = condition
                break

        if found_condition is None:
            return (dep, None, None)

        dep, version = dep.split(found_condition)

        return (dep, found_condition, version)
