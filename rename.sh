# for file in **/jpm.toml; do
#     mv "$file" "${file%.txt}_1.txt"
# done

find . -depth -name "jpm.toml" -exec sh -c 'f="{}"; mv -- "$f" "${f%jpm.toml}espm.toml"' \;
