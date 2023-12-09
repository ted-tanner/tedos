PROJECT_DIR=$(dirname $BASH_SOURCE)
PROJECT_FILES=$(echo $PROJECT_DIR/build.sh $(find $PROJECT_DIR -name "*.toml") $(find $PROJECT_DIR/src -name "*.rs") $(find $PROJECT_DIR/src -name "*.c") $(find $PROJECT_DIR/src -name "*.h") $(find $PROJECT_DIR/src -name "*.S") $(find $PROJECT_DIR/src -name "*.ld") $PROJECT_DIR/Readme.md)

emacs $PROJECT_FILES -f shell
