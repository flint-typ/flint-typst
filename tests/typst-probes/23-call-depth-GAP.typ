#let depth(n) = if n <= 0 { 0 } else { 1 + depth(n - 1) }
#repr(depth(5000))
