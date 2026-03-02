# Demo todo-list приложение на ratatui

# Задания на закрепление

## Вариант 1

Расширить данный todo-list, сделав обработку событий асинхронными (tokio).

Для этого нужно будет перейти на асинхронную обработку ивентов (crossterm::event::EventStream) и написать свой event loop.
В качестве примера можно рассмотреть https://ratatui.rs/tutorials/counter-async-app/async-event-stream/
Для примеров архитектуры приложения можно посмотреть, что предлагает сам ratatui https://ratatui.rs/concepts/application-patterns/

## Вариант 2

Мини-vim

Текстовый редактор, поддерживающий режим навигации и режим ввода (Normal mode/Insert mode) и базовый набор команд (:w :q).
В качестве примера можно рассмотреть:
- 3rd-party виджеты https://github.com/sayanarijit/tui-input и https://github.com/rhysd/tui-textarea
- пример от ratatui https://ratatui.rs/examples/apps/user_input/

## Вариант 3

Мини-top

Простой монитор ресурсов системы. 
Можно взять за основу какие-то данные о текущем состоянии системы, например
- /proc/stat
- /proc/meminfo

поллить их с определенной периодичностью и выводить состояние на экран.
Для отрисовки графиков ratatui предоставляет https://ratatui.rs/examples/widgets/canvas/
