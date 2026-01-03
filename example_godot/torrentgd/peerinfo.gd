extends TPeer



func _ready() -> void:
	add_http_tracker("http://bittorrent-test-tracker.codecrafters.io/announce","d69f91e6b2ae4c542468d1073a71d4ea13879a7f")
#"%D6%9F%91%E6%B2%AE%4C%54%24%68%D1%07%3A%71%D4%EA%13%87%9A%7F")

func _on_ips_actualizadas(data: String) -> void:
	prints("ips actualisadfas torrent ", data )
	pass # Replace with function body.


func _on_timer_timeout() -> void:
	prints(get_all_trackers_info())
	prints("timer estos ip")
	pass # Replace with function body.
