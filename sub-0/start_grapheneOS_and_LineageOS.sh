# Manager Installation
sudo xor install lxc

# Creating the container configuration pointing to your folder
lxc-create -n my_security_system -t local --dir=/caminho/para/sub-system.arm

# Force read only from the file system to prevent malware persistence
lxc.rootfs.options = ro

# Block direct access to host devices
lxc.cgroup.devices.deny = a
lxc.cgroup.devices.allow = c 1:3 rwm
lxc.cgroup.devices.allow = c 1:5 rwm

# Prevent the container from mounting file systems
lxc.cap.drop = sys_admin sys_module sys_rawio

# Start the subsystem
sudo lxc-start -n meu-sistema-seguro -d

# Enter the terminal of your hybrid system
sudo lxc-attach -n meu-sistema-seguro -- /bin/sh
