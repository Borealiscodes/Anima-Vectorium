# Visual Grammar Map — Specification (📘)
Anima‑Vectorium v2.0 — Diagram Rendering Instructions

This document defines the exact rendering specification for:

`
diagrams/visual-grammar/visual-grammar-map.png
`

It is the authoritative layout for the altitude diagram.

---

1. Canvas

- Size: 2400 × 1600 px  
- Background: #FAFAFA  
- Padding: 120 px on all sides  
- Grid: 12‑column layout, 80 px gutters  

---

2. Altitude Blocks

Each altitude is a rounded rectangle with:

- Corner radius: 32 px  
- Stroke: 3 px, #222222 at 40% opacity  
- Shadow: none (flat design)  
- Padding: 48 px internal  

Altitude Colors (from JSON)

| Altitude | Glyph | Color |
|----------|-------|--------|
| Geometric | 🟦 | #3A7FFF |
| Safety | 🟧 | #FF8C00 |
| Infrastructure | ⬢ | #7B61FF |
| Expressive | 🟥 | #FF6A6A |
| Bridge | 🟫 | #C49A6C |
| Documentation | 📘 | #4A90E2 |

Layout (top → bottom)

`
[ Geometric ]      (top-left)
[ Safety ]         (top-right)

[ Infrastructure ] (center)

[ Expressive ]     (bottom-left)
[ Bridge ]         (bottom-center)
[ Documentation ]  (bottom-right)
`

---

3. Glyph Placement

Inside each altitude block:

- Glyph size: 72 px  
- Glyph alignment: top-left  
- Glyph margin: 8 px from top, 8 px from left  
- Title font: Inter SemiBold 48 px  
- Body font: Inter Regular 32 px  

Example (Geometric):

`
🟦 Geometric Altitude
↻ ∿ ⇢ ◎ 🔍
Holonomy, curvature, drift, stability, diagnostics.
`

---

4. Operator Rows

Each altitude block contains a horizontal operator row:

- Glyph size: 48 px  
- Spacing: 32 px between glyphs  
- Alignment: left  
- Row height: 64 px  

---

5. Flow Arrows

Arrows are straight, 4 px thick, with altitude‑colored strokes.

Expressive → Bridge → Infrastructure → Geometric → Safety

`
🟥 → 🟫 → ⬢ → 🟦 → 🟧
`

Arrow Glyphs

- Expressive → Bridge: 🪢  
- Bridge → Kernel: 🧷  
- Kernel → Operators: ↻ ∿ ⇢ ◎  
- Operators → Safety: ⚖️ → 🛡️  

Arrow Style

- Stroke: 4 px  
- Color: altitude color of the source block  
- Head: triangular, 18 px  
- Spacing: 40 px from block edges  

---

6. Accessibility Requirements

- All glyphs must have text labels beneath them.  
- Color contrast must meet WCAG AA.  
- PNG must include embedded alt text:

`
"Altitude diagram showing expressive, bridge, infrastructure, geometric, and safety layers with glyphs and flow arrows."
`

---

7. Export Requirements

- Format: PNG  
- Color profile: sRGB  
- Compression: lossless  
- No transparency  
- No anti‑aliased text rasterization artifacts  
- All text must remain selectable if exported to SVG (optional secondary export)

---

Provenance Footer — Visual Grammar Map Specification (📘)

- Artifact: visual-grammar-map-spec.md  
- Altitude: Diagrams  
- Glyph: 📘  
- Roadmap: vectoriumroadmapv2.json (artifact: diagrams-visual-grammar-map-spec)  
- License: GVL‑1.1 + Stell Non‑Commercial  
- Description: Rendering specification for the altitude diagram PNG, defining layout, glyph placement, flows, colors, typography, and accessibility requirements.
`

---

