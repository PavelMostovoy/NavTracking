# App run with Dioxus

    cargo install dioxus-cli

Run dev version:

Run default desktop app
    
    dx serve  
 

Run Web version 

    dx serve --platform web



Build desktop version:

    dx bundle --platform  macos

Build web version:

     dx bundle --platform  web


Mobile (android example):
Install NDK and make env variable for it location

install toolchain

    rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android


Run Build command 

    dx bundle --platform  android


Temp instruction for mac :

    brew install bundletool
    bundletool build-apks --bundle=app-release.aab --output=app-release.apks
    bundletool install-apks --apks=app-release.apks