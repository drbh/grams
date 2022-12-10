DST="."
APPDIR="Grams.app"
rm -rf "$DST/$APPDIR"
mkdir "$DST/$APPDIR/"
mkdir "$DST/$APPDIR/Contents/"
mkdir "$DST/$APPDIR/Contents/Resources/"
mkdir "$DST/$APPDIR/Contents/MacOS/"
cp -a target/release/grams "$DST/$APPDIR/Contents/MacOS/"
cp -a assets/grams.icns "$DST/$APPDIR/Contents/Resources/"
/usr/bin/strip -u -r "$DST/$APPDIR/Contents/MacOS/grams"
cat > "$DST/$APPDIR/Contents/Info.plist" << EOF
{
   CFBundleName = grams;
   CFBundleDisplayName = Grams;
   CFBundleIdentifier = "com.drbh.grams";
   CFBundleExecutable = grams;
   CFBundleIconFile = "grams.icns";
   CFBundleVersion = "0.0.1";
   CFBundleShortVersionString = "0.0.1";
   CFBundleInfoDictionaryVersion = "6.0";
   CFBundlePackageType = APPL;
   CFBundleSignature = xxxx;
   LSMinimumSystemVersion = "10.10.0";
}
EOF
echo "🌈 Built Grams.app"