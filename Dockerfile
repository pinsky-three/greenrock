FROM oven/bun:1 AS base

WORKDIR /greenrock-web-ui

COPY greenrock-web-ui/package.json .
COPY greenrock-web-ui/bun.lock .

FROM base AS install

RUN mkdir -p /temp/dev
COPY greenrock-web-ui/package.json greenrock-web-ui/bun.lock /temp/dev/
RUN cd /temp/dev && bun install --frozen-lockfile

RUN mkdir -p /temp/prod
COPY greenrock-web-ui/package.json greenrock-web-ui/bun.lock /temp/prod/
RUN cd /temp/prod && bun install --frozen-lockfile --production

FROM base AS prerelease
COPY --from=install /temp/dev/node_modules node_modules
COPY greenrock-web-ui/ .

ENV NODE_ENV=production
# RUN bun test
RUN bun run build

FROM rust:1.89

WORKDIR /greenrock-engine

COPY . .
COPY --from=prerelease /greenrock-web-ui/dist /greenrock-web-ui/dist

RUN cargo build --release --package greenrock-engine

CMD ["target/release/greenrock-engine"]