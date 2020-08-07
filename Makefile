build:
	(cd frontend && rm -rf ./dist && npx webpack) && (cd rust_backend && cargo run)

run_python:
	(cd python_backend && python manage.py runserver)