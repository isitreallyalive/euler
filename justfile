@h +text:
  printf "{{text}}" | openssl sha256 | cut -d' ' -f2
  
@b:
  cd book && mdbook serve

@r number:
  bacon problem -- {{number}}

@rm number:
  rm problems/src/p{{number}}.rs
  rm book/src/problems/{{number}}.md