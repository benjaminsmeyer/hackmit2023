build:
    rm -rf build/* && mkdir -p build
    (cd ./game; cargo build --release; mv -f target/release/game ../build/)
    (cd ./gameio; raco pkg install --auto --no-docs; raco exe main.rkt;  mv -f ./main ../build/gameio)
    cp -r ./gameio/templates/ ./build/
