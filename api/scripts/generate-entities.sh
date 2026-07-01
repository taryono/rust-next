#!/usr/bin/env bash
set -e

echo "🔄 Generating SeaORM entities..."

# load env
if [ -f .env ]; then
  export $(grep -v '^#' .env | xargs)
else
  echo "❌ .env not found"
  exit 1
fi

OUT_DIR="entity_tmp"

rm -rf $OUT_DIR

sea-orm-cli generate entity \
  -u "$DATABASE_URL" \
  -o "$OUT_DIR" \
  --with-serde \
  --expanded-format

echo "✅ Done!"
echo "📂 Output: $OUT_DIR/"
echo "📝 Compare with:"
echo "   diff -r entity/src $OUT_DIR"
