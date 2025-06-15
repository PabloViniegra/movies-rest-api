# Movies API REST (Rust + Actix-web + SeaORM + SQLite)

API RESTful para la gestión de películas, directores, actores y géneros.
Desarrollada en Rust usando Actix-web y SeaORM, con persistencia en SQLite y migraciones automatizadas.

## Requisitos previos

- Rust: https://www.rust-lang.org/tools/install
- cargo: https://doc.rust-lang.org/cargo/
- sea-orm-cli (CLI de migraciones de SeaORM): https://www.sea-ql.org/SeaORM/docs/install-cli/
  Instálalo con:
      cargo install sea-orm-cli
- SQLite (opcional, solo si quieres inspeccionar la base de datos manualmente): https://www.sqlite.org/download.html

---

## Instalación de la CLI de migraciones (sea-orm-cli)

Para poder ejecutar las migraciones, necesitas instalar la CLI de SeaORM si no la tienes ya instalada:

    cargo install sea-orm-cli

Puedes comprobar si ya la tienes instalada con:

    sea-orm-cli --version

---

## Pasos para levantar el proyecto

1. Clona el repositorio:

       git clone <URL-DEL-REPO>
       cd movies-api-rest

2. Instala la CLI de migraciones de SeaORM (solo la primera vez):

       cargo install sea-orm-cli

3. Copia el archivo de ejemplo de entorno y edítalo si necesitas:

       cp .env.example .env

   El contenido mínimo de `.env`:

       DATABASE_URL=sqlite://movies.db

4. Instala dependencias y compila el proyecto:

       cargo build

5. Ejecuta las migraciones para crear la base de datos y las tablas:

       sea-orm-cli migrate up

6. Arranca el servidor de la API:

       cargo run

   El servidor quedará escuchando en http://localhost:8080

7. Probar el endpoint de películas:

       curl http://localhost:8080/movies

---

## Documentación interactiva (Swagger / OpenAPI)

Este proyecto puede exponer una interfaz Swagger para probar la API desde el navegador.

**Cómo habilitar Swagger:**

1. Asegúrate de tener en tu `Cargo.toml` las dependencias:

       utoipa = { version = "4", features = ["actix_extras"] }
       utoipa-swagger-ui = { version = "5", features = ["actix-web"] }


2. Levanta el servidor y accede a:

       http://localhost:8080/swagger-ui/

---

## Notas importantes

- No subas el archivo `.env` ni la base de datos (`movies.db`) al repositorio.
- Las migraciones fuente (los archivos `.rs` dentro de `migration/src`) **sí deben subirse**.
  Solo ignora los binarios generados y los targets.
- Si modificas el modelo de datos, genera una nueva migración y ejecútala con `sea-orm-cli migrate generate ...` y luego `migrate up`.
- Si tienes problemas, puedes borrar `movies.db` y volver a migrar desde cero.

---

## Estructura del proyecto

movies-api-rest/
│
├─ src/
│   ├─ main.rs
│   ├─ models.rs
│   ├─ db.rs
│   └─ route_handler.rs
├─ migration/
│   ├─ src/
│   │   └─ <archivos_de_migración>.rs
│   └─ Cargo.toml
├─ .env.example
├─ .gitignore
├─ Cargo.toml
└─ README.md

---

¡Feliz hacking con Rust! 🚀
