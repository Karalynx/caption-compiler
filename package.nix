{
  lib,
  rustPlatform
}:

let
  manifest = (lib.importTOML ./Cargo.toml).package;
in
  rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;

    __structuredAttrs = true;

    src = lib.cleanSource ./.;

    cargoLock.lockFile = ./Cargo.lock;

    meta = {
      description = "Compiles and describes Valve's closed caption files";
      homepage = "https://github.com/Karalynx/caption-compiler";
      license = with lib.licenses; [ gpl3 ];
      platforms = with lib.platforms; linux ++ darwin ++ windows;
    };
  }