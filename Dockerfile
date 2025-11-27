#---------Stage 1---------------


FROM rust:latest As builder

WORKDIR /app

#Manifest
COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release



#---------Stage 2---------

FROM debian:bookworm-slim

#Install deps
RUN apt-get update && \
    apt-get install -y \
    openssh-server \
    sudo \
    && rm -rf /var/lib/apt/lists/*

#Portfolio user
RUN useradd -m -s /bin/bash portfolio && \
    echo "portfolio:portfolio123" | chpasswd

# Copy the binary from builder
COPY --from=builder /app/target/release/ssh_medaly /usr/local/bin/ssh-portfolio
RUN chmod +x /usr/local/bin/ssh-portfolio

# Set portfolio user's shell to the app
RUN usermod -s /usr/local/bin/ssh-portfolio portfolio

# Create .hushlogin to suppress MOTD
RUN touch /home/portfolio/.hushlogin && \
    chown portfolio:portfolio /home/portfolio/.hushlogin

#Configuration SSH
RUN mkdir /var/run/sshd && \
    sed -i 's/#PasswordAuthentication yes/PasswordAuthentication yes/' /etc/ssh/sshd_config && \
    sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin no/' /etc/ssh/sshd_config

# Expose SSH port
EXPOSE 22

# Start SSH service
CMD ["/usr/sbin/sshd", "-D"]
