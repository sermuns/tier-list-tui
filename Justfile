watch images-path="tmp":
	watchexec --watch src --stop-timeout 0 --restart --wrap-process session --clear -- cargo run {{images-path}} 2>/dev/null
