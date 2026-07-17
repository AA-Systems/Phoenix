FROM oven/bun:1 AS base
WORKDIR /client

FROM base AS deps
COPY package.json bun.lock ./
RUN bun install --frozen-lockfile

FROM deps AS build
ARG NEXT_PUBLIC_API_URL
ENV NEXT_PUBLIC_API_URL=$NEXT_PUBLIC_API_URL

COPY . .
RUN bun run build

FROM base AS runner
WORKDIR /client
ENV NODE_ENV=production
ENV PORT=3001

COPY --from=build /client ./

EXPOSE 3001
CMD ["bun", "run", "start"]
