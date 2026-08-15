#let d = (a: 1, b: 2)
#let arr = (3, 1, 2)
#repr((
  get: d.at("z", default: none), keys: d.keys(), values: d.values(), pairs: d.pairs(),
  has: "a" in d, sorted: arr.sorted(), sortedkey: ((v: 2), (v: 1)).sorted(key: x => x.v),
  dedup: (1,1,2).dedup(), sum: arr.sum(), fold: arr.fold(0, (a,b)=>a+b),
  filter: arr.filter(x => x > 1), map: arr.map(x => x*2),
  any: arr.any(x=>x>2), all: arr.all(x=>x>0), slice: arr.slice(0,2),
  zip: arr.zip((9,8,7)), enum: arr.enumerate().first(),
))
