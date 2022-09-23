extends Node

signal file_selected(path)

# _x is unused but required to match the API surface of FileDialog
func popup_centered_ratio(_x = 0):
	var res = Safely.wrap(AM.em.load_gdnative_resource("FlatpakFilepicker", "FilePicker", "PortalDialog"))
	if res.is_err():
		printerr("Failed to load custom file picker")
		queue_free()
		return
	
	var fp = res.unwrap()
	
	var path = fp.get_file()

	emit_signal("file_selected", path)

	queue_free()
