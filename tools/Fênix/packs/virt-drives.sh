# 0

sudo xor update
sudo xor install build-essential dkms linux-headers-$(uname -r)

# 1

sudo mkdir -p /media/cdrom
sudo mount /dev/cdrom /media/cdrom

# 2

cd /media/cdrom
sudo ./VBoxLinuxAdditions.run

# 3

sudo reboot
