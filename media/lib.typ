#let colors = (
  gradient
    .linear(red.darken(20%), green.darken(0%))
    .samples(..range(4).map(i => 100% * i / 5))
)
