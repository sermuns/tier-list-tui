#import "lib.typ": *

#set page(
  height: 500pt,
  width: 500pt,
  margin: 50pt,
  fill: none,
  background: box(
    width: 100%,
    height: 100%,
    fill: luma(10%),
    radius: 10%,
  ),
)
#set align(center)

#set text(
  size: 105pt,
  font: "JetBrains Mono",
  weight: 600,
)


#let s = text(colors.at(0), "t")
#let a = text(colors.at(1), "i")
#let b = text(colors.at(2), "e")
#let f = text(colors.at(3), "r")
#let h = text(baseline: -0.12em, fill: luma(50%), "_")

#grid(
  columns: 4 * (1fr,),
  rows: 1fr,
  ..(s,) * 2,
  ..(h,) * 2,
  ..(a,) * 4,
  ..(b,) * 2,
  ..(h,) * 2,
  ..(f,) * 3,
  ..(h,) * 1,
)
