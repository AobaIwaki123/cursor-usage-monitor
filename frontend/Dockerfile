FROM oven/bun:1-alpine

WORKDIR /app

# Copy package files
COPY package.json bun.lock ./

# Install dependencies
RUN bun install

# Copy the rest of the application
COPY . .

# Expose port
EXPOSE 3000

# Start development server with hot reload
CMD ["bun", "run", "dev"]