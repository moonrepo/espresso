find . -depth -name "espm.toml" -exec sh -c 'f="{}"; mv -- "$f" "${f%espm.toml}esp.toml"' \;
