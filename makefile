
backend-bin: backend/**/*
	cd backend \
	cargo build --release
backend: backend/**/* ./backend/Dockerfile
	cd backend; \
	docker build -t it1-twitter-backend:latest --rm .
backend-run: set_env.sh backend/**/*
	sh set_env.sh; \
	cd backend; \
	cargo run

backend-push: backend
	cd backend; \
	heroku container:push web --app it1-twitter-backend
backend-release: backend-push backend 
	cd backend; \
	heroku container:release web --app it1-twitter-backend

frontend-run: frontend/**/*
	cd frontend; \
	pnpm dev
frontend: frontend/**/* ./frontend/Dockerfile
	cd frontend; \
	docker build -t it1-twitter-frontend:latest --rm .

frontend-push: frontend
	cd frontend; \
	heroku container:push web --app it1-twitter
frontend-release: frontend-push frontend 
	cd frontend; \
	heroku container:release web --app it1-twitter

release: frontend-release backend-release
	@echo "🎉 App is up"
