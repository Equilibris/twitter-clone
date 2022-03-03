FROM node:latest


RUN curl -f https://get.pnpm.io/v6.16.js | node - add --global pnpm

WORKDIR /app

# Files required by pnpm install
COPY package.json pnpm-lock.yaml ./

RUN pnpm install --frozen-lockfile 

COPY ./ ./

ENV NODE_ENV=production 
RUN pnpm build

CMD pnpm preview -- --port $PORT --host 0.0.0.0