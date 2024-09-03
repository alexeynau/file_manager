# File Manager

Простой файловый менеджер на связке Flutter - Rust

## Getting Started

### Dependencies

* [Rust 1.79.0](https://www.rust-lang.org/)
* [Flutter 3.22.2](https://docs.flutter.dev/get-started/install)
* [Flutter Rust Bridge 2.3.0](https://cjycode.com/flutter_rust_bridge/)
```
cargo install flutter_rust_bridge_codege
```

### Build
Сгенерировать FFI:
```sh
flutter_rust_bridge_codegen generate
```
Запустить
```sh
flutter run
```

## API
Всё сгенерированное API находится в `lib/src/rust`

```dart
String greet({required String name});

void createFile({required String path});

void createFileWithContent({required String path, required String content});

String readFile({required String path});

void updateFile({required String path, required String content});

void deleteFile({required String path});
```