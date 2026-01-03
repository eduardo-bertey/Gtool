extends Control

func _ready():
	test_group_signatures()

func test_group_signatures():
	var nostringer = Nostringer.new()
	print("--- Starting Group (Ring) Signature Test ---")

	# 1. Create a ring of 5 people
	var private_keys = []
	var public_keys = []
	for i in range(5):
		var kp = nostringer.generate_keypair("xonly")
		private_keys.append(kp["private_key"])
		public_keys.append(kp["public_key"])
	
	print("Ring created with 5 members.")

	# 2. Votación con detección de fraude (Linkability)
	var message = "Vote: Proposal #123".to_utf8_buffer()
	var votes_db = {} # stores { "key_image_hex": "voter_name" }

	var vote = func(voter_name: String, priv_key: String):
		var res = nostringer.sign(message, priv_key, public_keys, "blsag")
		var sig = res["signature"]
		var ki = res["key_image"]
		
		print("\n[%s] is voting..." % voter_name)
		
		# verify()
		if not nostringer.verify(sig, message, public_keys, ki):
			print("  ERROR: Invalid signature!")
			return

		# check linkability
		if votes_db.has(ki):
			print("  REJECTED: Double vote detected! Identical Key Image found.")
		else:
			votes_db[ki] = voter_name
			print("  SUCCESS: Vote counted anonymously.")

	# Miembro 0 votes
	vote.call("Voter Alpha", private_keys[0])
	
	# Miembro 2 votes
	vote.call("Voter Beta", private_keys[2])
	
	# Miembro 0 tries to vote again (malicious)
	vote.call("Voter Alpha (Attempt 2)", private_keys[0])

	print("\n--- Summary ---")
	print("Total physical votes in group: ", votes_db.size())
	print("--- Group Test Completed ---")
