# Build image for the thisdid test harness (@thisdid/webvh-did-resolver).
#
# thisdid is a resolver-only DIF `did-resolver` driver vendored inside the
# larger decentralized-identity/thisdid monorepo, at vendor/webvh-did-resolver/.
# Its own toolchain (pnpm) is used to build that subpackage; the harness that
# drives it is a self-contained npm project (independent of this repo's root
# pnpm workspace, which would otherwise swallow a nested pnpm project).
#
# At runtime mount:
#   /workspace/vectors                  ← <repo>/vectors/
#   /workspace/implementations/thisdid  ← <repo>/implementations/thisdid/
#
# node_modules live at /workspace/node_modules/ (not inside
# implementations/thisdid/) so they survive the runtime mount.

FROM node:24-slim

ARG REPO=https://github.com/decentralized-identity/thisdid
ARG BRANCH=main

ENV COREPACK_HOME=/opt/corepack

RUN apt-get update && apt-get install -y --no-install-recommends \
      ca-certificates git jq curl \
    && corepack enable \
    && corepack prepare pnpm@11.10.0 --activate \
    && chmod -R a+rX "$COREPACK_HOME" \
    && rm -rf /var/lib/apt/lists/*

# Clone the thisdid monorepo, save the exact commit used for provenance, then
# build only the vendored webvh-did-resolver subpackage (it has its own
# pnpm-lock.yaml).
#
# Since thisdid commit 97f6b8f, webvh-did-resolver depends on
# `didwebvh-ts: link:../forks/didwebvh-ts`, a git submodule (GoPlausible's
# fork of didwebvh-ts, pinned by the thisdid commit). A `link:` dependency is
# neither installed nor built by pnpm, so the submodule must be checked out,
# installed and built first. The resolver also compiles node-guard.ts against
# Node's types, which upstream CI gets from the monorepo root's `npm ci`; here
# just @types/node (at the root lockfile's version) is installed into
# vendor/node_modules, where tsc finds it by walking up from the subpackage.
# Both steps are skipped for older thisdid commits that predate the fork.
RUN mkdir -p /app \
    && git clone --depth 1 --branch "$BRANCH" "$REPO" /deps/thisdid \
    && git -C /deps/thisdid rev-parse HEAD > /app/library-commit \
    && cd /deps/thisdid \
    && if git config -f .gitmodules --get submodule.vendor/forks/didwebvh-ts.path >/dev/null 2>&1; then \
         git submodule update --init --depth 1 vendor/forks/didwebvh-ts \
         && (cd vendor/forks/didwebvh-ts && pnpm install --frozen-lockfile && pnpm run build) \
         && TYPES_NODE="$(jq -r '.packages["node_modules/@types/node"].version // "latest"' package-lock.json 2>/dev/null || echo latest)" \
         && npm install --no-save --no-package-lock --ignore-scripts --prefix vendor "@types/node@${TYPES_NODE}"; \
       fi \
    && cd vendor/webvh-did-resolver \
    && pnpm install --frozen-lockfile \
    && pnpm run build

WORKDIR /workspace

# Copy the harness manifest, repoint @thisdid/webvh-did-resolver at the built
# local copy, then install with plain npm (avoids this repo's root
# pnpm-workspace.yaml, which would otherwise claim this nested project).
COPY implementations/thisdid/package.json ./
RUN jq '.dependencies["@thisdid/webvh-did-resolver"] = "file:/deps/thisdid/vendor/webvh-did-resolver"' package.json \
      > /tmp/package.json && mv /tmp/package.json package.json \
    && npm install

# Copy harness source (overridden at runtime by the host mount, but useful
# for standalone image use and for import resolution during build).
COPY implementations/thisdid/ ./implementations/thisdid/

CMD ["npx", "tsx", "implementations/thisdid/src/harness.ts"]
