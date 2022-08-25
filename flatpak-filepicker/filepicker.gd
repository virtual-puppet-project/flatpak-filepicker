extends Node

signal file_picked(path)

func _init() -> void:
	pass

func _ready() -> void:
	var res = Safely.wrap(AM.em.load_gdnative_resource("FlatpakFilepicker", "filepicker", "PortalDialog"))
	if res.is_err():
		printerr("Failed to load custom file picker")
		queue_free()
		return
	
	var fp = res.unwrap().new()
	
	var path = fp.execute()

	emit_signal(path)

	queue_free()

