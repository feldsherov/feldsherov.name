@PHONY: copy restart status

copy:
	cp -r ${CURDIR} /usr/

copy-service:
	cp ${CURDIR}/feldsherov.name.service /etc/systemd/system/feldsherov.name.service

restart:
	systemctl restart feldsherov.name

status:
	systemctl status feldsherov.name
