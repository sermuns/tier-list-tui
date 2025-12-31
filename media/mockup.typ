#set page(
  width: 640pt,
  height: 480pt,
  fill: luma(15%),
  margin: 2em,
)
#set text(
  fill: luma(90%),
  font: "Monaspace Krypton",
)

#import grid: cell

#let colors = (
  gradient
    .linear(red.darken(50%), green.darken(20%))
    .samples(..range(5).map(i => 100% * i / 5))
)

#grid(
  columns: (5em, 1fr),
  rows: 6em,
  inset: 1em,
  align: (x, y) => horizon + if x == 0 { center },
  fill: (x, y) => if x == 0 { colors.at(y) } else { luma(7%) },
  stroke: 1pt + white.darken(50%),

  [S], [],
  [A], [],
  [B], [],
  [C], [],
  [F], [],
)
