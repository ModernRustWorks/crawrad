#!/usr/bin/env python3
"""Regenerate the MSIX Store assets from the IDE's own icon.

The executable and the installed Store app take their icon from different
places: the `.exe` carries `crawrad.ico`, embedded by `embed-resource`, while
Windows shows an installed Store app the PNGs declared in `AppxManifest.xml`.
They drifted — the `.ico` was updated and these PNGs were left behind, so the
Store build wore an older crab than the product.

This script makes the PNGs a product OF the `.ico` instead of a copy kept by
hand, so the two cannot disagree again. Run it whenever the icon changes:

    python scripts/msix-assets.py

It needs Pillow (`pip install pillow`).
"""

import pathlib
import struct
import sys

try:
    from PIL import Image
except ImportError:  # pragma: no cover - a tool, not library code
    sys.exit("Pillow is required: pip install pillow")

RAIZ = pathlib.Path(__file__).resolve().parent.parent
ICO = RAIZ / "crates" / "crawrad-ide" / "crawrad.ico"
ASSETS = RAIZ / "installer" / "windows" / "msix" / "Assets"

# Each asset, and how much of its width the icon fills. The fractions are the
# ones the hand-made assets already used: Windows adds its own padding around a
# taskbar icon but not around a tile, so a tile keeps the icon smaller.
ALVOS = [
    ("StoreLogo.png", 50, 50, 0.92),
    ("Square44x44Logo.png", 44, 44, 0.93),
    ("Square44x44Logo.targetsize-256_altform-unplated.png", 256, 256, 0.96),
    ("Square150x150Logo.png", 150, 150, 0.82),
    ("Wide310x150Logo.png", 310, 150, 0.46),
]


def maior_imagem_do_ico(caminho: pathlib.Path) -> Image.Image:
    """The largest image inside a `.ico`, as RGBA.

    Pillow opens a `.ico` at one size; reading the directory itself is how the
    256x256 master comes out whole instead of upscaled from a smaller entry.
    """
    dados = caminho.read_bytes()
    quantas = struct.unpack("<H", dados[4:6])[0]
    melhor = None
    for i in range(quantas):
        entrada = dados[6 + i * 16 : 6 + (i + 1) * 16]
        largura = entrada[0] or 256
        tamanho, deslocamento = struct.unpack("<II", entrada[8:16])
        if melhor is None or largura > melhor[0]:
            melhor = (largura, dados[deslocamento : deslocamento + tamanho])
    if melhor is None:
        sys.exit(f"{caminho} holds no images")
    import io

    return Image.open(io.BytesIO(melhor[1])).convert("RGBA")


def recortado(im: Image.Image) -> Image.Image:
    """The icon without its transparent margin, so framing is about the art."""
    caixa = im.getchannel("A").getbbox()
    return im.crop(caixa) if caixa else im


def main() -> None:
    if not ICO.exists():
        sys.exit(f"{ICO} not found")
    arte = recortado(maior_imagem_do_ico(ICO))
    ASSETS.mkdir(parents=True, exist_ok=True)
    for nome, largura, altura, fracao in ALVOS:
        # The icon is square: the short side decides, so a wide tile keeps it
        # whole instead of stretching it across the tile.
        alvo = int(min(largura, altura) * (fracao if largura == altura else 0.95))
        escala = min(alvo / arte.width, alvo / arte.height)
        novo = arte.resize(
            (max(1, round(arte.width * escala)), max(1, round(arte.height * escala))),
            Image.LANCZOS,
        )
        tela = Image.new("RGBA", (largura, altura), (0, 0, 0, 0))
        tela.paste(novo, ((largura - novo.width) // 2, (altura - novo.height) // 2), novo)
        destino = ASSETS / nome
        tela.save(destino, "PNG", optimize=True)
        print(f"  {nome:52s} {largura}x{altura}")


if __name__ == "__main__":
    main()
