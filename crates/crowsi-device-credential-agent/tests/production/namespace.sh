#!/bin/sh
set -eu
# Run only inside a new user/mount/PID namespace, never on the host mount table.
test "$(id -u)" = 0
awk 'NF == 3 && $1 == 0 && $2 > 0 && $3 == 1 { isolated = 1 }
     END { exit !isolated }' /proc/self/uid_map
test "$(readlink /proc/self/ns/user)" != "$3"
test "$(readlink /proc/self/ns/mnt)" != "$4"
root=$1
command=$2
test -d "$root" && test ! -L "$root"
case "$root" in /*/target/production-agent-*/root) ;; *) exit 1 ;; esac
mount --make-rprivate /
for directory in usr proc; do mkdir -p "$root/$directory"; done
mount --rbind /usr "$root/usr"
mount --make-rslave "$root/usr"
mount -o remount,bind,ro "$root/usr"
for directory in lib lib64 bin sbin; do
  if test -L "/$directory"; then
    ln -sfnT "$(readlink "/$directory")" "$root/$directory"
  elif test -d "/$directory"; then
    mkdir -p "$root/$directory"
    mount --rbind "/$directory" "$root/$directory"
    mount -o remount,bind,ro "$root/$directory"
  fi
done
mount -t proc proc "$root/proc"
printf 'isolated-installation-ready\n' > "$root/case/namespace-ready"
exec /usr/sbin/chroot "$root" /app/agent "$command" --config /case/config.json
