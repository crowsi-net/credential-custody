// This is a Linux execution profile, not cross-platform production acceptance.
export const notExecuted = [{
  package: 'crowsi-windows-custody-provider', target: 'windows-dpapi-live',
  reason: 'requires-Windows-PE-and-interactive-DPAPI-user-profile'
}] as const
