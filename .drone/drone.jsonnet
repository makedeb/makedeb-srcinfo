local UnitTests() = {
    name: "run-tests",
    kind: "pipeline",
    type: "docker",
    trigger: {branch: ["main"]},
    steps: [{
	name: "run-tests",
        image: "proget.makedeb.org/docker/makedeb/makedeb:ubuntu-jammy",
	commands: [
	    "sudo chown 'makedeb:makedeb' ./ -R",
	    ".drone/scripts/setup-pbmpr.sh",
	    "sudo apt install rustup -y",
	    "rustup install stable",
	    "cargo fmt --check",
	    "cargo clippy -- -D warnings",
	    "cargo test"
	]
    }]
};

local DeployCratesIO() = {
    name: "deploy-crates-io",
    kind: "pipeline",
    type: "docker",
    trigger: {branch: ["main"]},
    steps: [{
	name: "deploy-crates-io",
        image: "proget.makedeb.org/docker/makedeb/makedeb:ubuntu-jammy",
	environment: {
	    CARGO_REGISTRY_TOKEN: {from_secret: "crates_api_key"}
	},
	commands: [
	    ".drone/scripts/setup-pbmpr.sh",
	    "sudo apt install rustup -y",
	    "rustup install stable",
	    "cargo publish",
	]
    }]
};

[
    UnitTests(),
    DeployCratesIO()
]

// vim: set sw=4:
