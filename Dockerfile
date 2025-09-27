# Multi-stage Docker build for Leptos Motion Demos
FROM rust:1.89 as builder

# Install trunk for CSR demo
RUN cargo install trunk

# Install Node.js for Playwright
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs

# Set working directory
WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/
COPY demos/ ./demos/
COPY tests/ ./tests/
COPY scripts/ ./scripts/
COPY package.json package-lock.json ./

# Build CSR demo
WORKDIR /app/demos/csr-demo
RUN trunk build --release

# Build SSR demo
WORKDIR /app/demos/ssr-demo
RUN cargo build --release --features ssr

# Install Playwright for testing
WORKDIR /app
RUN npm install
RUN npx playwright install

# Production stage
FROM nginx:alpine

# Copy CSR demo
COPY --from=builder /app/demos/csr-demo/dist /usr/share/nginx/html/csr

# Copy SSR demo binary
COPY --from=builder /app/demos/ssr-demo/target/release/ssr-demo /usr/local/bin/

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Copy test files
COPY --from=builder /app/tests/ /usr/share/nginx/html/tests/
COPY --from=builder /app/scripts/ /usr/local/bin/scripts/

# Expose ports
EXPOSE 80 9000 9001

# Start nginx and SSR demo
CMD ["sh", "-c", "nginx -g 'daemon off;' & /usr/local/bin/ssr-demo & wait"]
