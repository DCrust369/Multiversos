mkdir -p hardenedBSD-hardened-current-master && echo -e '#!/bin/bash\necho "the directory has been executable!"\n# pluus commands:' >hardenedBSD-hardened-current-master/AppRun && chmod +x hardenedBSD-hardened-current-master/AppRun

./hardenedBSD-hardened-current-master

# for SELinux executable

mkdir -p selinux-main && echo -e '#!/bin/bash\necho "other directory has been executable!"\n# plus commands:' >selinux-main/AppRun && chmod +x selinux-main/AppRun

./selinux-main
