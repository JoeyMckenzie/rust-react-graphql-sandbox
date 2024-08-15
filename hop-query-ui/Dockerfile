# Build stage
FROM node:20-bullseye-slim AS build

# Install pnpm
RUN npm install -g pnpm@9.6.x

WORKDIR /app

# Copy pnpm-lock.yaml in addition to package.json
COPY package.json pnpm-lock.yaml ./

# Install dependencies
RUN pnpm install --frozen-lockfile

# Copy the rest of the code
COPY . .

# Build the app
RUN pnpm run build

# Serve stage
FROM nginx:alpine

# Copy the build output to replace the default nginx contents.
COPY --from=build /app/dist /usr/share/nginx/html

# Copy custom nginx config
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]