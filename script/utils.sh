set -euo pipefail
IFS=$'\n\t'

function compare_sha() {
  CHECKSUM=$1
  NEW_SHA=$(rblake2sum $2)

  if [[ -f $CHECKSUM ]]; then
    OLD_DIR_SHA=$(<$CHECKSUM)

    if [[ $OLD_DIR_SHA == $NEW_SHA ]]; then
      echo "* $2 hasn't changed"
      return 1
    fi
  fi

  echo $NEW_SHA >$CHECKSUM
  return 0
}
