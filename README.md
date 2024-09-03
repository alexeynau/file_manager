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

### Testing
Тестирование Rust библиотеки
```sh
cd rust
cargo test -- --test-threads 1
```

## API
Всё сгенерированное API находится в `lib/src/rust`
Функции также могут прокидывать исключения
```dart
String greet({required String name});

void createFile({required String path});

void createFileWithContent({required String path, required String content});

String readFile({required String path});

void updateFile({required String path, required String content});

void deleteFile({required String path});

File readFile({required String path});

void updateFile({required String path, required String content});

void deleteFile({required String path});

void createDir({required String path});

List<File> listAll({required String path});

void deleteDir({required String path});
```