readonly TARGET_ARCH=armv7-unknown-linux-gnueabihf

cargo build --release --target=${TARGET_ARCH}

systemctl stop input-switch
cp input-switch.service /etc/systemd/system/input-switch.service
cp ./target/${TARGET_ARCH}/release/input-switch /usr/bin/input-switch
systemctl start input-switch
systemctl enable input-switch