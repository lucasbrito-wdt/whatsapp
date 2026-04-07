#!/bin/bash
set -euo pipefail
source /mnt/dev/wdt/whatsapp/parts/whatsapp-unofficial/run/environment.sh
set -x
cp --archive --link --no-dereference . "/mnt/dev/wdt/whatsapp/parts/whatsapp-unofficial/install"
