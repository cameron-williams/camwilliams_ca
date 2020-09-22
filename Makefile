run:
	(cd frontend && rm -rf ./dist && npx webpack && cp ./src/static/* ./dist/static) && (cd rust_backend && cargo run)

run_python:
	(cd python_backend && python manage.py runserver)

install:
	(cd frontend && rm -rf ./dist && npx webpack && cp ./src/static/* ./dist/static) && (cd rust_backend && cargo install --path .)