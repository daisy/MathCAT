---
layout: default
lang: ru
ref: developers
title: Руководство разработчика
---
# Руководство разработчика

В этом руководстве приведена техническая информация для разработчиков, работающих с кодовой базой MathCAT.

## Предварительные требования

Для разработки MathCAT необходимо установить Rust. Если вы ещё этого не сделали:

1. [Скачайте и установите Rust](https://www.rust-lang.org/tools/install).
2. Клонируйте репозиторий MathCAT.
3. Откройте каталог проекта в интегрированной среде разработки (IDE).

## Работа с Cargo

Cargo — система сборки и менеджер пакетов Rust. Ниже приведены основные команды.

### Сборка проекта

```bash
# Собрать проект в режиме отладки
cargo build

# Собрать проект в режиме выпуска (с оптимизацией)
cargo build --release
```

### Запуск проекта

```bash
# Запустить основной исполняемый файл
cargo run

# Запустить с указанными аргументами
cargo run -- <args>
```

### Управление зависимостями

Зависимости задаются в `Cargo.toml`. Cargo автоматически скачивает их и управляет ими.

```bash
# Обновить зависимости до последних совместимых версий
cargo update
```

## Тестирование

Тестирование необходимо для поддержания качества кода и предотвращения ошибок в существующей функциональности после внесения изменений.

### Запуск тестов

```bash
# Запустить все тесты
cargo test

# Запустить указанный тест
cargo test test_name
```

### Написание тестов

Тесты MathCAT проверяют, что выражения MathML преобразуются в ожидаемый текст для озвучивания. Пример:

```rust
#[test]
fn test_simple_fraction() {
    let expr = "<math>
                    <mfrac>
                        <mn>1</mn>
                        <mn>2</mn>
                    </mfrac>
                </math>";
    test("en", "SimpleSpeak", expr, "1 half");
}
```

### Покрытие тестами

Покрытие тестами помогает определить, какие части кода проверяются тестами, а для каких частей необходимо добавить проверки.

<details>
<summary>Использование grcov в macOS</summary>

В этом подходе для создания отчётов о покрытии тестами используются `llvm-cov` и `grcov`. [grcov](https://github.com/mozilla/grcov) должен работать и в других операционных системах, но может потребовать некоторых изменений путей LLVM и конфигурации.

**Однократная настройка:**

```bash
# Установить необходимые компоненты
rustup component add llvm-tools-preview
cargo install grcov
```

**Создание отчёта о покрытии:**

```bash
# Задать переменную среды для данных профилирования
export LLVM_PROFILE_FILE="target/coverage/%p-%m.profraw"

# Запустить тесты со сбором данных о покрытии
RUSTFLAGS="-Cinstrument-coverage" cargo test

# Пример: запустить один тест
# RUSTFLAGS="-Cinstrument-coverage" cargo test Languages::zh::tw::units::without_prefix_powers_of_2

# Создать HTML-отчёт
grcov . \
  --source-dir . \
  --binary-path ./target/debug/deps \
  -t html \
  --branch \
  --ignore-not-existing \
  --ignore "target/*" \
  -o target/coverage/html

# Открыть отчёт в браузере
open target/coverage/html/index.html
```

</details>

**Альтернатива: интеграция с IDE**

Во многих IDE для Rust, например RustRover или VS Code, есть встроенная поддержка анализа покрытия тестами.
