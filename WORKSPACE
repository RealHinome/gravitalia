workspace(name = "gravitalia")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "814680e1ab535f799fd10e8739ddca901351ceb4d2d86dd8126c22d36e9fcbd9",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.29.0/rules_rust-v0.29.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    versions=["1.72.1"],
    
    edition = "2021",
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:graphql/cargo-bazel-lock.json",
    manifests = [
        "//:Cargo.toml",
        "//graphql:Cargo.toml",
    ],
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()