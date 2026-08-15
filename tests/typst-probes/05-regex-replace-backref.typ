// python: re.sub(r"([a-z0-9])([A-Z])", r"\1 \2", name)
#let s = "camelCaseValue"
#repr(s.replace(regex("([a-z0-9])([A-Z])"), m => m.captures.at(0) + " " + m.captures.at(1)))
