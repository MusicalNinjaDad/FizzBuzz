FROM docker.io/library/fedora

# ---
# Create the default user - vscode mounts workspace directory chowned to 1000:1000
ARG USERNAME=fizzbuzz
ARG USER_UID=1000
ARG USER_GID=${USER_UID}
RUN groupadd --gid ${USER_GID} ${USERNAME} \
&& useradd --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME} \
&& echo ${USERNAME} ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/${USERNAME} \
&& chmod 0440 /etc/sudoers.d/${USERNAME}

RUN dnf -y --setopt=install_weak_deps=False install \
    git \
    which \
&& dnf -y install \
    python \
    python-pip \
    python-pytest \
    clang

ENV RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    PATH=/opt/cargo/bin:$PATH
RUN dnf -y install \
    rustup \
&& rustup-init -v -y \
&& rustup component add rust-src

# # Stuff to delete later
# && dnf -y clean all
# && ln -s /usr/lib/rustlib/x86_64-unknown-linux-gnu/lib /usr/lib/rustlib/src/rust/library

USER ${USERNAME}