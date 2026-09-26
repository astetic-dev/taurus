# Bouwt src/fonts/taurus-line-icons.woff2: Lucide-glyphs (ISC) onder de
# codepoints van de emoji die de Taurus-UI gebruikt (#icon-sets).
import json, sys
from fontTools.ttLib import TTFont
from fontTools import subset

MAP = {
    0x1F4C1: "folder", 0x1F4C2: "folder-open", 0x1F399: "mic", 0x1F5D1: "trash-2",
    0x1F441: "eye", 0x2699: "settings", 0x1F5A5: "monitor", 0x1F50A: "volume-2",
    0x270E: "pencil", 0x270B: "hand", 0x27F3: "refresh-cw", 0x21BB: "rotate-cw",
    0x26A0: "triangle-alert", 0x2139: "info", 0x2715: "x", 0x1F50E: "search",
    0x2315: "search", 0x1F4AC: "message-circle", 0x1F465: "users", 0x1F5FA: "map",
    0x1FA7A: "stethoscope", 0x1F4D0: "ruler", 0x1F6A6: "traffic-cone",
    0x2697: "flask-conical", 0x2712: "pen-tool", 0x25B6: "play", 0x23F8: "pause",
    0x2B21: "hexagon", 0x2261: "list", 0x2212: "minus", 0x21F1: "arrow-up-left",
}
src, cps, out = sys.argv[1], sys.argv[2], sys.argv[3]
codes = json.load(open(cps))
f = TTFont(src)
cmap = f.getBestCmap()
target = {}
for cp, name in MAP.items():
    glyph = cmap[codes[name]]
    target[cp] = glyph
# Nieuwe cmap: alleen onze codepoints.
for t in f["cmap"].tables:
    if t.isUnicode():
        t.cmap = {cp: g for cp, g in target.items() if t.format == 12 or cp <= 0xFFFF}
opts = subset.Options()
opts.flavor = "woff2"
opts.layout_features = []
opts.name_IDs = []
opts.notdef_outline = False
sub = subset.Subsetter(opts)
sub.populate(unicodes=list(target.keys()))
sub.subset(f)
# Uitlijnen op tekst: Lucide tekent van de basislijn tot 1000 (de hele em erboven),
# dus een icoon stond hoger dan de letters ernaast. 90% en 120 eenheden omlaag
# zet het midden op de x-hoogte, zoals een emoji.
glyf, hmtx = f["glyf"], f["hmtx"]
for name in f.getGlyphOrder():
    g = glyf[name]
    if g.numberOfContours and g.numberOfContours > 0:
        c = g.coordinates
        c.scale((0.9, 0.9))
        c.translate((50, -120))
        g.recalcBounds(glyf)
        adv, _ = hmtx[name]
        hmtx[name] = (adv, g.xMin)
f["hhea"].descent = -200
f["OS/2"].sTypoDescender = -200
f["OS/2"].usWinDescent = 200
f["name"].setName("Taurus Line Icons", 1, 3, 1, 0x409)
f["name"].setName("Taurus Line Icons", 4, 3, 1, 0x409)
f.flavor = "woff2"
f.save(out)
g = TTFont(out)
print("glyphs", len(g.getGlyphOrder()), "cmap", len(g.getBestCmap()), "upm", g["head"].unitsPerEm, "asc", g["hhea"].ascent, "desc", g["hhea"].descent)
