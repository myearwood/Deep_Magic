cp config/deep_magic.service /etc/systemd/system/deep_magic.service

# enable the geth service
sudo systemctl daemon-reload
sudo systemctl enable deep_magic
systemctl restart deep_magic
