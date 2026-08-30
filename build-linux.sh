#!/usr/bin/env bash

set -u

APPDIR="target/release/bundle/appimage/BongoCat.AppDir"
OUTPUT="target/release/bundle/appimage/BongoCat.AppImage"

echo "==> Gerando AppDir..."

cargo tauri build --bundles appimage || true

if [ ! -d "$APPDIR" ]; then
    echo "ERRO: AppDir não encontrado."
    exit 1
fi

echo "    AppDir OK"

# Corrige o ícone esperado pelo Desktop Entry
if [ -f "$APPDIR/BongoCat.png" ]; then
    cp "$APPDIR/BongoCat.png" "$APPDIR/bongo-cat.png"
    echo "    Ícone OK"
else
    echo "ERRO: BongoCat.png não encontrado."
    exit 1
fi

# Verifica appimagetool
if ! command -v appimagetool >/dev/null 2>&1; then
    echo "ERRO: appimagetool não está instalado."
    echo "Instale com: yay -S appimagetool-bin"
    exit 1
fi

rm -f "$OUTPUT"

echo "==> Criando AppImage..."

ARCH=x86_64 appimagetool "$APPDIR" "$OUTPUT"

if [ ! -f "$OUTPUT" ]; then
    echo "ERRO: AppImage não foi criado."
    exit 1
fi

chmod +x "$OUTPUT"

echo
echo "================================"
echo " AppImage criado com sucesso!"
echo "================================"
echo
echo "$OUTPUT"
