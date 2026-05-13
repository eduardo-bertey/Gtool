extends Control

# Mapeo local para asegurar compatibilidad total
const ALGO = {
	"HAMMING": 0,
	"LEVENSHTEIN": 1,
	"NORMALIZED_LEVENSHTEIN": 2,
	"OSA_DISTANCE": 3,
	"DAMERAU_LEVENSHTEIN": 4,
	"NORMALIZED_DAMERAU_LEVENSHTEIN": 5,
	"JARO": 6,
	"JARO_WINKLER": 7,
	"SORENSEN_DICE": 8
}

func _ready():
	var strsim = StringSimilarity.new()
	
	# 1. Uso simple
	strsim.algorithm = ALGO.LEVENSHTEIN
	var d = strsim.compare("kitten", "sitting")
	print("Distancia Levenshtein (kitten, sitting): ", d)
	
	# 2. Cambiando algoritmo
	strsim.algorithm = ALGO.JARO_WINKLER
	var s = strsim.compare("cheeseburger", "cheese fries")
	print("Similitud Jaro-Winkler: ", s)
	
	# 3. Comparación de todos los algoritmos
	var s1 = "rust"
	var s2 = "rustacean"
	
	var output = "Comparando '%s' vs '%s':\n\n" % [s1, s2]
	
	# Iteramos sobre nuestro diccionario local
	for name in ALGO.keys():
		var val = ALGO[name]
		var res = strsim.get_similarity(s1, s2, val)
		output += "- %s: %f\n" % [name, res]
	
	print(output)
	if has_node("Label"):
		$Label.text = output
