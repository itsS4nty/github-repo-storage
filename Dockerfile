# Use the official Rust image as a parent image
FROM rust:1.89.0

# Install cron and other necessary packages
RUN apt-get update && apt-get install -y \
    cron \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src/ ./src/

# Build the application
RUN cargo build --release

# Copy the environment file template (you'll need to provide actual values)
# COPY .env ./

# Create a script that will be executed by cron
RUN echo '#!/bin/bash\ncd /app && ./target/release/github-repo-storage' > /app/run-app.sh && \
    chmod +x /app/run-app.sh

# Add cron job to run the application daily at 2 AM
RUN echo "0 2 * * * /app/run-app.sh >> /var/log/github-repo-storage.log 2>&1" | crontab -

# Create log file
RUN touch /var/log/github-repo-storage.log

# Create startup script that runs the app immediately, then starts cron
RUN echo '#!/bin/bash\necho "Starting GitHub Repository Storage..."\necho "Running initial execution..."\n/app/run-app.sh\necho "Initial execution completed. Starting cron for daily runs..."\ncron\ntail -f /var/log/github-repo-storage.log' > /app/startup.sh && \
    chmod +x /app/startup.sh

# Start with immediate execution and then cron
CMD ["/app/startup.sh"]