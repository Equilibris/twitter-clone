
backend-bin: backend/**/*
	cd backend \
	cargo build --release
backend: backend/**/* backend.dockerfile
	docker build -t it1-twitter-backend:latest -f backend.dockerfile --rm .
backend-run: set_env.sh backend/**/*
	sh set_env.sh; \
	cd backend; \
	cargo run

backend-push: backend
	heroku container:push -a it1-twitter-backend backend
backend-release: backend-push backend 
	heroku container:push -a it1-twitter-backend backend

frontend-run: frontend/**/*
	cd frontend
	pnpm dev

