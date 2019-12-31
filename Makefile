SYSTEMD := true
FEATURES := system-d
PREFIX := /usr/local
BIN := sbin
SYSTEMD_DIR := /etc/systemd/systemd



.PHONY: clean install uninstall

./target/release/wpa_statusd:
	cargo build --release --features $(FEATURES)

clean:
	cargo clean

install:
	@cp -v ./target/release/wpa_statusd $(PREFIX)/$(BIN)
	@if [ "$(SYSTEMD)" = "true" ]; then\
		cp -v ./systemd/wpa_statusd.service $(SYSTEMD_DIR);\
		cp -v ./systemd/wpa_statusd_reset_permissions.sh $(PREFIX)/$(BIN)/wpa_statusd_reset_permissions;\
	fi

uninstall:
	@rm -vf $(PREFIX)/$(BIN)/wpa_statusd
	@rm -vf $(SYSTEMD_DIR)/wpa_statusd.service
	@rm -vf $(PREFIX)/$(BIN)/wpa_statusd_reset_permissions

