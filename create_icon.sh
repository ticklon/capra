#!/bin/bash

# 引数がなければデフォルトで "original.png" を探す
SOURCE="${1:-original.png}"
OUT_DIR="icons.iconset"
OUT_FILE="assets/icons.icns"

# 元画像の存在チェック
if [ ! -f "$SOURCE" ]; then
    echo "Error: 元画像 '$SOURCE' が見つかりません。"
    echo "使い方: ./create_icon.sh [画像ファイル名]"
    exit 1
fi

# 保存先ディレクトリ作成
mkdir -p assets
mkdir -p "$OUT_DIR"

echo "Processing $SOURCE ..."

# 必要なサイズを一括生成する関数
generate_icon() {
    local size=$1
    local scale=$2
    local pixel_size=$((size * scale))
    
    if [ "$scale" -eq 2 ]; then
        filename="icon_${size}x${size}@2x.png"
    else
        filename="icon_${size}x${size}.png"
    fi
    
    # sipsでリサイズ (出力を抑制して静かに実行)
    sips -z $pixel_size $pixel_size "$SOURCE" --out "$OUT_DIR/$filename" > /dev/null 2>&1
}

# ループで全サイズ生成 (16, 32, 128, 256, 512)
for size in 16 32 128 256 512; do
    generate_icon $size 1
    generate_icon $size 2
done

# .icns に変換
iconutil -c icns "$OUT_DIR" -o "$OUT_FILE"

# 作業用ディレクトリを削除
rm -rf "$OUT_DIR"

echo "✅ 完了: $OUT_FILE が生成されました"
