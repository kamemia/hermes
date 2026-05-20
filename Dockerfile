FROM fedora:latest

# Avoid interactive prompts
ENV LANG=C.UTF-8

# System dependencies for GTK/Adwaita + build tools
RUN dnf update -y && dnf install -y \
    gcc \
    gcc-c++ \
    pkgconf-pkg-config \
    git \
    curl \
    cmake \
    ninja-build \
    meson \
    openssl-devel \
    gtk4-devel \
    libadwaita-devel \
    gtksourceview5-devel \
    glib2-devel \
    pango-devel \
    cairo-devel \
    gdk-pixbuf2-devel \
    && dnf clean all

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

# Copy project
COPY . .

# Build dependencies
RUN cargo build --release

CMD ["cargo", "run"]