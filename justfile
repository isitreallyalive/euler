hash +text:
  @printf "{{text}}" | openssl sha256 | cut -d' ' -f2