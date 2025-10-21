import re
import sys

def is_doc_comment(line):
    return line.strip().startswith("///") or line.strip().startswith("//!")

def contains_url(line):
    return "http://" in line or "https://" in line

def clean_comments(code):
    lines = code.split('\n')
    cleaned_lines = []

    inside_block_comment = False

    for line in lines:
        stripped = line.strip()

        # 블록 주석 시작
        if not inside_block_comment and "/*" in stripped:
            if "*/" not in stripped:
                inside_block_comment = True
            continue  # skip this line

        # 블록 주석 내부
        if inside_block_comment:
            if "*/" in stripped:
                inside_block_comment = False
            continue  # skip this line

        # 문서화 주석은 유지
        if is_doc_comment(stripped):
            cleaned_lines.append(line)
            continue

        # 일반 주석 줄 (//)
        if stripped.startswith("//"):
            if contains_url(stripped):
                cleaned_lines.append(line)  # URL이 포함된 주석은 유지
            # 그 외는 제거 (skip)
            continue

        # 코드 줄
        # 코드 줄 안에 있는 인라인 주석 제거
        # let x = 5; // comment → let x = 5;
        line = re.sub(r'(?<!:)//(?!/).*$', '', line)  # https:// 제외
        cleaned_lines.append(line)

    return '\n'.join(cleaned_lines)

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("사용법: python remove_comments.py <파일명>")
        sys.exit(1)

    with open(sys.argv[1], "r", encoding="utf-8") as f:
        original_code = f.read()

    cleaned_code = clean_comments(original_code)

    with open(sys.argv[1], "w", encoding="utf-8") as f:
        f.write(cleaned_code)

    print("주석 제거 완료 (문서화 주석과 URL 포함 주석은 유지됨)!")
