

FROM node:20-alpine AS frontend

WORKDIR /app

# Install deps
COPY package.json package-lock.json ./
RUN npm ci

# Copy configs and source
COPY tsconfig.json tsconfig.node.json ./
COPY vite.config.ts ./
COPY tailwind.config.* postcss.config.* ./
COPY src ./src
COPY public ./public
COPY index.html ./
# Build React app
RUN npm run build


# Stage 2 - Build Tauri app
#FROM ivangabriele/tauri:debian-bookworm-22

#WORKDIR /app

#COPY src-tauri ./src-tauri
#COPY --from=frontend /app/dist ./src-tauri/dist
 
#WORKDIR /app/src-tauri

#RUN cargo build --release

# Run app
#CMD ["./target/release/writepad"]
FROM ivangabriele/tauri:debian-bookworm-22

WORKDIR /app

COPY src-tauri ./src-tauri
COPY --from=frontend /app/dist ./src-tauri/dist

# ⬇️ Set working directory where Cargo.toml lives
WORKDIR /app/src-tauri

# 🔨 Build the Tauri binary
RUN cargo build --release

CMD ["./target/release/writepad"]