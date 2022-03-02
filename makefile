
backend-bin: backend/**/*
	cd backend \
	cargo build --release
backend: backend/**/* Dockerfile
docker build -t it1-twitter-backend:latest -f Dockerfile --rm .
backend-run: set_env.sh backend/**/*
	sh set_env.sh; \
	cd backend; \
	cargo run

backend-push: backend
	heroku container:push web --app it1-twitter-backend
backend-release: backend-push backend 
	heroku container:release web --app it1-twitter-backend

frontend-run: frontend/**/*
	cd frontend; \
	pnpm dev

