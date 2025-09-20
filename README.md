# fileDRust - 다목적 백업 도구

Rust로 작성된 파일 백업 및 실시간 동기화 도구입니다.

## 기능

- **로컬 백업/복원**: 파일 해시 기반 무결성 검증
- **실시간 원격 동기화**: SSH를 통한 실시간 파일 동기화
- **다중 백업**: 여러 타겟 디렉토리에 동시 백업
- **임시 파일 필터링**: 에디터 임시 파일 자동 제외

## 예제
> todo: ~~cargo run~~ -> [some_command]
### 로컬 백업
```bash
# 기본 백업 (설정 파일 사용)
cargo run backup

# 특정 소스와 타겟 지정
cargo run backup source <Source_Directory> --targets <Backup_Directory1> <Backup_Directory2>
```

### 복원
```bash
cargo run restore --backup_path ./backup1 --restore_to ./recovered
```

### 실시간 원격 동기화
```bash
cargo run -- sync
```