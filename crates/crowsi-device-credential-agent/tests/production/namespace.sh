#!/bin/sh
set -eu
# Run only inside a new user/mount/PID namespace, never on the host mount table.
test "$(id -u)" = 0
awk 'NF == 3 && $1 == 0 && $2 > 0 && $3 == 1 { isolated = 1 }
     END { exit !isolated }' /proc/self/uid_map
test "$(readlink /proc/self/ns/user)" != "$3"
test "$(readlink /proc/self/ns/mnt)" != "$4"
test "$(readlink /proc/self/ns/net)" != "$5"
test "$(readlink /proc/self/ns/pid)" != "$6"
root=$1
command=$2
test -d "$root" && test ! -L "$root"
case "$root" in /*/target/production-agent-*/root) ;; *) exit 1 ;; esac
mount --make-rprivate /
readonly_runtime() {
  mount --rbind "$1" "$2"
  findmnt -R --mountpoint "$2" --raw -n -o TARGET | while IFS= read -r target; do
    case "$target" in "$2"|"$2"/*) ;; *) exit 1 ;; esac
    mount -o remount,bind,ro,nosuid,nodev "$target"
  done
  findmnt -R --mountpoint "$2" --list -n -o VFS-OPTIONS | awk -F, '
    { ro = suid = dev = 0; for (i = 1; i <= NF; i++) {
        if ($i == "ro") ro = 1; if ($i == "nosuid") suid = 1; if ($i == "nodev") dev = 1
      }; if (!ro || !suid || !dev) exit 1 }
    END { if (NR == 0) exit 1 }'
}
for directory in usr proc; do mkdir -p "$root/$directory"; done
readonly_runtime /usr "$root/usr"
for directory in lib lib64 bin sbin; do
  if test -L "/$directory"; then
    ln -sfnT "$(readlink "/$directory")" "$root/$directory"
  elif test -d "/$directory"; then
    mkdir -p "$root/$directory"
    readonly_runtime "/$directory" "$root/$directory"
  fi
done
mount -t proc -o nosuid,nodev,noexec proc "$root/proc"
printf 'isolated-installation-ready\n' > "$root/case/namespace-ready"
exec /usr/sbin/chroot "$root" /app/agent "$command" --config /case/config.json
