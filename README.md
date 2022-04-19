# Gourmand Web Viewer

## About ##
Gourmand Web Viewer is a fork of [VersatiList](https://github.com/newca12/VersatiList).

Gourmand Web Viewer is compatible with [Gourmand Recipe Manager](https://github.com/GourmandRecipeManager/gourmand).  

Gourmand Web Viewer allow recipe search by ingredients you have at home using your browser on all your devices.

gourmand-web-wiewer is an EDLA project.

The purpose of [edla.org](http://www.edla.org) is to promote the state of the art in various domains.

## Web version ##
You can try the [demo online](http://edla.org/GourmandWebViewer)
or built it yourself (replace gourmand-web-viewer-cli/src/data/recipes.xml with your own Gourmand export) :  
to be done once :
```
rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli --version 0.2.69
```
to be done each time recipes.xml is updated
```
cargo run --bin gourmand-web-viewer-cli -- --cli --command generate
cargo build --release --package gourmand-web-viewer --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/gourmand-web-viewer.wasm --out-dir GourmandWebViewer --web
```

## A GUI is also available ##
To launch the GUI 
```
cargo run --bin gourmand-web-viewer
``` 

### License ###
© 2021 Olivier ROLAND. Distributed under the GPLv3 License.
