- Mount disk to /storage
- Install docker
- Install docker-compose
- uid=1000(feldsherov) gid=1000(feldsherov)
- Repository is copied to /usr/feldsherov.name
    - `sudo make copy`
- /usr/feldsherov.name/feldsherov.name.service copied to /etc/systemd/system/
- `sudo systemctl enable feldsherov.name`
- `sudo systemctl start feldsherov.name`

# Transmission
`sudo docker exec -it transmission /bin/bash`


# Samba
- Create samba/.samba_pw with password
