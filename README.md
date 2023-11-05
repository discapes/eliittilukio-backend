# eliittilukio-backend

### Developing locally

- Set the following variables in .env: `DATABASE_URL,SMTP_USER,SMTP_PASS,JWT_KEY,ADMIN_EMAIL`
  - Note that sqlx validates queries based on DATABASE_URL being available
- Initialize the database with `sqlite3 database.db "$(cat src/schema.sql)"`
- `cargo run`
- `sudo caddy` if you plan on using the frontend
- Releasing a build:
	```bash
	DEST=861821644425.dkr.ecr.eu-north-1.amazonaws.com
	aws ecr get-login-password --region eu-north-1 | docker login --username AWS --password-stdin $DEST
	docker build . -t $DEST/eliittilukio-backend; docker push $DEST/eliittilukio-backend
	```

### Deploying the site

```bash
# make sure .env is filled
aws ecr get-login-password --region eu-north-1 | docker login --username AWS --password-stdin 861821644425.dkr.ecr.eu-north-1.amazonaws.com
docker-compose pull
docker-compose up --abort-on-container-exit
```