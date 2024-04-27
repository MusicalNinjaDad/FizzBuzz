FROM docker.io/library/fedora

# ---
# Setup base system ...
# ---

# Enable man pages by commenting out the nodocs flag
COPY <<EOF /etc/dnf/dnf.conf
[main]
gpgcheck=True
installonly_limit=3
clean_requirements_on_remove=True
best=False
skip_if_unavailable=True
# tsflags=nodocs
EOF

# Rust stuff goes in /opt so we don't end up with system and user installs: this is a single user system.
# Use (setf)acl to default all contents of RUSTUP_HOME & CARGO_HOME to world read-write (x = directories only).
ENV RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    PATH=/opt/cargo/bin:$PATH
RUN mkdir --mode=777 --parents $RUSTUP_HOME \
&& mkdir --mode=777 --parents $CARGO_HOME


# ---
# Install ...
# ---

# Man pages for all the stuff which is already installed, man itself and basic manpages
RUN dnf -y --setopt=install_weak_deps=False reinstall $(dnf list --installed | awk '{print $1}') \
&& dnf -y --setopt=install_weak_deps=False install \
    man \
    man-db \
    man-pages \
&& dnf -y update

# Basic development tools
RUN dnf -y --setopt=install_weak_deps=False install \
    git \
    which

# Python
RUN dnf -y install \
    python \
    python-pip \
    python-pytest

# Rust (and python headers)
RUN dnf -y install \
    clang \
    python3-devel \
    rustup \
&& rustup-init -v -y \
&& rustup component add rust-src

# pyo3 specifics
RUN dnf -y install \
    nox \
    openssl-devel \
&& cargo install cargo-llvm-cov \
&& rustup component add llvm-tools-preview \
&& cargo install mdbook \
&& cargo install cargo-expand \
&& cargo install lychee


# ---
# Final setup steps
# ---

# Create the default user - most agents mount workspace directory chowned to 1000:1000
# and chown CARGO_HOME and RUSTUP_HOME to the default user 
ARG USERNAME=fizzbuzz
ARG USER_UID=1000
ARG USER_GID=${USER_UID}
RUN groupadd --gid ${USER_GID} ${USERNAME} \
&& useradd --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME} \
&& echo ${USERNAME} ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/${USERNAME} \
&& chmod 0440 /etc/sudoers.d/${USERNAME} \
&& chown -R ${USER_UID} ${CARGO_HOME} \
&& chown -R ${USER_UID} ${RUSTUP_HOME}

# Set the default user
USER ${USERNAME}
