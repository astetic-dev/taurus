# Snijdt de stier uit het oorspronkelijke logo (#251): weg met de grijze
# achtergrond, de lichtoranje verfvlek (~31° tint; de kop is geler), het raster,
# losse spetters en dunne schetslijnen. Gebruik:
#   python tools/cut-taurus-logo.py <origineel.png> src/taurus-logo.png
# (Pillow, numpy, scipy). Het origineel staat in de git-geschiedenis.
import numpy as np
from PIL import Image, ImageFilter
from scipy import ndimage as nd
import colorsys, sys
src, out = sys.argv[1], sys.argv[2]
im = Image.open(src).convert('RGB')
a = np.asarray(im).astype(np.float32) / 255
r, g, b = a[..., 0], a[..., 1], a[..., 2]
mx, mn = a.max(-1), a.min(-1)
sat = np.where(mx > 0, (mx - mn) / np.maximum(mx, 1e-6), 0)
val = mx
# tint (graden)
h = np.zeros_like(mx)
d = np.maximum(mx - mn, 1e-6)
h = np.where(mx == r, ((g - b) / d) % 6, h)
h = np.where(mx == g, (b - r) / d + 2, h)
h = np.where(mx == b, (r - g) / d + 4, h)
h = h * 60
light_grey = (sat < 0.22) & (val > 0.55)             # achtergrond + rasterstippen
orange = (h > 15) & (h < 34) & (sat > 0.45) & (val > 0.55)  # verfvlek ~31°; de kop is geler (36-39°)
cand = light_grey | orange
lab, n = nd.label(cand)
border = set(np.unique(np.concatenate([lab[0], lab[-1], lab[:, 0], lab[:, -1]]))) - {0}
bg = np.isin(lab, list(border))
fg = ~bg
# alleen de grootste samenhangende vorm: weg met losse spetters buiten de stier
lab2, n2 = nd.label(fg)
sizes = nd.sum(fg, lab2, range(1, n2 + 1))
keep = 1 + int(np.argmax(sizes))
fg = lab2 == keep
fg = nd.binary_fill_holes(fg)
# Dunne schetslijnen die uit de figuur steken weg: openen met een schijf van
# straal 5 haalt alles weg dat smaller is dan ~10 px, en terugzetten binnen een
# rand rond wat overbleef houdt de echte contouren (en de hoornpunten) scherp.
yy, xx = np.ogrid[-5:6, -5:6]
disk = (xx * xx + yy * yy) <= 25
core = nd.binary_opening(fg, structure=disk)
fg = fg & nd.binary_dilation(core, structure=disk, iterations=2)
lab3, n3 = nd.label(fg)
s3 = nd.sum(fg, lab3, range(1, n3 + 1))
fg = lab3 == 1 + int(np.argmax(s3))
alpha = Image.fromarray((fg * 255).astype(np.uint8)).filter(ImageFilter.GaussianBlur(1.2))
rgba = im.copy(); rgba.putalpha(alpha)
bbox = alpha.point(lambda v: 255 if v > 8 else 0).getbbox()
rgba = rgba.crop(bbox)
# vierkant met wat marge
w, hgt = rgba.size; side = int(max(w, hgt) * 1.06)
canvas = Image.new('RGBA', (side, side), (0, 0, 0, 0))
canvas.paste(rgba, ((side - w) // 2, (side - hgt) // 2), rgba)
canvas = canvas.resize((512, 512), Image.LANCZOS)
canvas.save(out)
print('components', n, 'border comps', len(border), 'fg comps', n2, 'kept px', int(sizes.max()))
