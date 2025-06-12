# Namumark
A namumark renderer made with Rust
# Why this parser?
- 정규식 파서보다 빠름
# 원리
todo
# 한계
일반적인 상황에서는 속도를 보장하나, 특정 상황에서는 속도를 보장하지 못함.
todo: 일단 최대한 잘못한거 찾고 못찾으면 가는 걸로
# 1단계
macro
~~link~~
~~literal~~
shberg
    ~~contentisnamumark,~~
    contentisliteral,
heading
# 2단계
reference
# 3단계
{{{+}}}
{{{-}}}
{{{#color,color }}}
literal 뜯어가지고 만들거임
나머지 텍스트 효과
`표`(가장 어려움...?)