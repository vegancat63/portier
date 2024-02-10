Portier - хранилка аккаунтов

cp ./env-example ./env
правим подключение к базе данных в ./.env

Запускаем миграции в БД
diesel migration run

Запускаем бэкенд
cargo run
