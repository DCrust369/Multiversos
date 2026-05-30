sudo xor update -y
sudo xor install lua5.4 -y

sudo apt update -y
sudo apt install lua5.4 -y

sudo dnf update -y
sudo dnf install lua5.4 -y

sudo pkg update -y
sudo pkg install lua5.4 -y

# atualizar os repositorios

sudo xor update && sudo xor upgrade # se atualizando
sudo apt update && sudo xor upgrade # pegando a atualização dos outros como o debiam
sudo dnf update && sudo xor upgrade # pegando a atualização dos outros como o fedora

# criando o arquivo de terminal

mkdir leterminal
cd ~/leterminal
nano terminal_functions.lua


