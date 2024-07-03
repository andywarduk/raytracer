#/bin/sh

package=$1
shift

if [ "x$package" == "x" ]
then
    echo "No package specified"
    exit 1
fi

cargo run --profile=release-lto --package $package -- $*
