# fbar

## Работа с архивами книг

Анализ содержимого архивов с книгами в fb2 формате, например [fb2-632000-634999.zip](http://trec.to/viewtopic.php?t=34961);
Комманда: archive 

| Command | Description |
| ------- | ----------- |
| `$ fbar ls <archive.zip>`          | Вывести список файлов в архиве |
| `$ fbar check <archive.zip>`       | Проверить возможность распарсить книги в архиве |
| `$ fbar show xml <archive.zip> [book]`    | Извлечь описание книг(и) в XML формате [FB2](http://fictionbook.org/) |
| `$ fbar show fb2 <archive.zip> [book]`    | Извлечь описание в виде дампа внутренней структуры []FictionBook](https://github.com/seb-odessa/fb2parser) |
| `$ fbar show inf <archive.zip> [book]`    | Извлечь однострочное описание книги |
| `$ fbar show zip <archive.zip> [book]`    | Извлечь описание физического расположения файла в архиве |

