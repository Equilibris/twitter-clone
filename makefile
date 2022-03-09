
backend-bin: backend/**/*
	cd backend \
	cargo build --release
backend: backend/**/* ./backend/Dockerfile
	cd backend; \
	docker build -t it1-twitter-backend:latest --rm .
backend-run: backend/**/*
	cd backend; \
	cargo run
backend-load: backend/**/*
	cd backend; \
	cargo run --profile load

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

# Devops

db-prepare:
	cd db; \
	make build

db: ./db/Dockerfile
	cd db; \
	docker build -t it1-twitter-db:latest --rm .

db-volume:
	-docker volume create it1-twitter-vol

db-run: db db-volume
	docker container rm -f it1-twitter-db 
	docker run -v it1-twitter-vol:/data -it -p 6379:6379 --rm --name it1-twitter-db it1-twitter-db:latest 

load-%:
	cd ./test/backend/; \
	pnpm build; \
	k6 run dist/api-$*.js

