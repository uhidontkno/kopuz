serve:
	dx serve

tailwind:
	npx @tailwindcss/cli -i ./tailwind.css -o ./rusic/assets/tailwind.css --content './rusic/**/*.rs,./components/**/*.rs,./pages/**/*.rs,./hooks/**/*.rs,./player/**/*.rs,./reader/**/*.rs'

build: tailwind
	dx build --package rusic --release
	@echo ""
	@echo "Build complete! Run 'make install' to install."

run-release: build
	cd target/dx/rusic/release/linux/app && ./rusic

install: build
	mkdir -p ~/.local/share/rusic
	mkdir -p ~/.local/bin
	mkdir -p ~/.local/share/applications
	mkdir -p ~/.local/share/icons/hicolor/scalable/apps
	cp target/dx/rusic/release/linux/app/rusic ~/.local/share/rusic/
	cp -r target/dx/rusic/release/linux/app/assets ~/.local/share/rusic/
	ln -sf ~/.local/share/rusic/rusic ~/.local/bin/rusic
	cp rusic/assets/logo.png ~/.local/share/icons/hicolor/scalable/apps/com.temidaradev.rusic.png
	@echo "[Desktop Entry]" > ~/.local/share/applications/com.temidaradev.rusic.desktop
	@echo "Name=Rusic" >> ~/.local/share/applications/com.temidaradev.rusic.desktop
	@echo "Comment=A modern music player" >> ~/.local/share/applications/com.temidaradev.rusic.desktop
	@echo "Exec=sh -c 'cd ~/.local/share/rusic && ./rusic'" >> ~/.local/share/applications/com.temidaradev.rusic.desktop
	@echo "Icon=com.temidaradev.rusic" >> ~/.local/share/applications/com.temidaradev.rusic.desktop
	@echo "Terminal=false" >> ~/.local/share/applications/com.temidaradev.rusic.desktop
	@echo "Type=Application" >> ~/.local/share/applications/com.temidaradev.rusic.desktop
	@echo "Categories=Audio;Music;Player;" >> ~/.local/share/applications/com.temidaradev.rusic.desktop
	update-desktop-database ~/.local/share/applications/ 2>/dev/null || true
	@echo "Installed to ~/.local/bin/rusic"

uninstall:
	rm -f ~/.local/bin/rusic
	rm -rf ~/.local/share/rusic
	rm -f ~/.local/share/applications/com.temidaradev.rusic.desktop
	rm -f ~/.local/share/icons/hicolor/scalable/apps/com.temidaradev.rusic.png
	update-desktop-database ~/.local/share/applications/ 2>/dev/null || true
	@echo "Uninstalled rusic"

clean:
	cargo clean
	rm -rf target/dx dist
