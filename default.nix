{
  lib,
  rustPlatform,
  fetchFromGitHub,
  nix-update-script,
  installShellFiles,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "tgr";
  version = "0.2.7";
  __structuredAttrs = true;

  src = fetchFromGitHub {
    owner = "L-Colombo";
    repo = "tgr";
    tag = "v${finalAttrs.version}";
    hash = "sha256-4oJDtfOYjSUxj2dAUXdGzhMYckhkxpsJNWZS0AlrUz4=";
  };

  nativeBuildInputs = [ installShellFiles ];

  postInstall = ''
    installManPage etc/man/tgr.1
    installManPage etc/man/tgr-count.1
    installManPage etc/man/tgr-locate.1
    installManPage etc/man/tgr-refile.1
    installManPage etc/man/tgr-search.1
    installManPage etc/man/tgr-sed.1
    installManPage etc/man/tgr-tags.1

    installShellCompletion etc/shell_comp/{_tgr,tgr.bash}
  '';

  cargoHash = "sha256-1vqYCV31rPfqAkjy8yXMm6hevuayJOsc61D98TOL6Sk=";

  passthru.updateScript = nix-update-script { };

  meta = {
    description = "";
    homepage = "https://github.com/L-Colombo/tgr";
    license = lib.licenses.mit;
    mainProgram = "tgr";
  };
})
