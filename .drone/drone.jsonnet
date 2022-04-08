local publishPypi() = {
    name: "publish-pypi",
    kind: "pipeline",
    type: "docker",
    trigger: {branch: ["main"]},

    steps: [{
        name: "publish-pypi",
        image: "python:3",
        environment: {pypi_api_key: {from_secret: "pypi_api_key"}},
        commands: [
            "pip install build twine",
            "python3 -m build",
            "python3 -m twine upload -u '__token__' -p \"$${pypi_api_key}\" --non-interactive dist/*"
        ]
    }]
};

[publishPypi()]

// vim: set sw=4 expandtab:
