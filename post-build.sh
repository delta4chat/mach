#!/bin/bash

set -x

cd target/release/

for i in libmach*.d
do
	continue
	mv $i _$i && trap "mv _$i $i" EXIT
done

if type 7zz
then
	if 7zz a libmach.7z libmach*
	then
		mv libmach.7z libmach.archive
		exit 0
	fi
fi

if type 7z
then
	echo "Notice: official 7-zip is not found."
	echo "Warning: fallback to p7zip (not actively maintained)"
	if 7z a libmach.7z libmach*
	then
		mv libmach.7z libmach.archive
		exit 0
	fi
fi

if type xz && type tar
then
	echo "Notice: no any 7-zip archiver found."
	echo "Info: fallback to .tar.xz"

	if tar vvcf libmach.tar libmach* && xz -v -9 libmach.tar -T8
	then
		mv libmach.tar.xz libmach.archive
		exit 0
	fi
fi

if type zip
then
	echo "Notice: no XZ compressor found."
	echo "Info: fallback to .zip"
	if zip libmach.zip libmach*
	then
		mv libmach.zip libmach.archive
		exit 0
	fi
fi

echo "Error: no any usable archiver/compressor found."
exit 1

