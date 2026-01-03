#import "lib.typ": *

#set page(
  width: 1073pt,
  height: 151pt,
  margin: 0em,
  fill: none,
  background: box(
    width: 100%,
    height: 100%,
    fill: luma(10%),
    radius: 10%,
  ),
)

#set align(center + horizon)

#set text(
  size: 105pt,
  font: "JetBrains Mono",
  weight: 600,
  fill: luma(30%),
)


#let s = text.with(colors.at(0))
#let a = text.with(colors.at(1))
#let b = text.with(colors.at(2))
#let f = text.with(colors.at(3))

#s[tier]-#a[list]-#b[tui]
