LIB_NAME=c_sharp_ffi
STATIC_LIB=$LIB_NAME.lib
DLL=$LIB_NAME.dll

cargo build 

# Create directories + clean
copyFiles () {
    OUTPUT_DIR=$1
    
    [[ -d $OUTPUT_DIR ]] && rm -rd $OUTPUT_DIR
    [[ ! -d $OUTPUT_DIR ]] && mkdir $OUTPUT_DIR;

    cp target/debug/$DLL ./$OUTPUT_DIR/
    cp target/debug/$STATIC_LIB ./$OUTPUT_DIR/
    cp target/*.h ./$OUTPUT_DIR/
}


copyFiles ffi_output
copyFiles FfiTest/lib

cd FfiTest
dotnet run
