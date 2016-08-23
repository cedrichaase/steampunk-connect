all:
	rsync -avr --exclude .git --exclude target . root@cryptkeeper:steampunk-connect/
	ssh root@cryptkeeper "cd steampunk-connect && cargo build"
