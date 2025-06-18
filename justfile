hash +text:
  @printf "{{text}}" | openssl sha256 | cut -d' ' -f2

book +args:
  @cd book && mdbook {{args}}

run number:
  @bacon problem -- {{number}}