VERSION=v0.2.6
NAME=tgr
EXEC=tgr
INSTALL_DIR=$(HOME)/.local/bin/
ZSH_COMP_DIR=$(HOME)/.zsh_local_comp/ #/usr/share/zsh/site-functions/
MAN_DIR=/usr/share/man/man1/

default: build_release

clean:
	@echo "Removing build artifacts..."
	@cargo clean --quiet
	@echo "Done!"

build_release:
	@echo "Building $(NAME) version $(VERSION)..."
	@cargo build --release
	@echo "Finished building $(NAME) version $(VERSION)"

install: build_release
	@echo "Installing $(NAME) version $(VERSION)..."
	@cp target/release/$(EXEC) $(INSTALL_DIR)
	@sudo cp etc/shell_comp/_tgr $(ZSH_COMP_DIR)
	# @sudo cp etc/man/{tgr.1,tgr-count.1,tgr-locate.1,tgr-refile.1,tgr-search.1,tgr-sed.1,tgr-tags.1} $(MAN_DIR)
	@echo "$(NAME) version $(VERSION) successfully installed"

uninstall:
	@echo "Uninstalling $(NAME) version $(VERSION)..."
	@rm $(INSTALL_DIR)/$(EXEC)
	@sudo rm $(ZSH_COMP_DIR)/_tgr
	# @sudo rm $(MAN_DIR)/{tgr.1,tgr-count.1,tgr-locate.1,tgr-refile.1,tgr-search.1,tgr-sed.1,tgr-tags.1}
	@echo "$(NAME) version $(VERSION) uninstalled"
