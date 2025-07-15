# Namumark
A namumark parser made with Rust<br />
이 라이브러리는 HTML을 렌더링하지 않습니다.<br />
추상화 구문 트리를 반환합니다.<br />
# Why this parser?
- 정규식 파서보다 빠름
# 원리
todo
# 한계
일반적인 상황에서는 속도를 보장하나, 특정 상황에서는 속도를 보장하지 못함.


# 1단계
macro<br />
~~link~~<br />
~~literal~~<br />
shberg<br />
    ~~contentisnamumark,~~<br />
    contentisliteral,<br />
heading
# 2단계
reference
# 3단계
{{{+}}}<br />
{{{-}}}<br />
{{{#color,color }}}<br />
나머지 텍스트 효과<br />
`표`(가장 어려움...?)
# 확장
문법 확장 지원 예정입니다.<br />
- 메크로
- #!