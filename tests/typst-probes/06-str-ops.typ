#repr((strip: "  x  ".trim(), lower: lower("ABC"), upper: upper("abc"),
  starts: "abc".starts-with("ab"), contains: "abc".contains("b"),
  join: ("a","b").join(","), slice: "abcdef".slice(1,3),
  pos: "abc".position("b"), rev: "abc".rev(), len: "abc".len(),
  first: "abc".first(), tostr: str(3.5), toint: int("42"), tofloat: float("3.14")))
