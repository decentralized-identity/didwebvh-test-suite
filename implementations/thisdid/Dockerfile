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

# Clone the thisdid monorepo, build only the vendored webvh-did-resolver
# subpackage (it has its own pnpm-lock.yaml and is buildable standalone),
# then save the exact commit used for provenance.
RUN mkdir -p /app \
    && git clone --depth 1 --branch "$BRANCH" "$REPO" /deps/thisdid \
    && git -C /deps/thisdid rev-parse HEAD > /app/library-commit \
    && cd /deps/thisdid/vendor/webvh-did-resolver \
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
