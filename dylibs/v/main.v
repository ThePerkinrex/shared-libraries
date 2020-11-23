[export: add]
fn add(a Drinck) {
  println('${cstring_to_vstring(a.name)}, V addict (Age ${a.age})')
}

struct Drinck {
	name charptr
  age u32
}