#!/usr/bin/env bash
set -euo pipefail

dry_run=false
case "${1:-}" in
    --dry-run) dry_run=true ;;
    "") ;;
    *) printf 'Usage: %s [--dry-run]\n' "$0" >&2; exit 1 ;;
esac
if (( $# > 1 )); then
    printf 'Usage: %s [--dry-run]\n' "$0" >&2
    exit 1
fi

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

for group in mini tsp; do
    group_dir="$repo_root/$group"
    [[ -d "$group_dir" ]] || continue

    while IFS= read -r -d '' artifact; do
        if [[ "$dry_run" == true ]]; then
            printf 'Would remove: %s\n' "${artifact#"$repo_root/"}"
        else
            printf 'Removing: %s\n' "${artifact#"$repo_root/"}"
            rm -rf -- "$artifact"
        fi
    done < <(find "$group_dir" \
        -type d \( -name target -o -name dist -o -name pkg -o -name node_modules \) -prune -print0)
done