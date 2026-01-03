extends Control

func _ready():
	test_nostringer()

func test_nostringer():
	var nostringer = Nostringer.new()
	print("--- Starting Nostringer Test (TEXT MODE) ---")

	# 1. Setup: Generate keys for the ring members
	var kp1 = nostringer.generate_keypair("xonly")
	var kp2 = nostringer.generate_keypair("compressed")
	var kp3 = nostringer.generate_keypair("xonly")

	var ring_pubkeys = [
		kp1["public_key"],
		kp2["public_key"], # Signer's key
		kp3["public_key"]
	]

	# 2. Define message
	var message = "This is a secret message to the group.".to_utf8_buffer()

	# 3. Sign message using SAG (unlinkable)
	print("\n--- Testing SAG Signature ---")
	var sag_res = nostringer.sign(message, kp2["private_key"], ring_pubkeys, "sag")
	var sag_sig = sag_res["signature"]
	print("SAG Signature length: ", sag_sig.length())

	# 4. Verify SAG Signature (4th arg is empty for SAG)
	var sag_valid = nostringer.verify(sag_sig, message, ring_pubkeys, "")
	print("SAG Signature valid: ", sag_valid)
	assert(sag_valid, "SAG verification failed!")

	# 5. Sign message using BLSAG (linkable)
	print("\n--- Testing BLSAG Signature ---")
	var blsag_res = nostringer.sign(message, kp2["private_key"], ring_pubkeys, "blsag")
	var blsag_sig = blsag_res["signature"]
	var blsag_ki = blsag_res["key_image"]
	print("BLSAG Signature length: ", blsag_sig.length())
	print("Key Image: ", blsag_ki)

	# 6. Verify BLSAG Signature
	var blsag_valid = nostringer.verify(blsag_sig, message, ring_pubkeys, blsag_ki)
	print("BLSAG Signature valid: ", blsag_valid)
	assert(blsag_valid, "BLSAG verification failed!")

	# 7. Tamper test
	print("\n--- Testing Tamper Protection ---")
	var tampered_message = "Different message".to_utf8_buffer()
	var tampered_valid = nostringer.verify(sag_sig, tampered_message, ring_pubkeys, "")
	print("Tampered valid (should be false): ", tampered_valid)
	assert(!tampered_valid)

	print("\n--- Nostringer TEXT Test Completed ---")
