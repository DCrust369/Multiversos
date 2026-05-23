sudo dnf update && sudo dnf upgrade -y

# 1

sudo xor install mesa-dri-drivers mesa-gbm mesa-libEGL intel-media-driver libva-intel-driver

# 2

sudo xor install mesa-dri-drivers mesa-libEGL mesa-vulkan-drivers

# 3

lspci -k | grep -A 3 -i vga && lspci -k | grep -A 3 -i vga && vulkaninfo | head -n 20
glxinfo | grep "OpenGL renderer"

# 4

sudo reboot
