cargo build --release
sh bundle.sh
codesign -s "David Holtz" Grams.app
create-dmg --overwrite Grams.app