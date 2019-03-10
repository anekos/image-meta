#!/bin/bash

set -euC
set -o pipefail

# exec 5>> /tmp/xmosh/shell-script-debug.out
# BASH_XTRACEFD="5"
# PS4='$LINENO: '
# set -x

if [ "$#" -lt 1 ]
then
  echo "$0 <IMAGE_DIRECTORY>" 1>&2
  exit 1
fi

target="$1"

cargo build --release

while read -r file
do
  echo "$file"

  if ! magick="$(identify -format '%[W]x%[H]\n' "$file" | head -n 1 2> /dev/null)"
  then
    echo '  → Skip'
    continue
  fi
  my="$(./target/release/example "$file")"

  if [ "$my" = "$magick" ]
  then
    echo "  → OK! $my"
  else
    echo "  → FAIL! $my != $magick"
    exit 1
  fi
done < <(find "$target" -type f -name '*.png' -or -name '*.gif' -or -name '*.jpg')

