# [Gravitalia](https://www.gravitalia.com/)
> Source code for Gravitalia social network.

Why is Gravitalia bloated?
- **Learn** new technologies such as Rust, Docker and Bazel.

## Services
| Name | Readme | Description |
|------------|------------|------------|
| Front | [readme](front/README.md) | Vue website using Nuxt as framework. |
| GraphQL | [readme](graphql/README.md) | HTTP API using GraphQL query language. |
| Notification | [readme] | Elixir [SSE](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events) to notify user in-app. ![In-Dev Badge](https://img.shields.io/badge/private%20beta-8A2BE2) |

## Install Gravitalia
### Dependencies
To build and run Gravitalia, you'll need:
- `git`
- `pnpm`
- `bazel`

### Building
1. Clone this repository with `git`:
   
   ```
   git clone https://github.com/RealHinome/gravitalia --recurse-submodules
   cd gravitalia
   ```
2. Install Node dependencies with `pnpm`:
   ```
   pnpm install
   ```
3. Build with `bazel`:
   ```
   bazel build //... # this will build every projects.
   ```


### License
[Mozilla Public License](https://github.com/RealHinome/gravitalia/blob/master/LICENSE)
