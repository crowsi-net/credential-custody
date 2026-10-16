#!/bin/sh
set -u
# Read-only capability diagnosis; never change sysctl/AppArmor or invoke sudo.
for path in /proc/sys/kernel/apparmor_restrict_unprivileged_userns \
  /proc/sys/kernel/unprivileged_userns_clone /proc/sys/user/max_user_namespaces; do
  if test -r "$path"; then printf '%s=' "$path"; cat "$path"; fi
done
if test -r /proc/self/attr/current; then cat /proc/self/attr/current; fi
/usr/bin/unshare --user --map-root-user /usr/bin/true
status=$?
printf 'Namespace capability probe exit=%s; this is not a product test result.\n' "$status"
exit 0
