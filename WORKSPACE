workspace(name = "gravitalia")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# Add rules_proto
http_archive(
    name = "rules_proto",
    sha256 = "dc3fb206a2cb3441b485eb1e423165b231235a1ea9b031b4433cf7bc1fa460dd",
    strip_prefix = "rules_proto-5.3.0-21.7",
    urls = [
        "https://github.com/bazelbuild/rules_proto/archive/refs/tags/5.3.0-21.7.tar.gz",
    ],
)
load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies", "rules_proto_toolchains")

rules_proto_dependencies()

rules_proto_toolchains()

# Add rules_rust
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

# Add rules_nodejs
http_archive(
    name = "aspect_rules_js",
    sha256 = "a949d56fed8fa0a8dd82a0a660acc949253a05b2b0c52a07e4034e27f11218f6",
    strip_prefix = "rules_js-1.33.1",
    url = "https://github.com/aspect-build/rules_js/releases/download/v1.33.1/rules_js-v1.33.1.tar.gz",
)

load("@aspect_rules_js//js:repositories.bzl", "rules_js_dependencies")

rules_js_dependencies()

load("@rules_nodejs//nodejs:repositories.bzl", "nodejs_register_toolchains")

nodejs_register_toolchains(
    name = "nodejs",
    node_version = "18.8.0",
)

load("@aspect_rules_js//npm:repositories.bzl", "npm_translate_lock")

npm_translate_lock(
    name = "npm",
    npmrc = "//:.npmrc",
    pnpm_lock = "//:pnpm-lock.yaml",
    verify_node_modules_ignored = "//:.bazelignore",
    dev = True,
)

load("@npm//:repositories.bzl", "npm_repositories")

npm_repositories()

# Add rules_ts
http_archive(
    name = "aspect_rules_ts",
    sha256 = "8aabb2055629a7becae2e77ae828950d3581d7fc3602fe0276e6e039b65092cb",
    strip_prefix = "rules_ts-2.0.0",
    url = "https://github.com/aspect-build/rules_ts/releases/download/v2.0.0/rules_ts-v2.0.0.tar.gz",
)

load("@aspect_rules_ts//ts:repositories.bzl", "LATEST_TYPESCRIPT_VERSION", "rules_ts_dependencies")

rules_ts_dependencies(
    ts_version = LATEST_TYPESCRIPT_VERSION,
)

# Add rules_oci
http_archive(
    name = "rules_oci",
    sha256 = "f6125c9a123a2ac58fb6b13b4b8d4631827db9cfac025f434bbbefbd97953f7c",
    strip_prefix = "rules_oci-0.3.9",
    url = "https://github.com/bazel-contrib/rules_oci/releases/download/v0.3.9/rules_oci-v0.3.9.tar.gz",
)

load("@rules_oci//oci:dependencies.bzl", "rules_oci_dependencies")

rules_oci_dependencies()

load("@rules_oci//oci:repositories.bzl", "LATEST_CRANE_VERSION", "oci_register_toolchains")

oci_register_toolchains(
    name = "oci",
    crane_version = LATEST_CRANE_VERSION,
)

# Pull distroless image
load("@rules_oci//oci:pull.bzl", "oci_pull")

oci_pull(
    name = "distroless_cc",
    digest = "sha256:8aad707f96620ee89e27febef51b01c6ff244277a3560fcfcfbe68633ef09193",
    image = "gcr.io/distroless/cc",
    platforms = ["linux/amd64","linux/arm64"],
)
