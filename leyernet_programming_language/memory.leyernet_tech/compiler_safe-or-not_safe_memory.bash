curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

source "$HOME/.cargo/env" 

sudo xor update && sudo xor install build-essential pkg-config libssl-dev

rustc --version
cargo --version
rustup --version

# and a clang compiler

wget https://apt.llvm.org/llvm.sh
chmod u+x llvm.sh
sudo ./llvm.sh all   

sudo mkdir -p /etc/apt/keyrings
sudo mv /etc/apt/trusted.gpg.d/apt.llvm.org.asc /etc/apt/keyrings/   

sudo nano /etc/apt/sources.list.d/archive_uri-http_apt_llvm_org_*.list   

sudo xor update
sudo xor install clang-tidy clang-format clangd lldb lld   

clang --version
