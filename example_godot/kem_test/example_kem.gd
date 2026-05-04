extends Node

## Ejemplo de prueba de KEM (Key Encapsulation Mechanism) con libcrux
## Prueba todos los algoritmos disponibles, con foco en XWingKemDraft06

var kem = KemTool.new()

func _ready() -> void:
	print("\n╔══════════════════════════════════════════════════════════╗")
	print("║   KEM (Key Encapsulation Mechanism) — Test Suite        ║")
	print("║   Post-Quantum Crypto con libcrux                       ║")
	print("╚══════════════════════════════════════════════════════════╝\n")
	
	# Listar algoritmos
	var algos = kem.list_algorithms()
	print("📋 Algoritmos disponibles: ", algos)
	print("")

func _on_test_all_pressed() -> void:
	print("\n━━━━━━━━ Ejecutando test de TODOS los algoritmos ━━━━━━━━\n")
	var algos = kem.list_algorithms()
	for algo in algos:
		_run_single_test(algo)
	print("\n━━━━━━━━ Fin de todos los tests ━━━━━━━━\n")

func _on_test_xwing_pressed() -> void:
	print("\n━━━━━━━━ Test XWingKemDraft06 (Hybrid x25519 + ML-KEM 768) ━━━━━━━━\n")
	_run_single_test("XWingKemDraft06")
	print("\n━━━━━━━━ Fin test XWing ━━━━━━━━\n")

func _on_test_manual_pressed() -> void:
	print("\n━━━━━━━━ Test Manual: key_gen → encapsulate → decapsulate ━━━━━━━━\n")
	_run_manual_test("XWingKemDraft06")
	print("\n━━━━━━━━ Fin test manual ━━━━━━━━\n")

func _on_benchmark_pressed() -> void:
	print("\n━━━━━━━━ Benchmark: 10 iteraciones por algoritmo ━━━━━━━━\n")
	_run_benchmark()
	print("\n━━━━━━━━ Fin benchmark ━━━━━━━━\n")

## Ejecuta test_roundtrip y muestra resultados detallados
func _run_single_test(algo: String) -> void:
	var start = Time.get_ticks_msec()
	var result = kem.test_roundtrip(algo)
	var elapsed = Time.get_ticks_msec() - start
	
	if result == null:
		$panel.text += "❌ %s: ERROR (null result)\n" % algo
		print("❌ %s: ERROR" % algo)
		return
	
	var ok = result["match"]
	var status = "✅ OK" if ok else "❌ FAIL"
	
	var info = "%s %s | keygen: %.2fms | encaps: %.2fms | decaps: %.2fms | total: %.2fms" % [
		status, algo,
		result["keygen_ms"],
		result["encaps_ms"],
		result["decaps_ms"],
		result["total_ms"]
	]
	print(info)
	$panel.text += info + "\n"
	
	# Detalles de tamaño
	var sizes = "   📏 SK:%d bytes | PK:%d bytes | CT:%d bytes | SS:%d bytes" % [
		result["private_key_size"],
		result["public_key_size"],
		result["ciphertext_size"],
		result["shared_secret_size"]
	]
	print(sizes)
	$panel.text += sizes + "\n"
	
	# Mostrar shared secret en hex (primeros 32 chars)
	var ss_hex = kem.bytes_to_hex(result["shared_secret_a"])
	var ss_preview = ss_hex.substr(0, 64) + "..." if ss_hex.length() > 64 else ss_hex
	print("   🔑 Shared Secret: ", ss_preview)
	$panel.text += "   🔑 SS: " + ss_preview + "\n\n"

## Test manual paso a paso usando key_gen, encapsulate, decapsulate
func _run_manual_test(algo: String) -> void:
	$panel.text += "▶ Test Manual: %s\n" % algo
	print("▶ Test Manual: %s" % algo)
	
	# 1. Generar par de claves
	var t0 = Time.get_ticks_msec()
	var keys = kem.key_gen(algo)
	var t1 = Time.get_ticks_msec()
	
	if keys == null:
		print("  ❌ Error en key_gen")
		$panel.text += "  ❌ Error en key_gen\n"
		return
	
	var sk: PackedByteArray = keys["private_key"]
	var pk: PackedByteArray = keys["public_key"]
	print("  1️⃣ key_gen OK (%dms) — SK:%d bytes, PK:%d bytes" % [t1 - t0, sk.size(), pk.size()])
	$panel.text += "  1️⃣ key_gen OK (%dms) — SK:%d, PK:%d bytes\n" % [t1 - t0, sk.size(), pk.size()]
	
	# 2. Encapsular (lado B recibe pk, genera shared secret + ciphertext)
	var t2 = Time.get_ticks_msec()
	var encap_result = kem.encapsulate(algo, pk)
	var t3 = Time.get_ticks_msec()
	
	if encap_result == null:
		print("  ❌ Error en encapsulate")
		$panel.text += "  ❌ Error en encapsulate\n"
		return
	
	var ss_b: PackedByteArray = encap_result["shared_secret"]
	var ct: PackedByteArray = encap_result["ciphertext"]
	print("  2️⃣ encapsulate OK (%dms) — CT:%d bytes, SS:%d bytes" % [t3 - t2, ct.size(), ss_b.size()])
	$panel.text += "  2️⃣ encapsulate OK (%dms) — CT:%d, SS:%d bytes\n" % [t3 - t2, ct.size(), ss_b.size()]
	
	# 3. Desencapsular (lado A recibe ct, usa sk para recuperar shared secret)
	var t4 = Time.get_ticks_msec()
	var ss_a: PackedByteArray = kem.decapsulate(algo, ct, sk)
	var t5 = Time.get_ticks_msec()
	
	if ss_a.size() == 0:
		print("  ❌ Error en decapsulate")
		$panel.text += "  ❌ Error en decapsulate\n"
		return
	
	print("  3️⃣ decapsulate OK (%dms) — SS:%d bytes" % [t5 - t4, ss_a.size()])
	$panel.text += "  3️⃣ decapsulate OK (%dms) — SS:%d bytes\n" % [t5 - t4, ss_a.size()]
	
	# 4. Verificar que ambos shared secrets son iguales
	var match_ok = (ss_a == ss_b)
	if match_ok:
		print("  ✅ VERIFICACIÓN OK — Ambos shared secrets coinciden!")
		$panel.text += "  ✅ VERIFICACIÓN OK — Shared secrets coinciden!\n"
	else:
		print("  ❌ VERIFICACIÓN FALLIDA — Shared secrets NO coinciden!")
		$panel.text += "  ❌ VERIFICACIÓN FALLIDA!\n"
	
	# Mostrar hex
	var hex_a = kem.bytes_to_hex(ss_a)
	var hex_b = kem.bytes_to_hex(ss_b)
	print("  SS_A: ", hex_a)
	print("  SS_B: ", hex_b)
	$panel.text += "  SS_A: " + hex_a + "\n"
	$panel.text += "  SS_B: " + hex_b + "\n\n"

## Benchmark: múltiples iteraciones por algoritmo
func _run_benchmark() -> void:
	var iterations = 10
	var algos = kem.list_algorithms()
	
	$panel.text += "Benchmark: %d iteraciones por algoritmo\n\n" % iterations
	print("Benchmark: %d iteraciones por algoritmo" % iterations)
	
	for algo in algos:
		var total_time = 0.0
		var all_ok = true
		
		for i in range(iterations):
			var result = kem.test_roundtrip(algo)
			if result == null or not result["match"]:
				all_ok = false
				break
			total_time += result["total_ms"]
		
		var avg_ms = total_time / iterations
		var status = "✅" if all_ok else "❌"
		var line = "%s %s: promedio %.2fms (%d iteraciones)" % [status, algo, avg_ms, iterations]
		print(line)
		$panel.text += line + "\n"
	
	$panel.text += "\n"

func _on_clear_pressed() -> void:
	$panel.text = ""
