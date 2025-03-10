#!/bin/sh

if [ "$COMMITFMT_VERBOSE" = "1" ] || [ "$COMMITFMT_VERBOSE" = "true" ]; then
  set -x
fi

if [ "$COMMITFMT" = "0" ]; then
  exit 0
fi

call_commitfmt() {
  bin_name="commitfmt{extension}"
  if test -n "$COMMITFMT_BIN"; then
    "$COMMITFMT_BIN" "$@"
  elif $bin_name -h >/dev/null 2>&1; then
    $bin_name "$@"
  else
    repo_root="$(git rev-parse --show-toplevel)"
    bin_path="$repo_root/node_modules/commitfmt-{os}-{arch}/bin/$bin_name"
    index_path="$repo_root/node_modules/commitfmt/bin/index.js"
    if test -f "$bin_path"; then
      "$bin_path" "$@"
    elif test -f "$index_path"; then
      "$index_path" "$@"
    elif yarn commitfmt -h >/dev/null 2>&1; then
      yarn commitfmt "$@"
    elif pnpm commitfmt -h >/dev/null 2>&1; then
      pnpm commitfmt "$@"
    else
      echo "ERROR: Can't find commitfmt in PATH."
      echo "Make sure commitfmt is available in your environment and re-try."
      echo "To skip these checks use --no-verify git argument or set COMMITFMT=0 env variable."
      exit 1
    fi
  fi
}

call_commitfmt apply "$@"
