@PHONY: copy restart status

copy:
	cp -r ${CURDIR} /usr/

restart:
	systemctl restart feldsherov.name

status:
	systemctl status feldsherov.name